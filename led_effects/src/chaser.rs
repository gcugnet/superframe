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
pub enum ChaserEnum<const N: usize> {
    RainbowUnicolor(RainbowChaser<Unicolor<Hsv, N>, N>),
    DoubleRainbow(RainbowChaser<Rainbow<N>, N>),
}

impl<const N: usize> From<RainbowChaser<Unicolor<Hsv, N>, N>>
    for ChaserEnum<N>
{
    fn from(chaser: RainbowChaser<Unicolor<Hsv, N>, N>) -> Self {
        ChaserEnum::RainbowUnicolor(chaser)
    }
}

impl<const N: usize> From<RainbowChaser<Rainbow<N>, N>> for ChaserEnum<N> {
    fn from(chaser: RainbowChaser<Rainbow<N>, N>) -> Self {
        ChaserEnum::DoubleRainbow(chaser)
    }
}

impl<const N: usize> Chaser<N> for ChaserEnum<N> {
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        match self {
            ChaserEnum::RainbowUnicolor(chaser) => {
                chaser.set_time_config(time_config)
            }
            ChaserEnum::DoubleRainbow(chaser) => {
                chaser.set_time_config(time_config)
            }
        }
    }
}

impl<const N: usize> Iterator for ChaserEnum<N> {
    type Item = OneParameterSequenceEnum<N>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ChaserEnum::RainbowUnicolor(chaser) => {
                chaser.next().map(Into::into)
            }
            ChaserEnum::DoubleRainbow(chaser) => chaser.next().map(Into::into),
        }
    }
}
