extern crate ncurses;
extern crate vlc;

mod controller;

use ncurses::*;
use vlc::{Instance, Media, MediaPlayer};

use controller::Response;

fn help() -> &'static str {
    return "
s: play music
p: pause music
l: list library
q: quit
<: slow down
>: speed up
";
}

fn mediaplayer() -> MediaPlayer {
    let instance = Instance::new().unwrap();
    let md = Media::new_path(
        &instance,
        "/home/lsund/Documents/git/tham/library/test.mp3",
    )
    .unwrap();
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);
    return mdp;
}

fn init_ncurses() {
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
}

fn main() {
    init_ncurses();
    addstr(help());
    let mdp = mediaplayer();
    loop {
        match controller::handle_char(&mdp, getch()) {
            Response::Stop => {
                break;
            }
            Response::Continue => {}
            Response::Print(x) => {
                addstr(&x);
            }
        }
    }
    endwin();
}
