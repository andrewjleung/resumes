use anyhow::{Context, Error, Result};
use notify::Event;
use notify::EventKind;
use notify::FsEventWatcher;
use notify::Watcher;
use notify::event::DataChange;
use notify::event::ModifyKind;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use crate::config::Config;
use crate::config::reload::ReloadableConfig;
use crate::view::View;

pub fn watch<F>(config: ReloadableConfig, f: F) -> Result<()>
where
    F: Fn(&Config) -> Result<()>,
{
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)
        .map_err(Error::new)
        .context("failed to create file watcher")?;

    watch_with_setup(config, &mut watcher, rx, f)
}

fn watch_with_setup<F>(
    config: ReloadableConfig,
    watcher: &mut FsEventWatcher,
    rx: Receiver<notify::Result<Event>>,
    f: F,
) -> Result<()>
where
    F: Fn(&Config) -> Result<()>,
{
    let mut config = config;
    watch_config_paths(&config.current, watcher)?;
    View::Watching.print(&config.current, true)?;

    for res in rx {
        match res {
            Ok(Event {
                kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
                ..
            }) => {
                config = reload(config, watcher)?;

                if let Err(e) = f(&config.current) {
                    View::Error(&e).print(&config.current, true)?;
                }
            }
            Err(e) => Err(Error::new(e).context("error while watching changes"))?,
            _ => (),
        }
    }

    Ok(())
}

fn reload(config: ReloadableConfig, watcher: &mut FsEventWatcher) -> Result<ReloadableConfig> {
    for path in config.current.watched_file_paths() {
        watcher.unwatch(path.as_ref())?;
    }

    let reloaded_config = config.clone().reload();
    watch_config_paths(&reloaded_config.current, watcher)?;
    Ok(reloaded_config)
}

fn watch_config_paths(config: &Config, watcher: &mut FsEventWatcher) -> Result<()> {
    for path in config.watched_file_paths() {
        watcher.watch(path.as_ref(), notify::RecursiveMode::NonRecursive)?;
    }

    Ok(())
}
