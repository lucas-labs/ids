//! ### Watcher
//!
//! The `watcher` module is responsible for watching the current directory, looking for changes in
//! the files and directories. When a change is detected, the watcher will broadcast a message.
//!
//! The message will trigger several actions, such as:
//! - Rebuilding the javascript file that contains the current changes in the repository
//! - Sending a message to the clients connected to the websocket in order to refresh the page and
//!   get the latest changes

use {
    crossbeam_channel::{unbounded, Receiver, Sender},
    std::{path::PathBuf, thread::spawn, time::Duration},
};

use {
    crate::Server,
    eyre::Result,
    lool::cli::stylize::Stylize,
    notify::{Config, RecommendedWatcher},
    notify_debouncer_mini::{new_debouncer_opt, Config as DebouncerConfig},
};

impl Server {
    pub(crate) fn start_watcher(&self) -> Receiver<()> {
        let (snd, recv) = unbounded();

        let path = self.repo_path.clone();
        spawn(move || watch(path, snd));

        recv
    }
}

// watch the path for changes and notify the sender when a change is detected
fn watch(path: PathBuf, sender: Sender<()>) -> Result<()> {
    println!("» Watching for changes in {}", path.to_string_lossy().replace('\\', "/").blue());

    let (tx, rx) = unbounded();
    let watcher_cfg = Config::default().with_compare_contents(true);

    let debouncer_cfg = DebouncerConfig::default()
        .with_notify_config(watcher_cfg)
        .with_timeout(Duration::from_millis(100));

    let mut debouncer = new_debouncer_opt::<_, RecommendedWatcher>(debouncer_cfg, tx).unwrap();

    debouncer.watcher().watch(path.as_path(), notify::RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                // filter svg files only
                if let Ok(events) = event {
                    // if any of the events is on an svg file, notify the sender
                    if events.iter().any(|e| e.path.extension().map_or(false, |ext| ext == "svg")) {
                        sender.send(())?;
                    }
                }
            }
            Err(e) => {
                eprintln!("» Watcher error: {:?}", e);
            }
        }
    }
}
