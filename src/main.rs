use abop::cli::Opt;
use abop::graphics;
use abop::parser;
use abop::turtle;
use std::fs;
use std::io::{self, Read};

use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let input = match opt.file {
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
        Some(path) => fs::read_to_string(path).unwrap(),
    };

    if opt.input_graphics_model {
        //to do : parse model
        graphics::run(vec![]);
        return;
    }

    let mut lsys = parser::parse_lsys(&input);

    if opt.iterations > 0 {
        lsys.nth(opt.iterations - 1);
    }

    if opt.output_lsystem {
        //to do : currently ignoring verbose. Need to solve how to implement Display for LString
        println!("{}", lsys);
        return;
    }

    let model = turtle::Turtle::new().interpret(&lsys.current);

    if opt.output_model {
        println!("{:?}", model); // to do - Display for Model
        return;
    }

    graphics::run(model);
}
