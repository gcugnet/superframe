/// A sequence in which all LEDs have the same color.
pub struct Unicolor<Color> {
    /// The color for all LEDs.
    color: Color,
    /// The number of LEDs.
    number: usize,
    /// The counter.
    counter: usize,
}

impl<Color> Iterator for Unicolor<Color>
where
    Color: Copy,
{
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.number {
            self.counter += 1;
            Some(self.color)
        } else {
            None
        }
    }
}

impl<Color> Unicolor<Color> {
    /// Create a new unicolor sequence.
    pub fn new(color: Color, number: usize) -> Self {
        Self {
            color,
            number,
            counter: 0,
        }
    }
}
