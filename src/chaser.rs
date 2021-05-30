//! A collection of LED chasers on top of `smart_leds`.

mod rainbow_chaser;

pub use rainbow_chaser::RainbowChaser;

use crate::sequence::{OneParameterSequenceEnum, Rainbow, Unicolor};
use smart_leds::hsv::Hsv;

/// A LED chaser.
pub trait Chaser: Iterator {
    fn set_step_number(&mut self, step_number: usize);
}

/// A LED chaser with one parameter.
pub trait OneParameterChaser<Color>: Chaser {
    fn new(first_color: Color, led_number: usize, step_number: usize) -> Self;
}

/// Container enum for one-parameter chasers.
pub enum OneParameterChaserEnum {
    Unicolor(RainbowChaser<Unicolor<Hsv>>),
    Rainbow(RainbowChaser<Rainbow>),
}

impl From<RainbowChaser<Unicolor<Hsv>>> for OneParameterChaserEnum {
    fn from(chaser: RainbowChaser<Unicolor<Hsv>>) -> Self {
        OneParameterChaserEnum::Unicolor(chaser)
    }
}

impl From<RainbowChaser<Rainbow>> for OneParameterChaserEnum {
    fn from(chaser: RainbowChaser<Rainbow>) -> Self {
        OneParameterChaserEnum::Rainbow(chaser)
    }
}

impl Chaser for OneParameterChaserEnum {
    fn set_step_number(&mut self, step_number: usize) {
        match self {
            OneParameterChaserEnum::Unicolor(chaser) => {
                chaser.set_step_number(step_number)
            }
            OneParameterChaserEnum::Rainbow(chaser) => {
                chaser.set_step_number(step_number)
            }
        }
    }
}

impl<Color: Into<Hsv>> OneParameterChaser<Color> for OneParameterChaserEnum {
    fn new(first_color: Color, led_number: usize, step_number: usize) -> Self {
        OneParameterChaserEnum::Unicolor(RainbowChaser::new(
            first_color,
            led_number,
            step_number,
        ))
    }
}

impl Iterator for OneParameterChaserEnum {
    type Item = OneParameterSequenceEnum;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OneParameterChaserEnum::Unicolor(chaser) => {
                chaser.next().map(Into::into)
            }
            OneParameterChaserEnum::Rainbow(chaser) => {
                chaser.next().map(Into::into)
            }
        }
    }
}
