use crate::{chord::quality::ChordQuality, interval::canonical::CanonicalInterval, note::Note};

pub mod round;
pub mod scorekeeper;

const STANDARD_NOTES: [Note; 17] = {
    use crate::{accidental::Accidental::*, note_name::NoteName::*};

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
            Sharp => {
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
    use crate::interval::canonical::CanonicalInterval::*;
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
    static ref STANDARD_CHORD_QUALITIES: Vec<ChordQuality> = {
        use crate::chord::quality::{sevenths, triads};
        vec![
            triads::MAJOR.clone(),
            triads::MINOR.clone(),
            triads::DIMINISHED.clone(),
            triads::AUGMENTED.clone(),
            sevenths::MAJOR.clone(),
            sevenths::MINOR.clone(),
            sevenths::DIMINISHED.clone(),
            sevenths::HALF_DIMINISHED.clone(),
            sevenths::DOMINANT.clone(),
        ]
    };
}
