use anyhow::{Context, Error, Result};
use camino::Utf8Path as Path;
use colored::Colorize;
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
    let main_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.rs");
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)
        .map_err(Error::new)
        .context("failed to create file watcher")?;

    watcher.watch(file_path.as_ref(), notify::RecursiveMode::NonRecursive)?;
    watcher.watch(main_path.as_ref(), notify::RecursiveMode::NonRecursive)?;

    println!(
        "watching for changes to {} and {}...",
        file_path.as_str().cyan(),
        main_path
            .file_name()
            .map(|f| f.cyan())
            .unwrap_or(main_path.as_str().cyan())
    );

    for res in rx {
        match res {
            Ok(event) => {
                if let Err(e) = handle(event, &f) {
                    println!("error occurred during rendering: {e}")
                }
            }
            Err(e) => Err(Error::new(e)
                .context(format!("error while watching changes to file: {file_path}",)))?,
        }
    }

    Ok(())
}
