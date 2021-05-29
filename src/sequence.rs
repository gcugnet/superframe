//! A collection of LED sequences on top of `smart_leds`.

mod gradient;
mod rainbow;
mod unicolor;

pub use gradient::Gradient;
pub use rainbow::Rainbow;
pub use unicolor::Unicolor;

/// A LED sequence.
pub trait Sequence: Iterator {}

/// A LED sequence with one parameter.
pub trait OneParameterSequence<Color>: Sequence {
    fn new(color: Color, led_number: usize) -> Self;
}
