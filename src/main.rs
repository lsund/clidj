extern crate ncurses;
extern crate vlc;

use ncurses::*;
// use std::thread;
use std::fs;
use vlc::{Instance, Media, MediaPlayer};

enum Key {
    S,
    L,
    Q,
    GreaterThan,
    LessThan,
    Unknown,
}

fn int_to_key(x: i32) -> Key {
    match x {
        115 => Key::S,
        108 => Key::L,
        113 => Key::Q,
        62 => Key::GreaterThan,
        60 => Key::LessThan,
        _ => Key::Unknown,
    }
}

fn help() -> &'static str {
    return "
s: play music
l: list library
q: quit
<: slow down
>: speed up
";
}

fn play(mdp: &MediaPlayer) {
    mdp.play().unwrap();
    let _ = mdp.set_rate(1.0);
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

fn handle_char(mdp: &MediaPlayer, ch_: i32) -> bool {
    let ch = int_to_key(ch_);
    let mut quit = false;
    match ch {
        Key::Q => {
            quit = true;
        }
        Key::S => {
            addstr("playing\n");
            play(&mdp);
        }
        Key::L => {
            let s = list();
            addstr("Library: \n");
            addstr(&s);
            addstr("\n");
        }
        Key::LessThan => {
            let rate = speed_down(&mdp);
            addstr(&format!("{:.*}\n", 2, rate,));
        }
        Key::GreaterThan => {
            let rate = speed_up(&mdp);
            addstr(&format!("{:.*}\n", 2, rate,));
        }
        _ => {
            addstr("press 'q' to quit\n");
        }
    }
    return quit;
}

fn main() {
    let instance = Instance::new().unwrap();
    let md = Media::new_path(
        &instance,
        "/home/lsund/Documents/git/tham/library/test.mp3",
    )
    .unwrap();

    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    addstr(help());

    loop {
        let x = handle_char(&mdp, getch());
        if x {
            break;
        }
    }

    refresh();

    endwin();

    // thread::sleep(::std::time::Duration::from_secs(360));
}
