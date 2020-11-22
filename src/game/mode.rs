use std::fmt;

use rand::{seq::SliceRandom, Rng};

use crate::game::{
    round::{ChordsRound, IntervalsRound, Round},
    scorekeeper::Scorekeeper,
};

#[derive(fmt::Debug, Copy, Clone)]
pub enum GameMode {
    Mixed,
    Intervals,
    Chords,
}

impl GameMode {
    pub fn play_round(&self, rng: &mut impl Rng, scorekeeper: &mut Scorekeeper) {
        match *self {
            GameMode::Mixed => {
                let round_mode = [GameMode::Intervals, GameMode::Chords].choose(rng).unwrap();
                round_mode.play_round(rng, scorekeeper);
            }
            GameMode::Intervals => IntervalsRound::new(rng).play(scorekeeper),
            GameMode::Chords => ChordsRound::new(rng).play(scorekeeper),
        }
    }
}
