#![feature(slice_concat_trait)]

use std::{
    fmt,
    io::{self, prelude::*},
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use rand::prelude::*;

use crate::{
    accidental::Accidental,
    chord::quality::{ChordQuality, triads, sevenths},
    game::scorekeeper::Scorekeeper,
    interval::canonical::CanonicalInterval,
    note::Note,
    note_name::NoteName,
};

mod accidental;
mod chord;
mod game;
mod interval;
mod note;
mod note_name;

const STANDARD_NOTES: [Note; 17] = {
    use {Accidental::*, NoteName::*};

    let mut result = [Note(C, Natural); 17];
    let mut pitch = 0;
    let mut i = 0;
    while i < 17 {
        let note = match Note::from_pitch(pitch) {
            Some(note) => note,
            None => Note(C, Natural),
        };
        result[i] = note;
        i += 1;

        match note.accidental() {
            Accidental::Sharp => {
                result[i] = Note(note.note_name().step_up(), Flat);
                i += 1;
            }
            _ => {}
        }

        pitch += 1;
    }
    result
};

const STANDARD_INTERVALS: [CanonicalInterval; 11] = {
    use CanonicalInterval::*;
    [
        MinorSecond,
        MajorSecond,
        MinorThird,
        MajorThird,
        PerfectFourth,
        Tritone,
        PerfectFifth,
        MinorSixth,
        MajorSixth,
        MinorSeventh,
        MajorSeventh,
    ]
};

lazy_static::lazy_static! {
    pub static ref STANDARD_CHORD_QUALITIES: Vec<ChordQuality> = vec![
        triads::MAJOR.clone(),
        triads::MINOR.clone(),
        triads::DIMINISHED.clone(),
        triads::AUGMENTED.clone(),
        sevenths::MAJOR.clone(),
        sevenths::MINOR.clone(),
        sevenths::DIMINISHED.clone(),
        sevenths::HALF_DIMINISHED.clone(),
        sevenths::DOMINANT.clone(),
    ];
}

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

#[derive(fmt::Debug, Copy, Clone)]
enum GameMode {
    Mixed,
    Intervals,
    Chords,
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

pub trait Round {
    fn play(&self, scorekeeper: &mut Scorekeeper) {
        print!("{}: ", self.prompt());
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

struct IntervalsRound {
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

struct ChordsRound {
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
