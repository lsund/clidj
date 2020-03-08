use crate::controller;
use controller::MediaCtrl;
use std::sync::mpsc;
use std::thread;
use vlc::{Instance, Media, MediaPlayer, Meta};

static RATE_DELTA: f32 = 0.002;

fn sequence(xs: Vec<Option<String>>) -> Option<Vec<String>> {
    return xs.iter().fold(Some(Vec::new()), move |acc, z| match z {
        Some(x) => {
            let mut y: Vec<String> = acc.unwrap();
            y.push(x.to_owned());
            return Some(y);
        }
        None => None,
    });
}

fn get_meta(mdp: &MediaPlayer) -> Option<String> {
    return mdp.get_media().and_then(|m| {
        sequence(vec![
            m.get_meta(Meta::Title),
            m.get_meta(Meta::Artist),
            m.get_meta(Meta::Genre),
        ])
        .map(|ms| ms.join("\n"))
    });
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
    mdp.get_media().map(|_| {
        if mdp.is_playing() {
            mdp.pause();
        } else {
            mdp.play().unwrap();
        }
    });
}

fn mediaplayer(mpath: Option<String>) -> MediaPlayer {
    let instance = Instance::new().unwrap();
    let mdp = MediaPlayer::new(&instance).unwrap();
    mpath.map(|path| {
        let md = Media::new_path(&instance, path).unwrap();
        mdp.set_media(&md);
    });
    return mdp;
}

pub fn init(tx: mpsc::Sender<String>, rx: mpsc::Receiver<MediaCtrl>) {
    thread::spawn(move || {
        let mut mdp = mediaplayer(None);
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
                Ok(MediaCtrl::Load(x)) => {
                    mdp = mediaplayer(Some(x));
                    mdp.get_media().map(|m| m.parse());
                    match get_meta(&mdp) {
                        None => tx.send("Error".to_owned()).unwrap(),
                        Some(x) => tx.send(x).unwrap(),
                    }
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }
    });
}
