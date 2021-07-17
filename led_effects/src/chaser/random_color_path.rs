use embedded_time::{duration::Extensions, rate::Hertz};
use rand::{
    distributions::{Distribution, Uniform},
    rngs::SmallRng,
    Rng, SeedableRng,
};
use smart_leds::{hsv::Hsv, RGB8};

use super::{
    Chaser, SimpleRandomChaser, TwoParameterChaser, UnicolorTransition,
};
use crate::{sequence::ColorPath, time::TimeConfig};

pub struct RandomColorPath<D: Distribution<u32>, const N: usize, const M: usize>
{
    /// The random number generator for color and transition speed selection.
    rng: SmallRng,
    /// The refresh rate.
    refresh_rate: Hertz,
    /// The transition speed distribution.
    transition_time_distr: D,

    // TODO: A list of Transition, of which color results will be passed to the
    // color path.
    colors: [RGB8; M],
}
