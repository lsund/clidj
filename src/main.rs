extern crate ncurses;
extern crate vlc;

use ncurses::*;
// use std::thread;
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
    addstr("Press 's' to play music.\n");

    let mut ch: String = keyname(getch()).unwrap();

    loop {
        if ch == "q" {
            break;
        } else if ch == "s" {
            addstr(&format!("{} playing\n", &ch));
            play_music(&mdp);
        } else {
            addstr(&format!("Press 'q' to quit.\n"));
        }
        ch = keyname(getch()).unwrap();
    }

    refresh();

    endwin();

    // thread::sleep(::std::time::Duration::from_secs(360));
}
