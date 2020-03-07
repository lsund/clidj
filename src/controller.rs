extern crate vlc;
use std::fs;

pub enum Response {
    Continue,
    Stop,
    Print(String),
    Refresh,
    PlayOrPause,
    SpeedDown,
    SpeedUp,
}

enum Key {
    S,
    R,
    L,
    Q,
    GreaterThan,
    LessThan,
    Unknown,
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
        114 => Key::R,
        108 => Key::L,
        113 => Key::Q,
        62 => Key::GreaterThan,
        60 => Key::LessThan,
        _ => Key::Unknown,
    }
}

pub fn handle_char(ch_: i32) -> Response {
    let ch = to_key(ch_);
    let mut resp = Response::Continue;
    match ch {
        Key::Q => {
            resp = Response::Stop;
        }
        Key::S => {
            resp = Response::PlayOrPause;
        }
        Key::R => {
            resp = Response::Refresh;
        }
        Key::L => {
            let s = list();
            resp = Response::Print(format!("Library: \n{}\n", &s));
        }
        Key::LessThan => {
            resp = Response::SpeedDown;
        }
        Key::GreaterThan => {
            resp = Response::SpeedUp;
        }
        _ => {
            Response::Print("press 'q' to quit\n".to_owned());
        }
    }
    return resp;
}
