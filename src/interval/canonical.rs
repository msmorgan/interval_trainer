use std::fmt;

#[derive(fmt::Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum CanonicalInterval {
    Unison = 0,
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
    Octave,
    MinorNinth,
    MajorNinth,
    MinorTenth,
    MajorTenth,
    PerfectEleventh,
    PerfectTwelfth = 19,
    MinorThirteenth,
    MajorThirteenth,
}

impl CanonicalInterval {
    pub fn size(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for CanonicalInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CanonicalInterval::*;
        write!(
            f,
            "{}",
            match self {
                Unison => "Unison",
                MinorSecond => "Minor 2",
                MajorSecond => "Major 2",
                MinorThird => "Minor 3",
                MajorThird => "Major 3",
                PerfectFourth => "Perfect 4",
                Tritone => "Tritone",
                PerfectFifth => "Perfect 5",
                MinorSixth => "Minor 6",
                MajorSixth => "Major 6",
                MinorSeventh => "Minor 7",
                MajorSeventh => "Major 7",
                Octave => "Octave",
                MinorNinth => "Minor 9",
                MajorNinth => "Major 9",
                MinorTenth => "Minor 10",
                MajorTenth => "Major 10",
                PerfectEleventh => "Perfect 11",
                PerfectTwelfth => "Perfect 12",
                MinorThirteenth => "Minor 13",
                MajorThirteenth => "Major 13",
            }
        )
    }
}
