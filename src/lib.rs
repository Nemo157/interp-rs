//! # Examples
//!
//! ## Hello World!
//!
//! ```rust
//! #![feature(proc_macro_hygiene)]
//! use interp::interp;
//! let who = "World";
//! assert_eq!(interp!("Hello { who }!").to_string(), "Hello World!");
//! ```

mod interpolator;

pub use interp_macros::interp;
pub use crate::interpolator::Interpolator;
