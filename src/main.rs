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

fn main() {
    let options = Options::default();
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
