//! A collection of LED sequences on top of `smart_leds`.

mod gradient;
mod rainbow;
mod unicolor;

pub use gradient::Gradient;
pub use rainbow::Rainbow;
pub use unicolor::Unicolor;
