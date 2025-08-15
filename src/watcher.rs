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

    if let Err(e) = f() {
        View::Error(&e).print(world)?;
    }

    for res in rx {
        match res {
            Ok(event) => {
                if let Err(e) = handle(event, &f) {
                    View::Error(&e).print(world)?;
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
        return f();
    }

    Ok(())
}
