use smart_leds::RGB8;

use super::Sequence;

/// A sequence in which the LEDs draw a gradient.
pub struct Gradient {
    /// The departure color of the gradient.
    start_color: RGB8,
    /// The arrival color of the gradient.
    end_color: RGB8,
    /// The number of LEDs.
    number: u8,
    /// The counter.
    counter: u8,
}

impl Sequence for Gradient {}

impl Iterator for Gradient {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.number {
            let color = RGB8 {
                r: gradient_step(
                    self.start_color.r,
                    self.end_color.r,
                    self.number,
                    self.counter,
                ),
                g: gradient_step(
                    self.start_color.g,
                    self.end_color.g,
                    self.number,
                    self.counter,
                ),
                b: gradient_step(
                    self.start_color.b,
                    self.end_color.b,
                    self.number,
                    self.counter,
                ),
            };
            self.counter += 1;
            Some(color)
        } else {
            None
        }
    }
}

impl Gradient {
    /// Create a new gradient sequence.
    pub fn new(
        start_color: impl Into<RGB8>,
        end_color: impl Into<RGB8>,
        number: u8,
    ) -> Self {
        Self {
            start_color: start_color.into(),
            end_color: end_color.into(),
            number,
            counter: 0,
        }
    }
}

fn gradient_step(start: u8, end: u8, led_number: u8, step: u8) -> u8 {
    let start_i16 = start as i16;
    let end_i16 = end as i16;
    let step_i16 = step as i16;
    let led_number = led_number as i16;

    (start_i16 + (step_i16 * (end_i16 - start_i16)) / (led_number - 1)) as u8
}
