//! A collection of LED sequences on top of `smart_leds`.

mod gradient;
mod rainbow;
mod unicolor;

pub use gradient::Gradient;
pub use rainbow::Rainbow;
pub use unicolor::Unicolor;

use smart_leds::{hsv::Hsv, RGB8};

/// A LED sequence.
pub trait Sequence: Iterator {}

/// A LED sequence with one parameter.
pub trait OneParameterSequence<Color>: Sequence {
    fn new(color: Color, led_number: usize) -> Self;
}

/// Container enum for one-parameter sequences.
pub enum OneParameterSequenceEnum {
    Unicolor(Unicolor<Hsv>),
    Rainbow(Rainbow),
}

impl From<Unicolor<Hsv>> for OneParameterSequenceEnum {
    fn from(sequence: Unicolor<Hsv>) -> Self {
        OneParameterSequenceEnum::Unicolor(sequence)
    }
}

impl From<Rainbow> for OneParameterSequenceEnum {
    fn from(sequence: Rainbow) -> Self {
        OneParameterSequenceEnum::Rainbow(sequence)
    }
}

impl Sequence for OneParameterSequenceEnum {}

impl Iterator for OneParameterSequenceEnum {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OneParameterSequenceEnum::Unicolor(sequence) => sequence.next(),
            OneParameterSequenceEnum::Rainbow(sequence) => sequence.next(),
        }
    }
}
