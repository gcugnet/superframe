//! A collection of LED sequences on top of `smart_leds`.

mod gradient;
mod rainbow;
mod unicolor;

pub use gradient::Gradient;
pub use rainbow::Rainbow;
pub use unicolor::Unicolor;

use smart_leds::{hsv::Hsv, RGB8};

/// A LED sequence.
pub trait Sequence<const N: usize>: Iterator {}

/// A LED sequence with one parameter.
pub trait OneParameterSequence<Color, const N: usize>: Sequence<N> {
    fn new(color: Color) -> Self;
}

/// Container enum for one-parameter sequences.
pub enum OneParameterSequenceEnum<const N: usize> {
    Unicolor(Unicolor<Hsv, N>),
    Rainbow(Rainbow<N>),
}

impl<const N: usize> From<Unicolor<Hsv, N>> for OneParameterSequenceEnum<N> {
    fn from(sequence: Unicolor<Hsv, N>) -> Self {
        OneParameterSequenceEnum::Unicolor(sequence)
    }
}

impl<const N: usize> From<Rainbow<N>> for OneParameterSequenceEnum<N> {
    fn from(sequence: Rainbow<N>) -> Self {
        OneParameterSequenceEnum::Rainbow(sequence)
    }
}

impl<const N: usize> Sequence<N> for OneParameterSequenceEnum<N> {}

impl<const N: usize> Iterator for OneParameterSequenceEnum<N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OneParameterSequenceEnum::Unicolor(sequence) => sequence.next(),
            OneParameterSequenceEnum::Rainbow(sequence) => sequence.next(),
        }
    }
}
