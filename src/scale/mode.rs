#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Mode {
    Ionian = 0,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        use Mode::*;
        match value {
            0 => Ionian,
            1 => Dorian,
            2 => Phrygian,
            3 => Lydian,
            4 => Mixolydian,
            5 => Aeolian,
            6 => Locrian,
            value if value >= 7 => Mode::from(value - 7),
            _ => unreachable!(),
        }
    }
}
