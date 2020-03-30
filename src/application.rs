use crate::controller;
use crate::library;
use crate::library::Library;
use crate::mediaplayer;
use crate::prompt_history;
use controller::Controller;
use mediaplayer::MediaCtrl;
use prompt_history::PromptHistory;
use std::sync::mpsc;

pub struct Application {
    pub library: Library,
    pub prompt_history: PromptHistory,
    pub tx: mpsc::Sender<MediaCtrl>,
    pub rx: mpsc::Receiver<String>,
    pub controller: Controller,
}

pub fn make(
    library_dir: &str,
    app_to_mp: (mpsc::Sender<MediaCtrl>, mpsc::Receiver<MediaCtrl>),
    mp_to_app: (mpsc::Sender<String>, mpsc::Receiver<String>),
) -> Application {
    let (app_tx, mplayer_rx) = app_to_mp;
    let (mplayer_tx, app_rx) = mp_to_app;
    mediaplayer::init(mplayer_tx, mplayer_rx);
    return Application {
        library: library::make(library_dir),
        prompt_history: prompt_history::make(),
        controller: controller::make(),
        tx: app_tx,
        rx: app_rx,
    };
}
impl Application {
    pub fn log(&mut self, msg: String) {
        self.prompt_history.update(msg)
    }

    pub fn get_song(&self, x: &u32) -> Option<&String> {
        return self.library.content.get(&x);
    }
}
