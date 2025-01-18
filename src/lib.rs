#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
mod flood;
mod general;
pub mod utils;

pub use crate::flood::extra_flood::*;
pub use crate::flood::normal_flood::*;
pub use crate::general::*;
