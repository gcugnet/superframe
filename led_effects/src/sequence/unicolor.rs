use smart_leds::RGB8;

use super::{OneParameterSequence, Sequence};

/// A sequence in which all LEDs have the same color.
pub struct Unicolor<Color, const N: usize> {
    /// The color for all LEDs.
    color: Color,
    /// The counter.
    counter: usize,
}

impl<Color: Copy + Into<RGB8>, const N: usize> Sequence<N>
    for Unicolor<Color, N>
{
}

impl<Color: Copy + Into<RGB8>, const N: usize> OneParameterSequence<Color, N>
    for Unicolor<Color, N>
{
    fn new(color: Color) -> Self {
        Self { color, counter: 0 }
    }
}

impl<Color: Copy + Into<RGB8>, const N: usize> Iterator for Unicolor<Color, N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            self.counter += 1;
            Some(self.color.into())
        } else {
            None
        }
    }
}
