use std::{
    fmt,
    io::{self, prelude::*},
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use rand::prelude::*;

use crate::game::{mode::GameMode, scorekeeper::Scorekeeper};

mod accidental;
mod chord;
mod game;
mod interval;
mod note;
mod note_name;

fn get_next_input() -> io::Result<Option<String>> {
    let mut buf = String::new();
    io::stdin().lock().read_line(&mut buf)?;

    buf.truncate(buf.trim_end().len()); // Remove trailing newline.

    if buf.is_empty() || buf == "exit" {
        Ok(None)
    } else {
        Ok(Some(buf))
    }
}

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
