use crate::controller;
use controller::Response;
use std::thread;
use vlc::{Instance, Media, MediaPlayer};

fn speed_up(mdp: &MediaPlayer) -> f32 {
    let rate = mdp.get_rate();
    let rate_ = rate + 0.01;
    let _ = mdp.set_rate(rate_);
    return rate_;
}

fn speed_down(mdp: &MediaPlayer) -> f32 {
    let rate = mdp.get_rate();
    let rate_ = rate - 0.01;
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

pub fn init(rx: std::sync::mpsc::Receiver<Response>) {
    thread::spawn(move || {
        let mdp = mediaplayer();
        loop {
            match rx.recv() {
                Ok(Response::PlayOrPause) => {
                    play_or_pause(&mdp);
                }
                Ok(Response::SpeedUp) => {
                    speed_up(&mdp);
                }
                Ok(Response::SpeedDown) => {
                    speed_down(&mdp);
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }
    });
}
