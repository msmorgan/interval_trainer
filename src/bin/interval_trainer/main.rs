use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use crate::game::mode::GameMode;
use crate::game::scorekeeper::Scorekeeper;

mod game;

#[derive(Debug, structopt::StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    #[structopt(default_value)]
    mode: GameMode,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            mode: GameMode::Mixed,
        }
    }
}

#[paw::main]
fn main(options: Options) {
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
