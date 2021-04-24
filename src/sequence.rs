//! A collection of LED sequences on top of `smart_leds`.

use smart_leds::RGB8;

/// A sequence in which all LEDs have the same color.
pub struct Unicolor {
    /// The color for all LEDs.
    color: RGB8,
    /// The number of LEDs.
    number: usize,
    /// The counter.
    counter: usize,
}

impl Iterator for Unicolor {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.number {
            self.counter += 1;
            Some(self.color)
        } else {
            None
        }
    }
}

impl Unicolor {
    /// Create a new unicolor sequence.
    pub fn new(color: RGB8, number: usize) -> Self {
        Self {
            color,
            number,
            counter: 0,
        }
    }
}
