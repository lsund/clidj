use crate::controller;
use crate::library;
use crate::library::Library;
use crate::mediaplayer;
use crate::prompt_history;
use controller::Message;
use prompt_history::PromptHistory;
use std::sync::mpsc;

pub struct Application {
    pub library: Library,
    pub prompt_history: PromptHistory,
    pub tx: mpsc::Sender<Message>,
    pub rx: mpsc::Receiver<String>,
}

pub fn make(
    library_dir: &str,
    main_tx: mpsc::Sender<Message>,
    main_rx: mpsc::Receiver<String>,
    mplayer_tx: mpsc::Sender<String>,
    mplayer_rx: mpsc::Receiver<Message>,
) -> Application {
    mediaplayer::init(mplayer_tx, mplayer_rx);
    return Application {
        library: library::make(library_dir),
        prompt_history: prompt_history::make(),
        tx: main_tx,
        rx: main_rx,
    };
}
