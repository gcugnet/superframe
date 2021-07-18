use smart_leds::RGB8;

use super::{Chaser, TwoParameterChaser};
use crate::{
    sequence::{OneParameterSequence, Unicolor},
    time::TimeConfig,
};

/// A simple transition between two colors.
pub struct UnicolorTransition<const N: usize> {
    /// The start color of the transition.
    start_color: RGB8,
    /// The end color of the transition.
    end_color: RGB8,
    /// The number of steps to perform the transition.
    step_number: u32,
    /// The current step.
    step: u32,
}

impl<const N: usize> UnicolorTransition<N> {
    pub fn end_color(&self) -> RGB8 {
        self.end_color
    }
}

impl<const N: usize> Chaser<N> for UnicolorTransition<N> {
    // IDEA: Factorise with other implementations.
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        let step_number = time_config.transition_steps();
        self.step = self.step * step_number / self.step_number;
        self.step_number = step_number;
    }
}

impl<C: Into<RGB8>, const N: usize> TwoParameterChaser<C, N>
    for UnicolorTransition<N>
{
    fn new(start_color: C, end_color: C, time_config: &TimeConfig) -> Self {
        Self {
            start_color: start_color.into(),
            end_color: end_color.into(),
            step_number: time_config.transition_steps(),
            step: 0,
        }
    }
}

impl<const N: usize> Iterator for UnicolorTransition<N> {
    type Item = Unicolor<RGB8, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step < self.step_number {
            let color = RGB8 {
                r: transition_step(
                    self.start_color.r,
                    self.end_color.r,
                    self.step,
                    self.step_number,
                ),
                g: transition_step(
                    self.start_color.g,
                    self.end_color.g,
                    self.step,
                    self.step_number,
                ),
                b: transition_step(
                    self.start_color.b,
                    self.end_color.b,
                    self.step,
                    self.step_number,
                ),
            };

            self.step += 1;
            Some(Unicolor::new(color))
        } else {
            None
        }
    }
}

fn transition_step(start: u8, end: u8, step: u32, step_number: u32) -> u8 {
    let start_i32 = start as i32;
    let end_i32 = end as i32;
    let step_i32 = step as i32;
    let step_number_i32 = step_number as i32;

    (start_i32 + (step_i32 * (end_i32 - start_i32)) / (step_number_i32 - 1))
        as u8
}
