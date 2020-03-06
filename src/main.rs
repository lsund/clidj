extern crate ncurses;
extern crate vlc;

mod controller;

static PROMPT_HISTORY_SIZE: usize = 5;

use ncurses::*;
use vlc::{Instance, Media, MediaPlayer};

use controller::Response;

fn display_help() {
    addstr(
        "
s: play music
p: pause music
l: list library
q: quit
<: slow down
>: speed up
",
    );
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
    display_help();
    let mdp = mediaplayer();
    let mut vec = Vec::new();
    let mut max_len = 0;
    loop {
        match controller::handle_char(&mdp, getch()) {
            Response::Stop => {
                break;
            }
            Response::Continue => {}
            Response::Print(x) => {
                addstr(&x);
                vec.push(x);
                max_len += 1;
            }
            Response::Refresh => {
                clear();
                display_help();
            }
        }
        clear();
        display_help();
        let start_index =
            vec.len() - std::cmp::min(max_len, PROMPT_HISTORY_SIZE);
        for x in &vec[start_index..vec.len()] {
            addstr(&x);
        }
    }
    endwin();
}
