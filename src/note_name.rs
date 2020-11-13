use std::fmt;

#[derive(fmt::Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum NoteName {
    A = 0,
    B = 2,
    C = 3,
    D = 5,
    E = 7,
    F = 8,
    G = 10,
}

#[allow(dead_code)]
impl NoteName {
    pub const fn pitch(self) -> u8 {
        self as u8
    }

    pub const fn step_up(self) -> Self {
        use NoteName::*;
        match self {
            A => B,
            B => C,
            C => D,
            D => E,
            E => F,
            F => G,
            G => A,
        }
    }

    pub const fn step_down(self) -> Self {
        use NoteName::*;
        match self {
            A => G,
            B => A,
            C => B,
            D => C,
            E => D,
            F => E,
            G => F,
        }
    }
}

impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
