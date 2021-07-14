//! A collection of LED chasers on top of `smart_leds`.

mod rainbow_chaser;

pub use rainbow_chaser::RainbowChaser;

use crate::sequence::{OneParameterSequenceEnum, Rainbow, Unicolor};
use smart_leds::hsv::Hsv;

/// A LED chaser.
pub trait Chaser<const N: usize>: Iterator {
    fn set_step_number(&mut self, step_number: usize);
}

/// A LED chaser with one parameter.
pub trait OneParameterChaser<Color, const N: usize>: Chaser<N> {
    fn new(start_color: Color, step_number: usize) -> Self;
}

/// Container enum for one-parameter chasers.
pub enum OneParameterChaserEnum<const N: usize> {
    Unicolor(RainbowChaser<Unicolor<Hsv, N>, N>),
    Rainbow(RainbowChaser<Rainbow<N>, N>),
}

impl<const N: usize> From<RainbowChaser<Unicolor<Hsv, N>, N>>
    for OneParameterChaserEnum<N>
{
    fn from(chaser: RainbowChaser<Unicolor<Hsv, N>, N>) -> Self {
        OneParameterChaserEnum::Unicolor(chaser)
    }
}

impl<const N: usize> From<RainbowChaser<Rainbow<N>, N>>
    for OneParameterChaserEnum<N>
{
    fn from(chaser: RainbowChaser<Rainbow<N>, N>) -> Self {
        OneParameterChaserEnum::Rainbow(chaser)
    }
}

impl<const N: usize> Chaser<N> for OneParameterChaserEnum<N> {
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

impl<Color: Into<Hsv>, const N: usize> OneParameterChaser<Color, N>
    for OneParameterChaserEnum<N>
{
    fn new(start_color: Color, step_number: usize) -> Self {
        OneParameterChaserEnum::Unicolor(RainbowChaser::new(
            start_color,
            step_number,
        ))
    }
}

impl<const N: usize> Iterator for OneParameterChaserEnum<N> {
    type Item = OneParameterSequenceEnum<N>;

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
