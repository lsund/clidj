extern crate ncurses;
extern crate vlc;

mod controller;
mod mediaplayer;

static PROMPT_HISTORY_SIZE: usize = 5;

use ncurses::*;
use std::sync::mpsc;

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

fn display_history(history: &Vec<String>, current_length: usize) {
    let start_index =
        history.len() - std::cmp::min(current_length, PROMPT_HISTORY_SIZE);
    for item in &history[start_index..history.len()] {
        addstr(&item);
    }
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

    let (tx, rx) = mpsc::channel();
    mediaplayer::init(rx);

    let mut vec = Vec::new();
    let mut current_length = 0;
    loop {
        match controller::handle_char(getch()) {
            Response::Stop => {
                break;
            }
            Response::Continue => {}
            Response::Print(x) => {
                addstr(&x);
                vec.push(x);
                current_length += 1;
            }
            Response::Refresh => {
                clear();
                display_help();
            }
            Response::PlayOrPause => {
                addstr("playing\n");
                tx.send(Response::PlayOrPause).unwrap();
            }
            Response::SpeedUp => {
                addstr("speed up\n");
                tx.send(Response::SpeedUp).unwrap();
            }
            Response::SpeedDown => {
                addstr("speed down\n");
                tx.send(Response::SpeedDown).unwrap();
            }
        }
        clear();
        display_help();
        // addstr(&format!("{}\n\n", mdp.get_time().unwrap()));
        display_history(&vec, current_length);
    }
    endwin();
}
