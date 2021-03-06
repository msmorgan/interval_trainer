use clap::arg_enum;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::game::round::{ChordsRound, IntervalsRound, Round, ScalesRound};
use crate::game::scorekeeper::Scorekeeper;

arg_enum! {
    #[derive(Debug, Copy, Clone)]
    pub enum GameMode {
        Mixed,
        Intervals,
        Chords,
        Scales,
    }
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Mixed
    }
}

impl GameMode {
    pub fn play_round(&self, rng: &mut impl Rng, scorekeeper: &mut Scorekeeper) {
        match *self {
            GameMode::Mixed => {
                let round_mode = [GameMode::Intervals, GameMode::Chords, GameMode::Scales]
                    .choose(rng)
                    .unwrap();
                round_mode.play_round(rng, scorekeeper);
            }
            GameMode::Intervals => IntervalsRound::new(rng).play(scorekeeper),
            GameMode::Chords => ChordsRound::new(rng).play(scorekeeper),
            GameMode::Scales => ScalesRound::new(rng).play(scorekeeper),
        }
    }
}
