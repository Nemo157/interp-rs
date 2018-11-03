//! # Examples
//!
//! ## Hello World!
//!
//! ```rust
//! #![feature(proc_macro_non_items)]
//! use interp::interp;
//! let who = "World";
//! assert_eq!(interp!("Hello { who }!").to_string(), "Hello World!");
//! ```

extern crate interp_macros;
extern crate take;

mod interpolator;

pub use interp_macros::interp;
pub use interpolator::Interpolator;
