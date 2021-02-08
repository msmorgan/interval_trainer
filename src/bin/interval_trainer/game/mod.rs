use music_exercises::chord::quality::ChordQuality;
use music_exercises::interval::canonical::CanonicalInterval;
use music_exercises::note::Note;
use music_exercises::scale::Scale;
use once_cell::sync::Lazy;

pub mod mode;
pub mod round;
pub mod scorekeeper;

const STANDARD_NOTES: [Note; 17] = {
    use music_exercises::accidental::Accidental::*;
    use music_exercises::note_name::NoteName::*;

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
    use music_exercises::interval::canonical::CanonicalInterval::*;
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

static STANDARD_CHORD_QUALITIES: Lazy<Vec<ChordQuality>> = Lazy::new(|| {
    use music_exercises::chord::quality::{sevenths, triads};

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
});

static STANDARD_SCALES: Lazy<Vec<Scale>> = Lazy::new(|| {
    vec![
        Scale::from_intervals("Major", &[2, 2, 1, 2, 2, 2, 1]),
        Scale::from_intervals("Minor", &[2, 1, 2, 2, 1, 2, 2]),
        Scale::from_intervals("Harmonic Minor", &[2, 1, 2, 2, 1, 3, 1]),
    ]
});
