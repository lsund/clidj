extern crate vlc;

use crate::application::Application;
use crate::mediaplayer::MediaCtrl;

pub struct Controller {
    active_control: PlaybackCtrl,
}

pub enum AppCtrl {
    Continue,
    Stop,
    Refresh,
}

enum PlaybackCtrl {
    Tempo,
    Bass,
}

enum Key {
    Num(u32),
    Letter(char),
    GreaterThan,
    LessThan,
    Unknown,
}

pub fn make() -> Controller {
    return Controller {
        active_control: PlaybackCtrl::Tempo,
    };
}

fn to_key(x: i32) -> Key {
    match x {
        48 => Key::Num(0),
        49 => Key::Num(1),
        50 => Key::Num(2),
        51 => Key::Num(3),
        52 => Key::Num(4),
        53 => Key::Num(5),
        54 => Key::Num(6),
        55 => Key::Num(7),
        56 => Key::Num(8),
        57 => Key::Num(9),
        97 => Key::Letter('A'),
        98 => Key::Letter('B'),
        99 => Key::Letter('C'),
        100 => Key::Letter('D'),
        101 => Key::Letter('E'),
        102 => Key::Letter('F'),
        103 => Key::Letter('G'),
        104 => Key::Letter('H'),
        105 => Key::Letter('I'),
        106 => Key::Letter('J'),
        107 => Key::Letter('K'),
        108 => Key::Letter('L'),
        109 => Key::Letter('M'),
        110 => Key::Letter('N'),
        111 => Key::Letter('O'),
        112 => Key::Letter('P'),
        113 => Key::Letter('Q'),
        114 => Key::Letter('R'),
        115 => Key::Letter('S'),
        116 => Key::Letter('T'),
        117 => Key::Letter('U'),
        118 => Key::Letter('V'),
        119 => Key::Letter('W'),
        120 => Key::Letter('X'),
        121 => Key::Letter('Y'),
        122 => Key::Letter('Z'),
        62 => Key::GreaterThan,
        60 => Key::LessThan,
        _ => Key::Unknown,
    }
}

fn dispatch(msg: MediaCtrl, app: &mut Application) {
    match app.tx.send(msg) {
        Err(_) => app.log("Internal Error".to_owned()),
        _ => {}
    }
}

fn receive(app: &mut Application) {
    match app.rx.recv() {
        Ok(data) => {
            app.log(format!("{}\n", data));
        }
        Err(_) => {}
    }
}

pub fn handle_char(ch_: i32, app: &mut Application) -> AppCtrl {
    let ch = to_key(ch_);
    let mut resp = AppCtrl::Continue;
    match ch {
        Key::Num(x) => {
            app.library.load(x);
            match app.get_song(&x) {
                Some(path) => {
                    dispatch(MediaCtrl::Load(path.to_owned()), app);
                    receive(app)
                }
                None => {
                    app.log(format!("Could not load song\n"));
                }
            }
        }
        Key::Letter('D') => {
            let s = app.library.list();
            app.log(format!("Library: \n{}\n", &s));
        }
        Key::Letter('S') => {
            dispatch(MediaCtrl::PlayOrPause, app);
        }
        Key::Letter('B') => app.controller.active_control = PlaybackCtrl::Bass,
        Key::LessThan => match app.controller.active_control {
            PlaybackCtrl::Tempo => {
                app.log("speed down\n".to_owned());
                dispatch(MediaCtrl::SpeedDown, app);
            }
            PlaybackCtrl::Bass => {
                app.log("bass down\n".to_owned());
                dispatch(MediaCtrl::BassDown, app)
            }
        },
        Key::GreaterThan => {
            app.log("speed up\n".to_owned());
            dispatch(MediaCtrl::SpeedUp, app);
        }
        Key::Letter('Q') => {
            resp = AppCtrl::Stop;
        }
        Key::Letter('R') => {
            resp = AppCtrl::Refresh;
        }
        _ => {}
    }
    return resp;
}
