//! A static parameter of the lights will be name below a "sequence".
//! The parade of multiple sequences will be named below as a "chaser".
//! The transition parameters between two sequences will be named below as an "effect".
//! The chaser is composed by sequences and effects.

use core::marker::PhantomData;

use smart_leds::hsv::Hsv;

use super::{Chaser, OneParameterChaser};
use crate::sequence::OneParameterSequence;

/// A struct which defines the chaser.
pub struct RainbowChaser<S: OneParameterSequence<Hsv, N>, const N: usize> {
    first_color: Hsv,
    step_number: usize,
    step: usize,
    _sequence: PhantomData<S>,
}

impl<S: OneParameterSequence<Hsv, N>, const N: usize> Chaser<N>
    for RainbowChaser<S, N>
{
    fn set_step_number(&mut self, step_number: usize) {
        self.step = self.step * step_number / self.step_number;
        self.step_number = step_number;
    }
}

impl<Color: Into<Hsv>, S: OneParameterSequence<Hsv, N>, const N: usize>
    OneParameterChaser<Color, N> for RainbowChaser<S, N>
{
    fn new(first_color: Color, step_number: usize) -> Self {
        Self {
            first_color: first_color.into(),
            step_number,
            step: 0,
            _sequence: PhantomData,
        }
    }
}

impl<S: OneParameterSequence<Hsv, N>, const N: usize> Iterator
    for RainbowChaser<S, N>
{
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
        Some(S::new(color))
    }
}
