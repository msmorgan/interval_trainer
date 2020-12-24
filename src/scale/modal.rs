use crate::{
    note::Note,
    scale::{mode::Mode, Scale},
};

pub struct ModalScale {
    pub scale: Scale,
    pub mode: Mode,
}

impl ModalScale {
    pub fn new(scale: Scale, mode: Mode) -> Self {
        ModalScale { scale, mode }
    }

    pub fn spell(&self, root: Note) -> Vec<Note> {
        self.scale.shift(self.mode as u8 as usize).spell(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell() {
        use crate::{accidental::Accidental::*, note_name::NoteName::*};

        let minor_scale = ModalScale::new(
            Scale::from_intervals("Major", &[2, 2, 1, 2, 2, 2, 1]),
            Mode::Aeolian,
        );

        let notes = minor_scale.spell(Note(F, Sharp));
        let expected_notes = vec![
            Note(F, Sharp),
            Note(G, Sharp),
            Note(A, Natural),
            Note(B, Natural),
            Note(C, Sharp),
            Note(D, Natural),
            Note(E, Natural),
        ];

        assert_eq!(notes.len(), expected_notes.len());

        for i in 0..notes.len() {
            assert_eq!(
                (notes[i].note_name(), notes[i].accidental()),
                (
                    expected_notes[i].note_name(),
                    expected_notes[i].accidental()
                )
            );
        }
    }
}
