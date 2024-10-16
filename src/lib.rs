#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::missing_transmute_annotations)]

pub mod bind;
mod externs;
pub mod id;
pub mod utils;
mod val;

pub use bind::*;
pub use id::*;
pub use val::*;

pub type TYPEID = emscripten_val_sys::val::TYPEID;
