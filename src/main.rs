extern crate ncurses;
extern crate vlc;

use ncurses::*;
// use std::thread;
use std::fs;
use vlc::{Instance, Media, MediaPlayer};

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
    let ch = keyname(ch_).unwrap();
    let mut quit = false;
    if ch == "q" {
        quit = true;
    } else if ch == "s" {
        addstr("playing\n");
        play(&mdp);
    } else if ch == "l" {
        let s = list();
        addstr("Library: \n");
        addstr(&s);
        addstr("\n");
    } else if ch == "<" {
        let rate = speed_down(&mdp);
        addstr(&format!("{:.*}\n", 2, rate,));
    } else if ch == ">" {
        let rate = speed_up(&mdp);
        addstr(&format!("{:.*}\n", 2, rate,));
    } else {
        addstr("press 'q' to quit\n");
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
