//! # Examples
//!
//! ## Hello World!
//!
//! ```rust
//! use interp::interp;
//! let who = "World";
//! assert_eq!(interp!("Hello { who }!").to_string(), "Hello World!");
//! ```

use proc_macro_hack::proc_macro_hack;

mod interpolator;

#[proc_macro_hack]
pub use interp_macros::interp;

pub use crate::interpolator::Interpolator;
