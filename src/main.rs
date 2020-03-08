extern crate ncurses;
extern crate vlc;

mod controller;
mod library;
mod mediaplayer;
mod prompt_history;

use ncurses::*;
use std::sync::mpsc;

use controller::Message;

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

    let (forward_mp_tx, forward_mp_rx) = mpsc::channel();
    let (backward_mp_tx, backward_mp_rx) = mpsc::channel();

    mediaplayer::init(backward_mp_tx, forward_mp_rx);

    let mut hist = prompt_history::make();
    loop {
        match controller::handle_char(getch()) {
            Message::Stop => {
                break;
            }
            Message::Continue => {}
            Message::Print(x) => {
                hist.update(x);
            }
            Message::Refresh => {
                hist.clear();
            }
            Message::PlayOrPause => {
                hist.update("playing\n".to_owned());
                forward_mp_tx.send(Message::PlayOrPause).unwrap();
            }
            Message::SpeedUp => {
                hist.update("speed up\n".to_owned());
                forward_mp_tx.send(Message::SpeedUp).unwrap();
            }
            Message::SpeedDown => {
                hist.update("speed down\n".to_owned());
                forward_mp_tx.send(Message::SpeedDown).unwrap();
            }
            Message::Meta => {
                forward_mp_tx.send(Message::Meta).unwrap();
                match backward_mp_rx.recv() {
                    Ok(x) => {
                        hist.update(format!("Meta: {}\n", x));
                    }
                    Err(_) => {}
                }
            }
        }
        clear();
        display_help();
        hist.display();
        // addstr(&format!("{}\n\n", mdp.get_time().unwrap()));
    }
    endwin();
}
