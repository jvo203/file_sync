use chrono::Local;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch() -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = r#try!(Watcher::new(tx, Duration::from_secs(10)));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    r#try!(watcher.watch("/home/chris", RecursiveMode::Recursive));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => {
                let now = Local::now();
                println!("[{}] {:?}", now, event);
            }
            Err(e) => {
                let now = Local::now();
                println!("[{}] watch error: {:?}", now, e);
            }
        }
    }
}

fn main() {
    println!("file syncing application started");

    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
