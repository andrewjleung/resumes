use anyhow::{Error, Result};
use camino::Utf8Path as Path;
use camino::Utf8PathBuf as PathBuf;
use notify::Event;
use notify::EventKind;
use notify::Watcher;
use notify::event::DataChange;
use notify::event::ModifyKind;
use notify::event::RenameMode;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

struct ResumeWatcher {
    resume_path: PathBuf,
    sender: Sender<notify::Result<Event>>,
    render: fn() -> Result<()>,
}

impl ResumeWatcher {
    /// Dual process modifications to content to allow simultaneous renames to always win.
    fn dual_process_event(&self, mut event: Event) -> Result<()> {
        if let Some(info) = event.attrs.info()
            && info == "seen"
        {
            (self.render)()?;
            println!("rerendered!");
        } else {
            event.attrs.set_info("seen");
            self.sender.send(Ok(event))?;
        }

        Ok(())
    }

    fn handle_event(&self, event: Event) -> Result<()> {
        match event {
            Event {
                kind: EventKind::Modify(ModifyKind::Name(RenameMode::To)),
                ..
            } => Err(Error::msg("file rename detected, stopping"))?,
            Event {
                kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
                ..
            } => self.dual_process_event(event),
            _ => Ok(()),
        }
    }
}

pub fn watch(resume_path: &Path, render: fn() -> Result<()>) -> Result<()> {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx.clone())?;
    let resume_watcher = ResumeWatcher {
        resume_path: resume_path.to_owned(),
        sender: tx,
        render,
    };

    watcher.watch(resume_path.as_ref(), notify::RecursiveMode::NonRecursive)?;

    println!("watching for changes to {resume_path}");

    for res in rx {
        match res {
            Ok(event) => resume_watcher.handle_event(event)?,
            Err(e) => Err(Error::new(e).context(format!(
                "error while watching changes to resume data: {resume_path}",
            )))?,
        }
    }

    Ok(())
}
