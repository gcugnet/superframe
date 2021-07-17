//! Utilities to deal with time in chasers.

use embedded_time::{duration::Generic, rate::Hertz};

/// Timing configuration.
#[derive(Debug)]
pub struct TimeConfig {
    pub refresh_rate: Hertz,
    pub transition_time: Generic<u32>,
}

impl TimeConfig {
    /// Builds a new timing configuration.
    pub fn new(
        refresh_rate: Hertz,
        transition_time: impl Into<Generic<u32>>,
    ) -> Self {
        Self {
            refresh_rate,
            transition_time: transition_time.into(),
        }
    }

    /// Returns the number of steps for a transition.
    pub fn transition_steps(&self) -> u32 {
        self.refresh_rate.0
            * self.transition_time.integer()
            * self.transition_time.scaling_factor().numerator()
            / self.transition_time.scaling_factor().denominator()
    }
}
