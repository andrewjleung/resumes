use crate::prelude::*;
use crate::utils::path::normalize_path;
use anyhow::{Context, Error, Result};
use notify::Event;
use notify::EventKind;
use notify::FsEventWatcher;
use notify::Watcher;
use notify::event::DataChange;
use notify::event::ModifyKind;
use std::collections::HashSet;
use std::sync::mpsc;
use std::time::Duration;

use crate::config::Config;
use crate::config::reload::ReloadableConfig;
use crate::view::View;

#[derive(PartialEq)]
struct Target {
    dir: PathBuf,
    filename: String,
}

impl Target {
    fn from_path(path: &Path) -> Result<Self> {
        let path = path
            .canonicalize_utf8()
            .context("failed to canonicalize watch target path")?;

        if path.is_file()
            && let Some(dir) = path.parent()
            && let Some(filename) = path.file_name()
        {
            Ok(Self {
                dir: dir.into(),
                filename: filename.to_owned(),
            })
        } else {
            Err(anyhow!("failed to recognize watch target {}", path))
        }
    }
}

struct TargetWatcher {
    watcher: FsEventWatcher,
    targets: Vec<Target>,
}

impl TargetWatcher {
    fn new(watcher: FsEventWatcher) -> Self {
        Self {
            watcher,
            targets: Vec::new(),
        }
    }

    fn watched_paths(&self) -> Vec<PathBuf> {
        HashSet::<PathBuf>::from_iter(self.targets.iter().map(|t| t.dir.clone()))
            .iter()
            .cloned() // TODO: two clones...
            .collect::<Vec<_>>()
    }

    fn clear(&mut self) -> Result<()> {
        for path in self.watched_paths() {
            self.watcher
                .unwatch(path.as_std_path())
                .context("failed to unwatch target during reload")?
        }

        Ok(())
    }

    fn watch(&mut self) -> Result<()> {
        for path in self.watched_paths() {
            self.watcher
                .watch(path.as_std_path(), notify::RecursiveMode::NonRecursive)
                .context("failed to watch target during reload")?
        }

        Ok(())
    }

    fn set_targets(&mut self, config: &Config) -> Result<()> {
        self.clear()?;
        self.targets = config
            .watched_file_paths()
            .into_iter()
            .filter_map(|path| Target::from_path(&path).ok())
            .collect::<Vec<_>>();
        self.watch()
    }

    fn is_watching(&self, path: &Path) -> bool {
        Target::from_path(path).is_ok_and(|target| self.targets.contains(&target))
    }
}

pub fn watch<F>(config: ReloadableConfig, f: F) -> Result<()>
where
    F: Fn(&Config) -> Result<()>,
{
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)
        .map_err(Error::new)
        .context("failed to create file watcher")?;

    watcher
        .configure(notify::Config::default().with_poll_interval(Duration::from_secs(5)))
        .context("failed to configure polling for file watcher")?;

    let mut config = config;
    let mut target_watcher = TargetWatcher::new(watcher);

    target_watcher.set_targets(&config.current)?;
    View::Watching.print(&config.current, true)?;

    for res in rx {
        match res {
            Ok(Event {
                kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
                paths,
                ..
            }) => {
                if !paths
                    .iter()
                    .filter_map(|path| normalize_path(path).ok())
                    .any(|path| target_watcher.is_watching(&path))
                {
                    continue;
                }

                config = config.reload();
                target_watcher.set_targets(&config.current)?;

                if let Err(e) = f(&config.current) {
                    View::Error(&e).print(&config.current, true)?;
                }

                View::Watching.print(&config.current, true)?;
            }
            Err(e) => Err(Error::new(e).context("error while watching changes"))?,
            _ => (),
        }
    }

    Ok(())
}
