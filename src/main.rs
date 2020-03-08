extern crate ncurses;
extern crate vlc;

mod application;
mod controller;
mod library;
mod mediaplayer;
mod prompt_history;

use controller::AppCtrl;
use ncurses::*;
use std::sync::mpsc;

static LIBRARY_DIR: &str = "/home/lsund/Media/audio/library";

fn display_help() {
    addstr(
        "
s: play music
p: pause music
d: list library
q: quit
<: slow down
>: speed up
",
    );
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

    let (main_tx, mplayer_rx) = mpsc::channel();
    let (mplayer_tx, main_rx) = mpsc::channel();

    let mut app = application::make(
        LIBRARY_DIR,
        main_tx,
        main_rx,
        mplayer_tx,
        mplayer_rx,
    );
    loop {
        match controller::handle_char(getch(), &mut app) {
            AppCtrl::Stop => {
                break;
            }
            AppCtrl::Refresh => {
                app.prompt_history.clear();
            }
            _ => {}
        }
        clear();
        display_help();
        app.prompt_history.display();
        // addstr(&format!("{}\n\n", mdp.get_time().unwrap()));
    }
    endwin();
}
