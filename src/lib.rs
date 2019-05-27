#[macro_use]
extern crate lazy_static;

pub mod cli;
pub mod expr;
pub mod graphics;
pub mod lsys;
pub mod parser;
pub mod turtle;

pub const KOCH: &str = include_str!("data/koch");
pub const SIMPLE: &str = include_str!("data/simple");
