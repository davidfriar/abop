mod graphics;
mod lsys;
mod parser;
mod turtle;

use crate::lsys::{ActualParam, Element, LString, LSystem, ParamList};
// use crate::parser;
use crate::turtle::Turtle;

const KOCH: &str = include_str!("data/koch");
const SIMPLE: &str = include_str!("data/simple");

fn main() {
    // let lsys=LSystem::new(KOCH);
    let mut lsys = parser::parse_lsys(SIMPLE);
    // for _ in 1..10 {
    //     lsys.generate();
    // }
    // let mut turtle = Turtle::new();

    // let model=turtle.interpret(lsys.current);

    // graphics::run(model);
    //
    //
    //
    //println!("{}", lsys);
    // for _ in 1..10 {
    //     lsys.generate();
    //     println!("{}", lsys);
    // }

    // let now = std::time::Instant::now();
    // for i in 1..28 {
    //     lsys.generate();
    //     println!(
    //         "{} - {} - {} - {}",
    //         i,
    //         lsys.current.capacity(),
    //         lsys.current.len(),
    //         now.elapsed().as_millis()
    //     );
    // }

    //println!("hello {:?}", lsys.take(2).collect::<Vec<LString>>());

    println!("hello {:?}", lsys.nth(0).unwrap());

    println!(
        "sizeof element : {}",
        std::mem::size_of::<Element<ActualParam>>()
    );
    println!(
        "sizeof paramlist : {}",
        std::mem::size_of::<ParamList<ActualParam>>()
    );
}
