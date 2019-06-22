#[macro_use]
extern crate lazy_static;
extern crate approx;
extern crate num_traits;

pub mod cli;
pub mod config;
pub mod expr;
pub mod graphics;
pub mod iter;
pub mod lsys;
pub mod parser;
pub mod turtle;

pub const KOCH: &str = include_str!("data/koch");
pub const SIMPLE: &str = include_str!("data/simple");
