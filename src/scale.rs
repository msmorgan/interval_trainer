use crate::interval::canonical::CanonicalInterval;
use crate::note::Note;

pub mod modal;
pub mod mode;

#[derive(Debug, Clone)]
pub struct Scale {
    pub name: String,
    pub intervals: Vec<CanonicalInterval>,
}

impl Scale {
    pub fn from_intervals(name: impl AsRef<str>, intervals: impl AsRef<[u8]>) -> Self {
        Scale {
            name: name.as_ref().to_string(),
            intervals: intervals
                .as_ref()
                .iter()
                .map(|n| CanonicalInterval::from(*n))
                .collect(),
        }
    }

    pub fn shift(&self, steps: usize) -> Self {
        let steps = steps % self.intervals.len();
        Scale {
            // FIXME: Shifted scale names.
            name: format!("{}(+{})", &self.name, steps),
            intervals: self
                .intervals
                .iter()
                .cloned()
                .skip(steps)
                .chain(self.intervals.iter().cloned().take(steps))
                .collect(),
        }
    }

    pub fn spell(&self, root: Note) -> Vec<Note> {
        let mut result = vec![root];

        let mut note = root;

        for interval in self.intervals.iter() {
            let prev_note = note;
            note = note + *interval;
            if note.note_name() == prev_note.note_name() {
                note = note.enharmonic();
            }
            result.push(note);
        }

        result.pop().unwrap(); // Remove octave.

        result
    }
}
