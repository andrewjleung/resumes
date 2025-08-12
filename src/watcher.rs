use anyhow::{Context, Error, Result};
use camino::Utf8Path as Path;
use notify::Event;
use notify::EventKind;
use notify::Watcher;
use notify::event::DataChange;
use notify::event::ModifyKind;
use std::sync::mpsc;

fn handle<F>(event: Event, f: F) -> Result<()>
where
    F: Fn() -> Result<()>,
{
    match event {
        Event {
            kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
            ..
        } => {
            println!("content updated!");
            f()
        }
        _ => Ok(()),
    }
}

pub fn watch<F>(file_path: &Path, f: F) -> Result<()>
where
    F: Fn() -> Result<()>,
{
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)
        .map_err(Error::new)
        .context("failed to create file watcher")?;

    watcher.watch(file_path.as_ref(), notify::RecursiveMode::NonRecursive)?;

    println!("watching for changes to {file_path}");

    for res in rx {
        match res {
            Ok(event) => handle(event, &f)?,
            Err(e) => Err(Error::new(e)
                .context(format!("error while watching changes to file: {file_path}",)))?,
        }
    }

    Ok(())
}
