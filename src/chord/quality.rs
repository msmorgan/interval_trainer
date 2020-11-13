use std::fmt;

use crate::{
    interval::canonical::CanonicalInterval,
    note::Note,
};

#[derive(fmt::Debug, Clone, PartialEq)]
pub struct ChordQuality {
    name: String,
    intervals: Vec<CanonicalInterval>,
}

impl ChordQuality {
    pub fn new(name: &str, intervals: &[CanonicalInterval]) -> Self {
        ChordQuality {
            name: name.to_string(),
            intervals: Vec::from(intervals),
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

pub mod triads {
    use lazy_static::lazy_static;

    use super::{
        ChordQuality,
        CanonicalInterval::*
    };

    lazy_static! {
        static ref MAJOR: ChordQuality = ChordQuality::new("Major", &[MajorThird, MinorThird]);
    }
}