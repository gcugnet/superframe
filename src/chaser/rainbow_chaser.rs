//! A static parameter of the lights will be name below a "sequence".
//! The parade of multiple sequences will be named below as a "chaser".
//! The transition parameters between two sequences will be named below as an "effect".
//! The chaser is composed by sequences and effects.

use core::marker::PhantomData;

use crate::sequence::{Rainbow, Unicolor};
use smart_leds::hsv::Hsv;
use smart_leds::RGB8;

/// A struct which defines the chaser.
pub struct RainbowChaser<S: OneParameterSequence> {
    first_color: Hsv,
    led_number: usize,
    step_number: usize,
    step: usize,
    _sequence: PhantomData<S>,
}

pub trait OneParameterSequence {
    fn new_sequence(color: Hsv, led_number: usize) -> Self;
}

impl OneParameterSequence for Unicolor<Hsv> {
    fn new_sequence(color: Hsv, led_number: usize) -> Self {
        Unicolor::new(color, led_number)
    }
}

impl OneParameterSequence for Rainbow {
    fn new_sequence(color: Hsv, led_number: usize) -> Self {
        Rainbow::new(color, led_number as u8)
    }
}

impl<S: OneParameterSequence> Iterator for RainbowChaser<S> {
    type Item = S;

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
        Some(S::new_sequence(color, self.led_number))
    }
}

impl<S: OneParameterSequence> RainbowChaser<S> {
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
            _sequence: PhantomData,
        }
    }
}
