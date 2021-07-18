use smart_leds::RGB8;

use super::{Sequence, TwoParameterSequence};

/// A sequence in which the LEDs draw a gradient.
pub struct Gradient<const N: usize> {
    /// The departure color of the gradient.
    start_color: RGB8,
    /// The arrival color of the gradient.
    end_color: RGB8,
    /// The counter.
    counter: usize,
}

impl<const N: usize> Sequence<N> for Gradient<N> {}

impl<Color: Into<RGB8>, const N: usize> TwoParameterSequence<Color, N>
    for Gradient<N>
{
    fn new(start_color: Color, end_color: Color) -> Self {
        Self {
            start_color: start_color.into(),
            end_color: end_color.into(),
            counter: 0,
        }
    }
}

impl<const N: usize> Iterator for Gradient<N> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            let color = RGB8 {
                r: gradient_step::<N>(
                    self.start_color.r,
                    self.end_color.r,
                    self.counter,
                ),
                g: gradient_step::<N>(
                    self.start_color.g,
                    self.end_color.g,
                    self.counter,
                ),
                b: gradient_step::<N>(
                    self.start_color.b,
                    self.end_color.b,
                    self.counter,
                ),
            };
            self.counter += 1;
            Some(color)
        } else {
            None
        }
    }
}

fn gradient_step<const N: usize>(start: u8, end: u8, step: usize) -> u8 {
    let start_i16 = start as i16;
    let end_i16 = end as i16;
    let step_i16 = step as i16;
    let led_number = N as i16;

    (start_i16 + (step_i16 * (end_i16 - start_i16)) / (led_number - 1)) as u8
}
