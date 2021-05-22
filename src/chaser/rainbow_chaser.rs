//! A static parameter of the lights will be name below a "sequence".
//! The parade of multiple sequences will be named below as a "chaser".
//! The transition parameters between two sequences will be named below as an "effect".
//! The chaser is composed by sequences and effects.

use crate::sequence::{Rainbow, Unicolor};
use smart_leds::hsv::Hsv;
use smart_leds::RGB8;

/// A struct which defines the chaser.
pub struct RainbowChaser {
    first_color: Hsv,
    led_number: usize,
    step_number: usize,
    step: usize,
}

impl Iterator for RainbowChaser {
    type Item = Rainbow;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == self.step_number {
            self.step = 0;
        }

        let color = Hsv {
            hue: self.first_color.hue
                + ((self.step * 255) / self.step_number) as u8,
            ..self.first_color // sat: self.first_color.sat, val: self.first_color.val
        };
        self.step += 1;
        Some(Rainbow::new(color, self.led_number as u8))
    }
}

impl RainbowChaser {
    pub fn new(
        first_color: impl Into<Hsv>,
        led_number: usize,
        step_number: usize,
    ) -> Self {
        Self {
            first_color: first_color.into(),
            led_number,
            step_number,
            step: 0,
        }
    }
}
