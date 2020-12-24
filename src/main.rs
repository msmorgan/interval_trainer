use std::{
    fmt,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use crate::game::{mode::GameMode, scorekeeper::Scorekeeper};

mod accidental;
mod chord;
mod game;
mod interval;
mod note;
mod note_name;
mod scale;

#[derive(fmt::Debug)]
struct Options {
    mode: GameMode,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            mode: GameMode::Mixed,
        }
    }
}

fn get_options() -> Options {
    let mut args = std::env::args();
    let process_name = args.next().unwrap();
    let mode_arg = args.next();
    Options {
        mode: match mode_arg.as_ref().map(|s| s.as_str()) {
            None | Some("mixed") => GameMode::Mixed,
            Some("intervals") => GameMode::Intervals,
            Some("chords") => GameMode::Chords,
            Some("scales") => GameMode::Scales,
            Some(_) => {
                eprintln!("Usage: {} [mixed|intervals|chords|scales]", process_name);
                std::process::exit(1);
            }
        },
    }
}

fn main() {
    let options = get_options();
    let scorekeeper = Arc::new(Mutex::new(Scorekeeper::new()));

    {
        let scorekeeper = scorekeeper.clone();
        ctrlc::set_handler(move || {
            scorekeeper.lock().unwrap().report_and_exit();
        })
        .expect("Error setting Ctrl-C handler.");
    }

    let mut rng = rand::thread_rng();

    loop {
        options
            .mode
            .play_round(&mut rng, scorekeeper.lock().unwrap().deref_mut());
    }
}
