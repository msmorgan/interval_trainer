use std::{io, io::Write};

use rand::{seq::SliceRandom, Rng};

use super::{STANDARD_CHORD_QUALITIES, STANDARD_INTERVALS, STANDARD_NOTES};
use crate::{
    chord::quality::ChordQuality,
    game::scorekeeper::Scorekeeper,
    interval::canonical::CanonicalInterval,
    note::Note,
};

pub trait Round {
    fn play(&self, scorekeeper: &mut Scorekeeper) {
        print!("{}: ", self.prompt());
        io::stdout().flush().unwrap();

        let input = match crate::get_next_input().unwrap() {
            Some(s) => s,
            None => scorekeeper.report_and_exit(),
        };

        println!("  {}", self.evaluate(input, scorekeeper));
    }
    fn prompt(&self) -> String;
    fn evaluate(&self, input: String, scorekeeper: &mut Scorekeeper) -> String;
}

pub struct IntervalsRound {
    root_note: Note,
    interval: CanonicalInterval,
}

impl IntervalsRound {
    pub fn new(rng: &mut impl Rng) -> Self {
        IntervalsRound {
            root_note: STANDARD_NOTES.choose(rng).cloned().unwrap(),
            interval: STANDARD_INTERVALS.choose(rng).cloned().unwrap(),
        }
    }
}

impl Round for IntervalsRound {
    fn prompt(&self) -> String {
        format!("Up a {} from {}", self.interval, self.root_note)
    }

    fn evaluate(&self, input: String, scorekeeper: &mut Scorekeeper) -> String {
        let expected = self.root_note + self.interval;

        let correct = match input.parse::<Note>() {
            Ok(input_note) => input_note == expected,
            Err(e) => return format!("Error: {:?}.", e),
        };

        let duration = scorekeeper.add_result(correct);
        if correct {
            format!("Correct! ({:.2} sec.)", duration.as_secs_f32())
        } else {
            format!("Incorrect! (Expected {}.)", expected)
        }
    }
}

pub struct ChordsRound {
    root_note: Note,
    chord: ChordQuality,
}

impl ChordsRound {
    pub fn new(rng: &mut impl Rng) -> Self {
        ChordsRound {
            root_note: STANDARD_NOTES.choose(rng).cloned().unwrap(),
            chord: STANDARD_CHORD_QUALITIES.choose(rng).cloned().unwrap(),
        }
    }
}

impl Round for ChordsRound {
    fn prompt(&self) -> String {
        format!("Notes in {} {}", self.root_note, self.chord)
    }

    fn evaluate(&self, input: String, scorekeeper: &mut Scorekeeper) -> String {
        let notes = input
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<Note>())
            .try_fold(vec![], |mut acc, note| match note {
                Ok(note) => {
                    acc.push(note);
                    Ok(acc)
                }
                Err(e) => Err(e),
            });

        match notes {
            Ok(notes) => {
                let expected = self.chord.spell(self.root_note);
                let correct = notes == expected;
                let duration = scorekeeper.add_result(correct);
                if correct {
                    format!("Correct! ({:.2} sec.)", duration.as_secs_f32())
                } else {
                    let mut result = "Incorrect! (Expected".to_string();
                    for note in expected {
                        result.push(' ');
                        result.push_str(&note.to_string());
                    }
                    result.push(')');
                    result
                }
            }
            Err(e) => format!("Error: {:?}.", e),
        }
    }
}
