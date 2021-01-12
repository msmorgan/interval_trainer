use std::fmt;

use crate::interval::canonical::CanonicalInterval;
use crate::note::Note;

#[derive(fmt::Debug, Clone, PartialEq)]
pub struct ChordQuality {
    name: String,
    intervals: Vec<CanonicalInterval>,
}

#[allow(dead_code)]
impl ChordQuality {
    pub fn new(name: impl ToString, intervals: impl AsRef<[CanonicalInterval]>) -> Self {
        ChordQuality {
            name: name.to_string(),
            intervals: Vec::from(intervals.as_ref()),
        }
    }

    pub fn from_intervals(name: impl ToString, intervals: impl AsRef<[u8]>) -> Self {
        ChordQuality {
            name: name.to_string(),
            intervals: intervals
                .as_ref()
                .iter()
                .cloned()
                .map(CanonicalInterval::from)
                .collect(),
        }
    }

    pub fn intervals(&self) -> &[CanonicalInterval] {
        &self.intervals
    }

    pub fn note_count(&self) -> usize {
        self.intervals.len() + 1
    }

    pub fn spell(&self, root: Note) -> Vec<Note> {
        let mut result = vec![root];
        let mut note = root;
        for interval in self.intervals.iter() {
            note = note + *interval;
            result.push(note);
        }
        result
    }
}

impl fmt::Display for ChordQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

macro_rules! define_chords {
    ($($name:ident = $label:literal [$($interval:expr),+],)*) => {
        $(
            pub static $name: ::once_cell::sync::Lazy<$crate::chord::quality::ChordQuality> =
                ::once_cell::sync::Lazy::new(|| {
                    $crate::chord::quality::ChordQuality::from_intervals($label, &[$($interval),+])
                });
        )*
    };
}

#[allow(dead_code)]
pub mod triads {
    define_chords! {
        MAJOR = "Maj" [4, 3],
        MINOR = "Min" [3, 4],
        DIMINISHED = "Dim" [3, 3],
        AUGMENTED = "Aug" [4, 4],
        SUSPENDED_2 = "Sus2" [2, 5],
        PHRYGIAN = "Phr" [1, 6],
        SUSPENDED_4 = "Sus4" [5, 2],
        LYDIAN = "Lyd" [6, 1],
    }
}

#[allow(dead_code)]
pub mod sevenths {
    define_chords! {
        DOMINANT = "Dom7" [4, 3, 3],
        MAJOR = "Maj7" [4, 3, 4],
        MINOR = "Min7" [3, 4, 3],
        DIMINISHED = "Dim7" [3, 3, 3],
        HALF_DIMINISHED = "Min7(b5)" [3, 3, 4],
        AUGMENTED_DOMINANT = "Dom7(#5)" [4, 4, 2],
        AUGMENTED_MAJOR = "Maj7(#5)" [4, 4, 3],
        DIMINISHED_MAJOR = "Dim(Maj7)" [3, 3, 7],
    }
}
