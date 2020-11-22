use std::{fmt, ops, str::FromStr};

use crate::{accidental::Accidental, interval::canonical::CanonicalInterval, note_name::NoteName};

#[derive(fmt::Debug, Copy, Clone)]
pub struct Note(pub NoteName, pub Accidental);

impl Note {
    pub const fn from_pitch(pitch: u8) -> Option<Self> {
        use Accidental::*;
        use NoteName::*;
        match pitch {
            0 => Some(Note(A, Natural)),
            1 => Some(Note(A, Sharp)),
            2 => Some(Note(B, Natural)),
            3 => Some(Note(C, Natural)),
            4 => Some(Note(C, Sharp)),
            5 => Some(Note(D, Natural)),
            6 => Some(Note(D, Sharp)),
            7 => Some(Note(E, Natural)),
            8 => Some(Note(F, Natural)),
            9 => Some(Note(F, Sharp)),
            10 => Some(Note(G, Natural)),
            11 => Some(Note(G, Sharp)),
            _ => None,
        }
    }

    pub const fn note_name(self) -> NoteName {
        self.0
    }

    pub const fn accidental(self) -> Accidental {
        self.1
    }

    pub const fn pitch(self) -> u8 {
        (12 + self.note_name().pitch() as i8 + self.accidental().interval()) as u8 % 12
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.pitch() == other.pitch()
    }
}

impl ops::Add<i8> for Note {
    type Output = Note;

    fn add(self, interval: i8) -> Self::Output {
        let interval = (interval % 12) + 12;
        let pitch = (self.pitch() + interval as u8) % 12;
        Self::from_pitch(pitch).unwrap()
    }
}

impl ops::Sub<i8> for Note {
    type Output = Note;

    fn sub(self, interval: i8) -> Self::Output {
        self + -interval
    }
}

impl ops::Add<CanonicalInterval> for Note {
    type Output = Note;

    fn add(self, interval: CanonicalInterval) -> Self::Output {
        self + interval.size() as i8
    }
}

impl ops::Sub<CanonicalInterval> for Note {
    type Output = Note;

    fn sub(self, interval: CanonicalInterval) -> Self::Output {
        self + -(interval.size() as i8)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(fmt::Debug, Clone, Eq, PartialEq)]
pub struct UnrecognizedNote(String);

impl FromStr for Note {
    type Err = UnrecognizedNote;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(UnrecognizedNote(s.to_string()))
        } else {
            use Accidental::*;
            use NoteName::*;

            let note_name = match s.chars().next().unwrap().to_ascii_uppercase() {
                'A' => A,
                'B' => B,
                'C' => C,
                'D' => D,
                'E' => E,
                'F' => F,
                'G' => G,
                _ => return Err(UnrecognizedNote(s.to_string())),
            };

            let accidental = match &s[1..] {
                "bb" => DoubleFlat,
                "b" => Flat,
                "" => Natural,
                "#" => Sharp,
                "##" => DoubleSharp,
                _ => return Err(UnrecognizedNote(s.to_string())),
            };

            Ok(Note(note_name, accidental))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_equality() {
        use self::{Accidental::*, NoteName::*};
        assert_eq!(Note(D, Sharp), Note(E, Flat));
    }
}
