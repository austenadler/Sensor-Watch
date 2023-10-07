use crate::sys;

pub enum BuzzerNote {
    /// < 55.00 Hz
    A1,
    /// < 58.27 Hz
    A1SharpB1Flat,
    /// < 61.74 Hz
    B1,
    /// < 65.41 Hz
    C2,
    /// < 69.30 Hz
    C2SharpD2Flat,
    /// < 73.42 Hz
    D2,
    /// < 77.78 Hz
    D2SharpE2Flat,
    /// < 82.41 Hz
    E2,
    /// < 87.31 Hz
    F2,
    /// < 92.50 Hz
    F2SharpG2Flat,
    /// < 98.00 Hz
    G2,
    /// < 103.83 Hz
    G2SharpA2Flat,
    /// < 110.00 Hz
    A2,
    /// < 116.54 Hz
    A2SharpB2Flat,
    /// < 123.47 Hz
    B2,
    /// < 130.81 Hz
    C3,
    /// < 138.59 Hz
    C3SharpD3Flat,
    /// < 146.83 Hz
    D3,
    /// < 155.56 Hz
    D3SharpE3Flat,
    /// < 164.81 Hz
    E3,
    /// < 174.61 Hz
    F3,
    /// < 185.00 Hz
    F3SharpG3Flat,
    /// < 196.00 Hz
    G3,
    /// < 207.65 Hz
    G3SharpA3Flat,
    /// < 220.00 Hz
    A3,
    /// < 233.08 Hz
    A3SharpB3Flat,
    /// < 246.94 Hz
    B3,
    /// < 261.63 Hz
    C4,
    /// < 277.18 Hz
    C4SharpD4Flat,
    /// < 293.66 Hz
    D4,
    /// < 311.13 Hz
    D4SharpE4Flat,
    /// < 329.63 Hz
    E4,
    /// < 349.23 Hz
    F4,
    /// < 369.99 Hz
    F4SharpG4Flat,
    /// < 392.00 Hz
    G4,
    /// < 415.30 Hz
    G4SharpA4Flat,
    /// < 440.00 Hz
    A4,
    /// < 466.16 Hz
    A4SharpB4Flat,
    /// < 493.88 Hz
    B4,
    /// < 523.25 Hz
    C5,
    /// < 554.37 Hz
    C5SharpD5Flat,
    /// < 587.33 Hz
    D5,
    /// < 622.25 Hz
    D5SharpE5Flat,
    /// < 659.25 Hz
    E5,
    /// < 698.46 Hz
    F5,
    /// < 739.99 Hz
    F5SharpG5Flat,
    /// < 783.99 Hz
    G5,
    /// < 830.61 Hz
    G5SharpA5Flat,
    /// < 880.00 Hz
    A5,
    /// < 932.33 Hz
    A5SharpB5Flat,
    /// < 987.77 Hz
    B5,
    /// < 1046.50 Hz
    C6,
    /// < 1108.73 Hz
    C6SharpD6Flat,
    /// < 1174.66 Hz
    D6,
    /// < 1244.51 Hz
    D6SharpE6Flat,
    /// < 1318.51 Hz
    E6,
    /// < 1396.91 Hz
    F6,
    /// < 1479.98 Hz
    F6SharpG6Flat,
    /// < 1567.98 Hz
    G6,
    /// < 1661.22 Hz
    G6SharpA6Flat,
    /// < 1760.00 Hz
    A6,
    /// < 1864.66 Hz
    A6SharpB6Flat,
    /// < 1975.53 Hz
    B6,
    /// < 2093.00 Hz
    C7,
    /// < 2217.46 Hz
    C7SharpD7Flat,
    /// < 2349.32 Hz
    D7,
    /// < 2489.02 Hz
    D7SharpE7Flat,
    /// < 2637.02 Hz
    E7,
    /// < 2793.83 Hz
    F7,
    /// < 2959.96 Hz
    F7SharpG7Flat,
    /// < 3135.96 Hz
    G7,
    /// < 3322.44 Hz
    G7SharpA7Flat,
    /// < 3520.00 Hz
    A7,
    /// < 3729.31 Hz
    A7SharpB7Flat,
    /// < 3951.07 Hz
    B7,
    /// < 4186.01 Hz
    C8,
    /// < 4434.92 Hz
    C8SharpD8Flat,
    /// < 4698.63 Hz
    D8,
    /// < 4978.03 Hz
    D8SharpE8Flat,
    /// < 5274.04 Hz
    E8,
    /// < 5587.65 Hz
    F8,
    /// < 5919.91 Hz
    F8SharpG8Flat,
    /// < 6271.93 Hz
    G8,
    /// < 6644.88 Hz
    G8SharpA8Flat,
    /// < 7040.00 Hz
    A8,
    /// < 7458.62 Hz
    A8SharpB8Flat,
    /// < 7902.13 Hz
    B8,
    /// < no sound
    Rest,
}

impl BuzzerNote {
    const fn to_i8(&self) -> i8 {
        match self {
            BuzzerNote::A1 => 0,
            BuzzerNote::A1SharpB1Flat => 1,
            BuzzerNote::B1 => 2,
            BuzzerNote::C2 => 3,
            BuzzerNote::C2SharpD2Flat => 4,
            BuzzerNote::D2 => 5,
            BuzzerNote::D2SharpE2Flat => 6,
            BuzzerNote::E2 => 7,
            BuzzerNote::F2 => 8,
            BuzzerNote::F2SharpG2Flat => 9,
            BuzzerNote::G2 => 10,
            BuzzerNote::G2SharpA2Flat => 11,
            BuzzerNote::A2 => 12,
            BuzzerNote::A2SharpB2Flat => 13,
            BuzzerNote::B2 => 14,
            BuzzerNote::C3 => 15,
            BuzzerNote::C3SharpD3Flat => 16,
            BuzzerNote::D3 => 17,
            BuzzerNote::D3SharpE3Flat => 18,
            BuzzerNote::E3 => 19,
            BuzzerNote::F3 => 20,
            BuzzerNote::F3SharpG3Flat => 21,
            BuzzerNote::G3 => 22,
            BuzzerNote::G3SharpA3Flat => 23,
            BuzzerNote::A3 => 24,
            BuzzerNote::A3SharpB3Flat => 25,
            BuzzerNote::B3 => 26,
            BuzzerNote::C4 => 27,
            BuzzerNote::C4SharpD4Flat => 28,
            BuzzerNote::D4 => 29,
            BuzzerNote::D4SharpE4Flat => 30,
            BuzzerNote::E4 => 31,
            BuzzerNote::F4 => 32,
            BuzzerNote::F4SharpG4Flat => 33,
            BuzzerNote::G4 => 34,
            BuzzerNote::G4SharpA4Flat => 35,
            BuzzerNote::A4 => 36,
            BuzzerNote::A4SharpB4Flat => 37,
            BuzzerNote::B4 => 38,
            BuzzerNote::C5 => 39,
            BuzzerNote::C5SharpD5Flat => 40,
            BuzzerNote::D5 => 41,
            BuzzerNote::D5SharpE5Flat => 42,
            BuzzerNote::E5 => 43,
            BuzzerNote::F5 => 44,
            BuzzerNote::F5SharpG5Flat => 45,
            BuzzerNote::G5 => 46,
            BuzzerNote::G5SharpA5Flat => 47,
            BuzzerNote::A5 => 48,
            BuzzerNote::A5SharpB5Flat => 49,
            BuzzerNote::B5 => 50,
            BuzzerNote::C6 => 51,
            BuzzerNote::C6SharpD6Flat => 52,
            BuzzerNote::D6 => 53,
            BuzzerNote::D6SharpE6Flat => 54,
            BuzzerNote::E6 => 55,
            BuzzerNote::F6 => 56,
            BuzzerNote::F6SharpG6Flat => 57,
            BuzzerNote::G6 => 58,
            BuzzerNote::G6SharpA6Flat => 59,
            BuzzerNote::A6 => 60,
            BuzzerNote::A6SharpB6Flat => 61,
            BuzzerNote::B6 => 62,
            BuzzerNote::C7 => 63,
            BuzzerNote::C7SharpD7Flat => 64,
            BuzzerNote::D7 => 65,
            BuzzerNote::D7SharpE7Flat => 66,
            BuzzerNote::E7 => 67,
            BuzzerNote::F7 => 68,
            BuzzerNote::F7SharpG7Flat => 69,
            BuzzerNote::G7 => 70,
            BuzzerNote::G7SharpA7Flat => 71,
            BuzzerNote::A7 => 72,
            BuzzerNote::A7SharpB7Flat => 73,
            BuzzerNote::B7 => 74,
            BuzzerNote::C8 => 75,
            BuzzerNote::C8SharpD8Flat => 76,
            BuzzerNote::D8 => 77,
            BuzzerNote::D8SharpE8Flat => 78,
            BuzzerNote::E8 => 79,
            BuzzerNote::F8 => 80,
            BuzzerNote::F8SharpG8Flat => 81,
            BuzzerNote::G8 => 82,
            BuzzerNote::G8SharpA8Flat => 83,
            BuzzerNote::A8 => 84,
            BuzzerNote::A8SharpB8Flat => 85,
            BuzzerNote::B8 => 86,
            BuzzerNote::Rest => 87,
        }
    }
}

impl const From<&BuzzerNote> for sys::BuzzerNote {
    fn from(value: &BuzzerNote) -> Self {
        sys::BuzzerNote(value.to_i8() as u32)
    }
}

impl const From<&sys::BuzzerNote> for BuzzerNote {
    fn from(value: &sys::BuzzerNote) -> Self {
        match value {
            sys::BuzzerNote(0) => Self::A1,
            sys::BuzzerNote(1) => Self::A1SharpB1Flat,
            sys::BuzzerNote(2) => Self::B1,
            sys::BuzzerNote(3) => Self::C2,
            sys::BuzzerNote(4) => Self::C2SharpD2Flat,
            sys::BuzzerNote(5) => Self::D2,
            sys::BuzzerNote(6) => Self::D2SharpE2Flat,
            sys::BuzzerNote(7) => Self::E2,
            sys::BuzzerNote(8) => Self::F2,
            sys::BuzzerNote(9) => Self::F2SharpG2Flat,
            sys::BuzzerNote(10) => Self::G2,
            sys::BuzzerNote(11) => Self::G2SharpA2Flat,
            sys::BuzzerNote(12) => Self::A2,
            sys::BuzzerNote(13) => Self::A2SharpB2Flat,
            sys::BuzzerNote(14) => Self::B2,
            sys::BuzzerNote(15) => Self::C3,
            sys::BuzzerNote(16) => Self::C3SharpD3Flat,
            sys::BuzzerNote(17) => Self::D3,
            sys::BuzzerNote(18) => Self::D3SharpE3Flat,
            sys::BuzzerNote(19) => Self::E3,
            sys::BuzzerNote(20) => Self::F3,
            sys::BuzzerNote(21) => Self::F3SharpG3Flat,
            sys::BuzzerNote(22) => Self::G3,
            sys::BuzzerNote(23) => Self::G3SharpA3Flat,
            sys::BuzzerNote(24) => Self::A3,
            sys::BuzzerNote(25) => Self::A3SharpB3Flat,
            sys::BuzzerNote(26) => Self::B3,
            sys::BuzzerNote(27) => Self::C4,
            sys::BuzzerNote(28) => Self::C4SharpD4Flat,
            sys::BuzzerNote(29) => Self::D4,
            sys::BuzzerNote(30) => Self::D4SharpE4Flat,
            sys::BuzzerNote(31) => Self::E4,
            sys::BuzzerNote(32) => Self::F4,
            sys::BuzzerNote(33) => Self::F4SharpG4Flat,
            sys::BuzzerNote(34) => Self::G4,
            sys::BuzzerNote(35) => Self::G4SharpA4Flat,
            sys::BuzzerNote(36) => Self::A4,
            sys::BuzzerNote(37) => Self::A4SharpB4Flat,
            sys::BuzzerNote(38) => Self::B4,
            sys::BuzzerNote(39) => Self::C5,
            sys::BuzzerNote(40) => Self::C5SharpD5Flat,
            sys::BuzzerNote(41) => Self::D5,
            sys::BuzzerNote(42) => Self::D5SharpE5Flat,
            sys::BuzzerNote(43) => Self::E5,
            sys::BuzzerNote(44) => Self::F5,
            sys::BuzzerNote(45) => Self::F5SharpG5Flat,
            sys::BuzzerNote(46) => Self::G5,
            sys::BuzzerNote(47) => Self::G5SharpA5Flat,
            sys::BuzzerNote(48) => Self::A5,
            sys::BuzzerNote(49) => Self::A5SharpB5Flat,
            sys::BuzzerNote(50) => Self::B5,
            sys::BuzzerNote(51) => Self::C6,
            sys::BuzzerNote(52) => Self::C6SharpD6Flat,
            sys::BuzzerNote(53) => Self::D6,
            sys::BuzzerNote(54) => Self::D6SharpE6Flat,
            sys::BuzzerNote(55) => Self::E6,
            sys::BuzzerNote(56) => Self::F6,
            sys::BuzzerNote(57) => Self::F6SharpG6Flat,
            sys::BuzzerNote(58) => Self::G6,
            sys::BuzzerNote(59) => Self::G6SharpA6Flat,
            sys::BuzzerNote(60) => Self::A6,
            sys::BuzzerNote(61) => Self::A6SharpB6Flat,
            sys::BuzzerNote(62) => Self::B6,
            sys::BuzzerNote(63) => Self::C7,
            sys::BuzzerNote(64) => Self::C7SharpD7Flat,
            sys::BuzzerNote(65) => Self::D7,
            sys::BuzzerNote(66) => Self::D7SharpE7Flat,
            sys::BuzzerNote(67) => Self::E7,
            sys::BuzzerNote(68) => Self::F7,
            sys::BuzzerNote(69) => Self::F7SharpG7Flat,
            sys::BuzzerNote(70) => Self::G7,
            sys::BuzzerNote(71) => Self::G7SharpA7Flat,
            sys::BuzzerNote(72) => Self::A7,
            sys::BuzzerNote(73) => Self::A7SharpB7Flat,
            sys::BuzzerNote(74) => Self::B7,
            sys::BuzzerNote(75) => Self::C8,
            sys::BuzzerNote(76) => Self::C8SharpD8Flat,
            sys::BuzzerNote(77) => Self::D8,
            sys::BuzzerNote(78) => Self::D8SharpE8Flat,
            sys::BuzzerNote(79) => Self::E8,
            sys::BuzzerNote(80) => Self::F8,
            sys::BuzzerNote(81) => Self::F8SharpG8Flat,
            sys::BuzzerNote(82) => Self::G8,
            sys::BuzzerNote(83) => Self::G8SharpA8Flat,
            sys::BuzzerNote(84) => Self::A8,
            sys::BuzzerNote(85) => Self::A8SharpB8Flat,
            sys::BuzzerNote(86) => Self::B8,
            sys::BuzzerNote(87) | sys::BuzzerNote(_) => Self::Rest,
        }
    }
}

pub enum Segment {
    Note(BuzzerNote, u8),
    Repeat(u8, u8),
}

impl Segment {
    const fn to_i8s(&self) -> [i8; 2] {
        match self {
            Segment::Note(note, length) => [note.to_i8(), *length as i8],
            Segment::Repeat(backwards, repetitions) => [-(*backwards as i8), *repetitions as i8],
        }
    }
}

pub const fn construct_note<const N: usize>(segments: [Segment; N]) -> [i8; N * 2 + 1] {
    let mut ret = [0; N * 2 + 1];
    let mut i = 0;
    while i < N {
        let [x, y] = segments[i].to_i8s();
        ret[i * 2] = x;
        ret[i * 2 + 1] = y;
        i += 1;
    }
    // for segment in segments {
    //     let (x, y) = segment.to_i8s();
    //     ret[]
    // }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_note() {
        assert_eq!(
            construct_note([
                Segment::Note(BuzzerNote::A1SharpB1Flat, 3),
                Segment::Repeat(1, 3)
            ]),
            [BuzzerNote::A1SharpB1Flat.to_i8(), 3, -1, 3, 0]
        );
    }
}
