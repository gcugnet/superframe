use smart_leds::hsv::Hsv;
use smart_leds::RGB;

/// A sequence in which the LEDs draw a rainbow.
pub struct Rainbow {
    /// The first color of the rainbow.
    first_color: Hsv,
    /// The number of LEDs.
    number: u8,
    /// The counter.
    counter: u8,
}

impl Iterator for Rainbow {
    type Item = RGB<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.number {
            let color = Hsv {
                hue: self.first_color.hue + self.counter * (255 / self.number),
                ..self.first_color // sat: self.first_color.sat, val: self.first_color.val
            };
            self.counter += 1;
            Some(smart_leds::hsv::hsv2rgb(color))
        } else {
            None
        }
    }
}

impl Rainbow {
    /// Create a new rainbow sequence.
    pub fn new(first_color: impl Into<Hsv>, number: u8) -> Self {
        Self {
            first_color: first_color.into(),
            number,
            counter: 0,
        }
    }
}
