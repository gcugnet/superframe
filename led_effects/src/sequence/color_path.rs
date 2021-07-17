use smart_leds::RGB8;

use super::{ArraySequence, Sequence};

/// A sequence in which the LEDs draw a color path.
pub struct ColorPath<const N: usize, const M: usize> {
    /// The list of colors.
    colors: [RGB8; M],
    // IDEA: Add an offset option.
    /// The counter.
    counter: usize,
}

impl<const N: usize, const M: usize> Sequence<N> for ColorPath<N, M> {}

// TODO: Check if we really need Copy here.
impl<Color: Into<RGB8> + Copy, const N: usize, const M: usize>
    ArraySequence<Color, N, M> for ColorPath<N, M>
{
    fn new(colors_input: [Color; M]) -> Self {
        let mut colors = [RGB8::default(); M];

        for (i, &color) in colors_input.iter().enumerate() {
            colors[i] = color.into();
        }

        Self { colors, counter: 0 }
    }
}

impl<const N: usize, const M: usize> Iterator for ColorPath<N, M> {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < N {
            let gradient_size = M / N;

            let start_color = self.colors[self.counter / gradient_size];
            let end_color = self.colors[self.counter / gradient_size + 1];

            let color = RGB8 {
                r: gradient_step(
                    start_color.r,
                    end_color.r,
                    self.counter % gradient_size,
                    gradient_size,
                ),
                g: gradient_step(
                    start_color.g,
                    end_color.g,
                    self.counter % gradient_size,
                    gradient_size,
                ),
                b: gradient_step(
                    start_color.b,
                    end_color.b,
                    self.counter % gradient_size,
                    gradient_size,
                ),
            };

            self.counter += 1;
            Some(color)
        } else {
            None
        }
    }
}

// IDEA: Factorise this with other modules.
fn gradient_step(start: u8, end: u8, step: usize, step_number: usize) -> u8 {
    let start_i16 = start as i16;
    let end_i16 = end as i16;
    let step_i16 = step as i16;
    let step_number_i16 = step_number as i16;

    (start_i16 + (step_i16 * (end_i16 - start_i16)) / (step_number_i16 - 1))
        as u8
}
