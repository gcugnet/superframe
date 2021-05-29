use smart_leds::{hsv::Hsv, RGB8};

use super::{OneParameterSequence, Sequence};

/// A sequence in which the LEDs draw a rainbow.
pub struct Rainbow {
    /// The first color of the rainbow.
    first_color: Hsv,
    /// The number of LEDs.
    number: usize,
    /// The counter.
    counter: usize,
}

impl Sequence for Rainbow {}

impl<Color: Into<Hsv>> OneParameterSequence<Color> for Rainbow {
    fn new(first_color: Color, number: usize) -> Self {
        Self {
            first_color: first_color.into(),
            number,
            counter: 0,
        }
    }
}

impl Iterator for Rainbow {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.number {
            let color = Hsv {
                hue: self.first_color.hue
                    + (self.counter * (255 / self.number)) as u8,
                ..self.first_color // sat: self.first_color.sat, val: self.first_color.val
            };
            self.counter += 1;
            Some(color.into())
        } else {
            None
        }
    }
}
