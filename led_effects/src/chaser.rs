//! A collection of LED chasers on top of `smart_leds`.

mod rainbow_chaser;

pub use rainbow_chaser::RainbowChaser;

use crate::{
    sequence::{OneParameterSequenceEnum, Rainbow, Unicolor},
    time::TimeConfig,
};

use smart_leds::hsv::Hsv;

/// A LED chaser.
pub trait Chaser<const N: usize>: Iterator {
    fn set_time_config(&mut self, time_config: &TimeConfig);
}

/// A LED chaser with one parameter.
pub trait OneParameterChaser<Color, const N: usize>: Chaser<N> {
    fn new(start_color: Color, time_config: &TimeConfig) -> Self;
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
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        match self {
            OneParameterChaserEnum::Unicolor(chaser) => {
                chaser.set_time_config(time_config)
            }
            OneParameterChaserEnum::Rainbow(chaser) => {
                chaser.set_time_config(time_config)
            }
        }
    }
}

impl<Color: Into<Hsv>, const N: usize> OneParameterChaser<Color, N>
    for OneParameterChaserEnum<N>
{
    fn new(start_color: Color, time_config: &TimeConfig) -> Self {
        OneParameterChaserEnum::Unicolor(RainbowChaser::new(
            start_color,
            time_config,
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
