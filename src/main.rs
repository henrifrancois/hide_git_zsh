extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs::{metadata, read_dir};
use std::process::Command;
use std::env::set_current_dir;

fn watch() -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = (Watcher::new(tx, Duration::from_secs(2)))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    (watcher.watch("/mnt/c/Users/User/Repositories/open_source", RecursiveMode::NonRecursive))?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Create(ref pathbuf) => {
                        if metadata(pathbuf.as_path()).unwrap().is_dir() {
                            if set_current_dir(pathbuf.as_path()).is_ok() {
                                // a check in case we have a non git repository should be done as well
                                println!("Executing git command: git config oh-my-zsh.hide-info 1"); // log the execution of the git command
                                Command::new("git")
                                    .args(["config", "oh-my-zsh.hide-info", "1"])
                                    .status()
                                    .expect("failed to execute git command");
                            }
                        }
                    },
                    _ => ()
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}