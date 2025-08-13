use anyhow::{Context, Error, Result};
use notify::Event;
use notify::EventKind;
use notify::Watcher;
use notify::event::DataChange;
use notify::event::ModifyKind;
use std::sync::mpsc;

use crate::view::View;
use crate::world::World;

pub fn watch<F>(world: &World, f: F) -> Result<()>
where
    F: Fn() -> Result<()>,
{
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)
        .map_err(Error::new)
        .context("failed to create file watcher")?;

    for path in world.watched_file_paths() {
        watcher.watch(path.as_ref(), notify::RecursiveMode::NonRecursive)?;
    }

    View::Watching.print(world)?;

    for res in rx {
        match res {
            Ok(event) => {
                if let Err(e) = handle(event, &f) {
                    Err(e.context("failed to handle file watching event"))?
                }
            }
            Err(e) => Err(Error::new(e).context("error while watching changes"))?,
        }
    }

    Ok(())
}

fn handle<F>(event: Event, f: F) -> Result<()>
where
    F: Fn() -> Result<()>,
{
    if let Event {
        kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
        ..
    } = event
    {
        f()?;
    }

    Ok(())
}
