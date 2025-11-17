/// Default tolerance (in Hz) used by helper utilities such as [`Pitch::approx_eq`].
pub const DEFAULT_FREQUENCY_EPSILON: f32 = 1.0e-4;

mod r#abstract;
mod errors;
mod implementation;
mod label;

pub use r#abstract::*;
pub use errors::*;
pub use implementation::*;
pub use label::*;
