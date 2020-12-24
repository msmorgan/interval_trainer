use std::{
    borrow::Cow,
    io::{self, prelude::*},
    time::Duration,
};

use either::*;
use rand::prelude::*;

use super::{STANDARD_CHORD_QUALITIES, STANDARD_INTERVALS, STANDARD_NOTES, STANDARD_SCALES};
use crate::{
    chord::quality::ChordQuality,
    game::scorekeeper::Scorekeeper,
    interval::canonical::CanonicalInterval,
    note::{Note, UnrecognizedNote},
    scale::{modal::ModalScale, mode::Mode, Scale},
};

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

pub trait Round {
    const ROUND_LABEL: &'static str;

    fn play(&self, scorekeeper: &mut Scorekeeper) {
        print!("{} - {}: ", Self::ROUND_LABEL, self.prompt());
        io::stdout().flush().unwrap();

        let input = match get_next_input().unwrap() {
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
    descending: bool,
    interval: CanonicalInterval,
}

impl IntervalsRound {
    pub fn new(rng: &mut impl Rng) -> Self {
        IntervalsRound {
            root_note: STANDARD_NOTES.choose(rng).cloned().unwrap(),
            descending: rng.gen(),
            interval: STANDARD_INTERVALS.choose(rng).cloned().unwrap(),
        }
    }
}

impl Round for IntervalsRound {
    const ROUND_LABEL: &'static str = "Interval";

    fn prompt(&self) -> String {
        format!(
            "{} {} a {}",
            self.root_note,
            if self.descending { "down" } else { "up" },
            self.interval
        )
    }

    fn evaluate(&self, input: String, scorekeeper: &mut Scorekeeper) -> String {
        let expected = if self.descending {
            self.root_note - self.interval
        } else {
            self.root_note + self.interval
        };

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
    const ROUND_LABEL: &'static str = "Chord";

    fn prompt(&self) -> String {
        format!("{} {}", self.root_note, self.chord)
    }

    fn evaluate(&self, input: String, scorekeeper: &mut Scorekeeper) -> String {
        let notes = notes_from_input(input);

        match notes {
            Ok(notes) => {
                let expected = self.chord.spell(self.root_note);
                let correct = notes == expected;
                let duration = scorekeeper.add_result(correct);
                feedback_expected_notes(expected, correct, duration)
            }
            Err(e) => format!("Error: {:?}.", e),
        }
    }
}

pub struct ScalesRound {
    root_note: Note,
    scale: Either<Scale, ModalScale>,
}

impl ScalesRound {
    pub fn new(rng: &mut impl Rng) -> Self {
        let root_note = STANDARD_NOTES.choose(rng).cloned().unwrap();
        let scale = if rng.gen() {
            Left(STANDARD_SCALES.choose(rng).cloned().unwrap())
        } else {
            let major = STANDARD_SCALES[0].clone();
            let mode = Mode::from(rng.gen_range(0, 7));
            Right(ModalScale::new(major, mode))
        };

        ScalesRound { root_note, scale }
    }
}

impl Round for ScalesRound {
    const ROUND_LABEL: &'static str = "Scale";

    fn prompt(&self) -> String {
        format!(
            "{} {}",
            self.root_note,
            match &self.scale {
                Left(scale) => Cow::Borrowed(&scale.name),
                Right(modal_scale) => Cow::Owned(format!("{:?}", &modal_scale.mode)),
            }
        )
    }

    fn evaluate(&self, input: String, scorekeeper: &mut Scorekeeper) -> String {
        let notes = notes_from_input(input);

        match notes {
            Ok(notes) => {
                let expected = match &self.scale {
                    Left(scale) => scale.spell(self.root_note),
                    Right(modal_scale) => modal_scale.spell(self.root_note),
                };
                let correct = notes == expected;
                let duration = scorekeeper.add_result(correct);
                feedback_expected_notes(expected, correct, duration)
            }
            Err(e) => format!("Error: {:?}.", e),
        }
    }
}

fn notes_from_input(input: String) -> Result<Vec<Note>, UnrecognizedNote> {
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
    notes
}

fn feedback_expected_notes(expected: Vec<Note>, correct: bool, duration: Duration) -> String {
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
