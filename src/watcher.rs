use logwatcher::{LogWatcher, LogWatcherAction, StartFrom};
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Notify;

pub type LineInfo = (u64, String);

/// Read a file, then send every new line to the other thread
pub struct TailReader {
    log_watcher: LogWatcher,
    tx: Sender<LineInfo>,
}

impl TailReader {
    pub fn new(file: PathBuf, position: u64, tx: Sender<LineInfo>) -> Result<Self, Box<dyn Error>> {
        info!(
            "Will start to read the file from the position `{}`",
            position
        );

        Ok(Self {
            log_watcher: LogWatcher::register(file, StartFrom::Offset(position))?,
            tx,
        })
    }

    pub fn work(mut self) -> Arc<Notify> {
        let panicked = Arc::new(Notify::new());
        let notifier = panicked.clone();

        std::thread::spawn(move || {
            let tx = self.tx;

            self.log_watcher.watch(&mut move |pos, len, line: String| {
                let state = pos + len as u64;

                if let Err(e) = tx.blocking_send((state, line)) {
                    panic!("Can't send to mpsc: {}", e); // this is a fatal error
                } else {
                    trace!("Line sent via mpsc!");
                }

                LogWatcherAction::None
            });

            notifier.notify_one();
        });

        panicked
    }
}
