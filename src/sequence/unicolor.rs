use smart_leds::RGB8;

use super::{OneParameterSequence, Sequence};

/// A sequence in which all LEDs have the same color.
pub struct Unicolor<Color> {
    /// The color for all LEDs.
    color: Color,
    /// The number of LEDs.
    number: usize,
    /// The counter.
    counter: usize,
}

impl<Color: Copy + Into<RGB8>> Sequence for Unicolor<Color> {}

impl<Color: Copy + Into<RGB8>> OneParameterSequence<Color> for Unicolor<Color> {
    fn new(color: Color, number: usize) -> Self {
        Self {
            color,
            number,
            counter: 0,
        }
    }
}

impl<Color: Copy + Into<RGB8>> Iterator for Unicolor<Color> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.number {
            self.counter += 1;
            Some(self.color.into())
        } else {
            None
        }
    }
}
