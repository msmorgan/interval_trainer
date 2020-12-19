use std::fmt;

#[derive(fmt::Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i8)]
#[allow(dead_code)]
pub enum Accidental {
    DoubleFlat = -2,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    pub const fn interval(self) -> i8 {
        self as i8
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Accidental::DoubleFlat => "bb",
                Accidental::Flat => "b",
                Accidental::Natural => "",
                Accidental::Sharp => "#",
                Accidental::DoubleSharp => "##",
            }
        )
    }
}
