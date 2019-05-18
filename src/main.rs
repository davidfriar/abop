
mod lsys;
mod turtle;
mod graphics;

use crate::lsys::LSystem;
use crate::turtle::Turtle;
// use crate::graphics::run;

const KOCH:&str = include_str!("data/koch");


fn main() {
    let lsys=LSystem::new(KOCH);
    for _ in 1..10 {
        lsys.generate();
    }
    let mut turtle = Turtle::new();

    let model=turtle.interpret(lsys.current);

    graphics::run(model);

    println!("{:?}", lsys)
}
