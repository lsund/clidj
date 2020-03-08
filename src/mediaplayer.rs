use crate::controller;
use controller::MediaCtrl;
use std::sync::mpsc;
use std::thread;
use vlc::{Instance, Media, MediaPlayer, Meta};

static RATE_DELTA: f32 = 0.002;

fn get_meta(mpd: &MediaPlayer) -> Option<String> {
    return mpd.get_media().and_then(|m| m.get_meta(Meta::Genre));
}

fn speed_up(mdp: &MediaPlayer) -> f32 {
    let rate = mdp.get_rate();
    let rate_ = rate + RATE_DELTA;
    let _ = mdp.set_rate(rate_);
    return rate_;
}

fn speed_down(mdp: &MediaPlayer) -> f32 {
    let rate = mdp.get_rate();
    let rate_ = rate - RATE_DELTA;
    let _ = mdp.set_rate(rate_);
    return rate_;
}

fn play_or_pause(mdp: &MediaPlayer) {
    if mdp.is_playing() {
        mdp.pause();
    } else {
        mdp.play().unwrap();
    }
}

fn mediaplayer() -> MediaPlayer {
    let instance = Instance::new().unwrap();
    let md = Media::new_path(
        &instance,
        "/home/lsund/Documents/git/tham/library/test.mp3",
    )
    .unwrap();
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);
    return mdp;
}

pub fn init(tx: mpsc::Sender<String>, rx: mpsc::Receiver<MediaCtrl>) {
    thread::spawn(move || {
        let mdp = mediaplayer();
        loop {
            match rx.recv() {
                Ok(MediaCtrl::PlayOrPause) => {
                    play_or_pause(&mdp);
                }
                Ok(MediaCtrl::SpeedUp) => {
                    speed_up(&mdp);
                }
                Ok(MediaCtrl::SpeedDown) => {
                    speed_down(&mdp);
                }
                Ok(MediaCtrl::Meta) => match get_meta(&mdp) {
                    None => tx.send("Error".to_owned()).unwrap(),
                    Some(x) => tx.send(x).unwrap(),
                },
                Ok(_) => {}
                Err(_) => {}
            }
        }
    });
}
