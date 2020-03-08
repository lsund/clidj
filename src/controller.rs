extern crate vlc;
use std::fs;

pub enum Message {
    Continue,
    Stop,
    Print(String),
    Refresh,
    PlayOrPause,
    SpeedDown,
    SpeedUp,
    Meta,
}

enum Key {
    S,
    R,
    L,
    Q,
    M,
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
        // 97 => Key::A,
        108 => Key::L,
        109 => Key::M,
        113 => Key::Q,
        114 => Key::R,
        115 => Key::S,
        62 => Key::GreaterThan,
        60 => Key::LessThan,
        _ => Key::Unknown,
    }
}

pub fn handle_char(ch_: i32) -> Message {
    let ch = to_key(ch_);
    let mut resp = Message::Continue;
    match ch {
        Key::Q => {
            resp = Message::Stop;
        }
        Key::S => {
            resp = Message::PlayOrPause;
        }
        Key::R => {
            resp = Message::Refresh;
        }
        Key::L => {
            let s = list();
            resp = Message::Print(format!("Library: \n{}\n", &s));
        }
        Key::LessThan => {
            resp = Message::SpeedDown;
        }
        Key::GreaterThan => {
            resp = Message::SpeedUp;
        }
        Key::M => {
            resp = Message::Meta;
        }
        _ => {
            Message::Print("press 'q' to quit\n".to_owned());
        }
    }
    return resp;
}
