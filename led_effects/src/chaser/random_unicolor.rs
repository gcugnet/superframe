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
use crate::{sequence::Unicolor, time::TimeConfig};

/// A LED chaser that performs random transitions.
pub struct RandomUnicolor<D: Distribution<u32>, const N: usize> {
    /// The random number generator for color and transition speed selection.
    rng: SmallRng,
    /// The refresh rate.
    refresh_rate: Hertz,
    /// The transition speed distribution.
    transition_time_distr: D,
    /// The ongoing transition.
    transition: UnicolorTransition<N>,
}

impl<const N: usize> Chaser<N> for RandomUnicolor<Uniform<u32>, N> {
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        // IDEA: Also update the ongoing transition, live!
        self.refresh_rate = time_config.refresh_rate;

        let median_time_ms = time_config.transition_time.integer()
            * 1000
            * time_config.transition_time.scaling_factor().numerator()
            / time_config.transition_time.scaling_factor().denominator();

        // IDEA: Make this parametrisable.
        let v = median_time_ms * 2 / 3;
        self.transition_time_distr =
            Uniform::new(median_time_ms - v, median_time_ms + v);
    }
}

impl<const N: usize> SimpleRandomChaser<Uniform<u32>, N>
    for RandomUnicolor<Uniform<u32>, N>
{
    fn new(refresh_rate: Hertz, transition_time_distr: Uniform<u32>) -> Self {
        let mut rng = SmallRng::seed_from_u64(0);

        let start_color = Hsv {
            hue: rng.gen(),
            sat: 255,
            val: 255,
        };

        let transition = generate_transition(
            &mut rng,
            refresh_rate,
            &transition_time_distr,
            start_color,
        );

        Self {
            rng,
            refresh_rate,
            transition_time_distr,
            transition,
        }
    }
}

impl<D: Distribution<u32>, const N: usize> Iterator for RandomUnicolor<D, N> {
    type Item = Unicolor<RGB8, N>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.transition.next() {
            Some(sequence) => Some(sequence),
            None => {
                self.transition = generate_transition(
                    &mut self.rng,
                    self.refresh_rate,
                    &self.transition_time_distr,
                    self.transition.end_color().into(),
                );

                self.transition.next()
            }
        }
    }
}

fn generate_transition<const N: usize>(
    rng: &mut impl Rng,
    refresh_rate: Hertz,
    transition_time_distr: &impl Distribution<u32>,
    start_color: Hsv,
) -> UnicolorTransition<N> {
    let end_color = Hsv {
        hue: rng.gen(),
        sat: 255,
        val: 255,
    };

    let transition_time = rng.sample(transition_time_distr).milliseconds();
    let time_config = TimeConfig::new(refresh_rate, transition_time);

    UnicolorTransition::new(start_color, end_color, &time_config)
}
