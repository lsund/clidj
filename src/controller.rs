extern crate vlc;
use std::fs;
use vlc::MediaPlayer;

pub enum Response {
    Continue,
    Stop,
    Print(String),
}

enum Key {
    S,
    P,
    L,
    Q,
    GreaterThan,
    LessThan,
    Unknown,
}

fn play_or_pause(mdp: &MediaPlayer) {
    if mdp.is_playing() {
        mdp.pause();
    } else {
        mdp.play().unwrap();
        let _ = mdp.set_rate(1.0);
    }
}

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

fn list() -> String {
    let y = fs::read_dir("./library").unwrap();
    let mut foo = String::new();
    for path in y {
        let x = &path.unwrap().path().display().to_string();
        foo.push_str(x);
    }
    return foo;
}

fn to_key(x: i32) -> Key {
    match x {
        115 => Key::S,
        112 => Key::P,
        108 => Key::L,
        113 => Key::Q,
        62 => Key::GreaterThan,
        60 => Key::LessThan,
        _ => Key::Unknown,
    }
}

pub fn handle_char(mdp: &MediaPlayer, ch_: i32) -> Response {
    let ch = to_key(ch_);
    let mut resp = Response::Continue;
    match ch {
        Key::Q => {
            resp = Response::Stop;
        }
        Key::S => {
            resp = Response::Print("playing\n".to_owned());
            play_or_pause(&mdp);
        }
        Key::P => {}
        Key::L => {
            let s = list();
            resp = Response::Print(format!("Library: \n{}\n", &s));
        }
        Key::LessThan => {
            let rate = speed_down(&mdp);
            resp = Response::Print(format!("{:.*}\n", 2, rate,));
        }
        Key::GreaterThan => {
            let rate = speed_up(&mdp);
            resp = Response::Print(format!("{:.*}\n", 2, rate,));
        }
        _ => {
            Response::Print("press 'q' to quit\n".to_owned());
        }
    }
    return resp;
}
