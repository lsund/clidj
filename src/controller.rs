extern crate vlc;

use crate::application::Application;

pub enum AppCtrl {
    Continue,
    Stop,
    Refresh,
}

pub enum MediaCtrl {
    PlayOrPause,
    SpeedDown,
    SpeedUp,
    Meta,
}

enum Key {
    Num(u32),
    Letter(char),
    GreaterThan,
    LessThan,
    Unknown,
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

pub fn handle_char(ch_: i32, app: &mut Application) -> AppCtrl {
    let ch = to_key(ch_);
    match ch {
        Key::Num(0) => {
            app.library.load(0);
            app.prompt_history.update(format!(
                "{} loaded into deck\n",
                app.library.content.get(&0).unwrap()
            ));
            return AppCtrl::Continue;
        }
        Key::Num(1) => {
            app.library.load(1);
            app.prompt_history.update(format!(
                "{} loaded into deck\n",
                app.library.content.get(&1).unwrap()
            ));
            return AppCtrl::Continue;
        }
        Key::Letter('D') => {
            let s = app.library.list();
            app.prompt_history.update(format!("Library: \n{}\n", &s));
            return AppCtrl::Continue;
        }
        Key::Letter('M') => {
            app.tx.send(MediaCtrl::Meta).unwrap();
            match app.rx.recv() {
                Ok(x) => {
                    app.prompt_history.update(format!("Meta: {}\n", x));
                }
                Err(_) => {}
            }
            return AppCtrl::Continue;
        }
        Key::Letter('S') => {
            app.prompt_history.update("playing\n".to_owned());
            app.tx.send(MediaCtrl::PlayOrPause).unwrap();
            return AppCtrl::Continue;
        }
        Key::LessThan => {
            app.prompt_history.update("speed down\n".to_owned());
            app.tx.send(MediaCtrl::SpeedDown).unwrap();
            return AppCtrl::Continue;
        }
        Key::GreaterThan => {
            app.prompt_history.update("speed up\n".to_owned());
            app.tx.send(MediaCtrl::SpeedUp).unwrap();
            return AppCtrl::Continue;
        }
        Key::Letter('Q') => {
            return AppCtrl::Stop;
        }
        Key::Letter('R') => {
            return AppCtrl::Refresh;
        }
        _ => {
            return AppCtrl::Continue;
        }
    }
}
