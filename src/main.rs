use std::{
    io::{self, prelude::*},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use rand::prelude::*;

use self::{
    accidental::Accidental, interval::canonical::CanonicalInterval, note::Note, note_name::NoteName,
};
use game::scorekeeper::Scorekeeper;

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

fn main() {
    let scorekeeper = Arc::new(Mutex::new(Scorekeeper::new()));

    {
        let scorekeeper = scorekeeper.clone();
        ctrlc::set_handler(move || {
            scorekeeper.lock().unwrap().report_and_exit();
        })
        .expect("Error setting Ctrl-C handler.");
    }

    loop {
        let mut rng = rand::thread_rng();
        let note = STANDARD_NOTES.choose(&mut rng).unwrap().clone();
        let interval = STANDARD_INTERVALS.choose(&mut rng).unwrap().clone();

        print!("Up a {} from {}: ", interval, note);
        io::stdout().flush().unwrap();

        let input = match get_next_input().unwrap() {
            Some(s) => s,
            None => scorekeeper.lock().unwrap().report_and_exit(),
        };

        let mut correct = false;
        match input.parse::<Note>() {
            Ok(input_note) => {
                if input_note == note + interval {
                    correct = true;
                }
            }
            Err(e) => {
                println!("  Error: {:?}.", e);
                println!();
                continue;
            }
        }

        let duration = scorekeeper.lock().unwrap().add_result(correct);

        if correct {
            println!("  Correct! ({:.2} sec.)", duration.as_secs_f32());
        } else {
            println!("  Incorrect! (Expected {}.)", note + interval);
        }
    }
}
