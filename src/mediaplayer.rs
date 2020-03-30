use std::sync::mpsc;
use std::thread;
use vlc::sys;
use vlc::{Instance, Media, MediaPlayer, Meta};

static RATE_DELTA: f32 = 0.002;

pub enum MediaCtrl {
    PlayOrPause,
    SpeedDown,
    SpeedUp,
    BassDown,
    Load(String),
}

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

fn make_equalizer() -> *mut sys::libvlc_equalizer_t {
    let n_bands = 18;
    unsafe {
        let eq = sys::libvlc_audio_equalizer_new();
        for i in 0..n_bands + 1 {
            sys::libvlc_audio_equalizer_set_amp_at_index(eq, 10.0, i);
        }
        return eq;
    }
}

fn apply_equalizer(
    mdp: &MediaPlayer,
    eq: *mut sys::libvlc_equalizer_t,
) -> *mut sys::libvlc_equalizer_t {
    unsafe {
        let amp = sys::libvlc_audio_equalizer_get_amp_at_index(eq, 0);
        sys::libvlc_audio_equalizer_set_amp_at_index(eq, amp + 1.0, 0);
        sys::libvlc_media_player_set_equalizer(mdp.raw(), eq);
    }
    return eq;
}

pub fn init(tx: mpsc::Sender<String>, rx: mpsc::Receiver<MediaCtrl>) {
    thread::spawn(move || {
        let mut mdp = mediaplayer(None);
        let mut eq = make_equalizer();
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
                Ok(MediaCtrl::BassDown) => {
                    eq = apply_equalizer(&mdp, eq);
                    apply_equalizer(&mdp, eq);
                }
                Ok(MediaCtrl::Load(path)) => {
                    mdp = mediaplayer(Some(path));
                    mdp.get_media().map(|m| m.parse());
                    match get_meta(&mdp) {
                        Some(data) => tx.send(data).unwrap(),
                        None => tx.send("Error".to_owned()).unwrap(),
                    }
                }
                Err(_) => {}
            }
        }
    });
}
