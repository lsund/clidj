extern crate ncurses;
extern crate vlc;

use ncurses::*;
use std::thread;
use vlc::{Instance, Media, MediaPlayer};

fn play_music(mdp: &MediaPlayer) {
    mdp.play().unwrap();
    let _ = mdp.set_rate(0.9);
}

fn main() {
    let instance = Instance::new().unwrap();
    let md =
        Media::new_path(&instance, "/home/lsund/Documents/git/tham/test.mp3")
            .unwrap();

    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    println!("{}", mdp.get_rate());

    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    addstr("Press Play.");

    let ch = getch();
    if ch == KEY_F1 {
        /* Enable attributes and output message. */
        attron(A_BOLD() | A_BLINK());
        addstr("\nF1");
        attroff(A_BOLD() | A_BLINK());
        addstr(" pressed");
    } else {
        /* Enable attributes and output message. */
        addstr("\nKey pressed: ");
        attron(A_BOLD() | A_BLINK());
        addstr(format!("{}\n", ch).as_ref());
        attroff(A_BOLD() | A_BLINK());
    }

    refresh();

    getch();

    play_music(&mdp);

    endwin();

    thread::sleep(::std::time::Duration::from_secs(360));
}
