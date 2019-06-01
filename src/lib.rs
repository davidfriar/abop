#[macro_use]
extern crate lazy_static;
extern crate num_traits;

#[macro_use]
extern crate approx;

pub mod cli;
pub mod expr;
pub mod graphics;
pub mod lsys;
pub mod parser;
pub mod turtle;

pub const KOCH: &str = include_str!("data/koch");
pub const SIMPLE: &str = include_str!("data/simple");
