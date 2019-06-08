use abop::cli::Opt;
use abop::graphics;
use abop::parser;
use abop::turtle;
use std::fs;
use std::io::{self, Read};

use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    let input = read_input(&opt);

    if opt.input_graphics_model {
        //to do : parse model
        graphics::Application::new().run(&vec![]);
        return;
    }

    let mut lsys = parser::parse_lsys(&input);
    let mut turtle = turtle::Turtle::new();

    lsys.nth(opt.iterations);

    if opt.output_lsystem {
        match opt.verbose {
            true => println!("{}", lsys),
            false => println!("{}", lsys.current),
        }
        return;
    }

    let model = turtle.interpret(&lsys.current);
    if opt.output_model {
        println!("{:?}", model); // to do - Display for Model
        return;
    }

    graphics::Application::new().run(model);
}

fn read_input(opt: &Opt) -> String {
    match &opt.file {
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
        Some(path) => fs::read_to_string(path).unwrap(),
    }
}
