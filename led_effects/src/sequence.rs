//! A collection of LED sequences on top of `smart_leds`.

mod color_path;
mod gradient;
mod rainbow;
mod unicolor;

pub use color_path::ColorPath;
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

/// A LED sequence with two parameters.
pub trait TwoParameterSequence<Color, const N: usize>: Sequence<N> {
    fn new(color1: Color, color2: Color) -> Self;
}

/// A LED sequence working on a color array.
pub trait ArraySequence<Color, const N: usize, const M: usize>:
    Sequence<N>
{
    fn new(colors: [Color; M]) -> Self;
}

/// Container enum for one-parameter sequences.
pub enum OneParameterSequenceEnum<const N: usize> {
    UnicolorRgb8(Unicolor<RGB8, N>),
    UnicolorHsv(Unicolor<Hsv, N>),
    Rainbow(Rainbow<N>),
}

impl<const N: usize> From<Unicolor<RGB8, N>> for OneParameterSequenceEnum<N> {
    fn from(sequence: Unicolor<RGB8, N>) -> Self {
        OneParameterSequenceEnum::UnicolorRgb8(sequence)
    }
}

impl<const N: usize> From<Unicolor<Hsv, N>> for OneParameterSequenceEnum<N> {
    fn from(sequence: Unicolor<Hsv, N>) -> Self {
        OneParameterSequenceEnum::UnicolorHsv(sequence)
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
            OneParameterSequenceEnum::UnicolorRgb8(sequence) => sequence.next(),
            OneParameterSequenceEnum::UnicolorHsv(sequence) => sequence.next(),
            OneParameterSequenceEnum::Rainbow(sequence) => sequence.next(),
        }
    }
}
