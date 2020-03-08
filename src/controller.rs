extern crate vlc;
use std::fs;

static LIBRARY_DIR: &str = "/home/lsund/Media/audio/library";

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
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    GreaterThan,
    LessThan,
    Unknown,
}

fn list() -> String {
    let y = fs::read_dir(LIBRARY_DIR).unwrap();
    let mut foo = String::new();
    let mut i = 0;
    for path in y {
        let x = &path.unwrap().path().display().to_string();
        foo.push_str(&format!("{} {}\n", i, x));
        i += 1;
    }
    return foo;
}

fn to_key(x: i32) -> Key {
    match x {
        97 => Key::A,
        98 => Key::B,
        99 => Key::C,
        100 => Key::D,
        101 => Key::E,
        102 => Key::F,
        103 => Key::G,
        104 => Key::H,
        105 => Key::I,
        106 => Key::J,
        107 => Key::K,
        108 => Key::L,
        109 => Key::M,
        110 => Key::N,
        111 => Key::O,
        112 => Key::P,
        113 => Key::Q,
        114 => Key::R,
        115 => Key::S,
        116 => Key::T,
        117 => Key::U,
        118 => Key::V,
        119 => Key::W,
        120 => Key::X,
        121 => Key::Y,
        122 => Key::Z,
        62 => Key::GreaterThan,
        60 => Key::LessThan,
        _ => Key::Unknown,
    }
}

pub fn handle_char(ch_: i32) -> Message {
    let ch = to_key(ch_);
    match ch {
        Key::Q => {
            return Message::Stop;
        }
        Key::S => {
            return Message::PlayOrPause;
        }
        Key::R => {
            return Message::Refresh;
        }
        Key::D => {
            let s = list();
            return Message::Print(format!("Library: \n{}\n", &s));
        }
        Key::LessThan => {
            return Message::SpeedDown;
        }
        Key::GreaterThan => {
            return Message::SpeedUp;
        }
        Key::M => {
            return Message::Meta;
        }
        _ => {
            Message::Print("press 'q' to quit\n".to_owned());
        }
    }
    return Message::Continue;
}
