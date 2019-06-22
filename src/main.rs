use abop::cli::Opt;
use abop::graphics;
use abop::lsys::LSystem;
use abop::parser;
use abop::turtle::Turtle;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::fs;
use std::io::{self, Read};
use structopt::StructOpt;

struct Application {
    options: Opt,
    input: String,
    lsystem: Option<LSystem>,
    turtle: Option<Turtle>,
}

impl Application {
    fn new(options: Opt) -> Self {
        let input = Self::read_input(&options);
        let mut app = Application {
            options,
            input,
            lsystem: None,
            turtle: None,
        };
        if app.options.use_lsystem() {
            app.lsystem = Some(parser::parse_lsys(&app.input));
        }
        if app.options.use_turtle() {
            app.turtle = Some(Turtle::new());
        }
        app
    }

    fn run(mut self) {
        if let Some(lsys) = &mut self.lsystem {
            lsys.nth(self.options.iterations);
        }

        if self.options.use_graphics() {
            graphics::Application::new(self).run();
        } else {
            'app: loop {
                if let Some(turtle) = &mut self.turtle {
                    if let Some(lsystem) = &self.lsystem {
                        println!("{:?}", turtle.interpret(&lsystem.current)); // to do - Display for Model
                    }
                } else if let Some(lsystem) = &self.lsystem {
                    match self.options.verbose {
                        true => println!("{}", lsystem),
                        false => println!("{}", lsystem.current),
                    }
                }

                if !self.options.interactive {
                    break;
                } else {
                    println!("Press SPACE for next iteration, ESC to quit");
                    let device_state = DeviceState::new();
                    loop {
                        let keys: Vec<Keycode> = device_state.get_keys();
                        if keys.contains(&Keycode::Space) {
                            if let Some(lsystem) = &mut self.lsystem {
                                lsystem.generate();
                                break;
                            }
                        }

                        if keys.contains(&Keycode::Escape) {
                            break 'app;
                        }
                    }
                }
            }
        }
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
}

fn main() {
    Application::new(Opt::from_args()).run();
}

impl graphics::Displayable for Application {
    fn vertices(&mut self) -> Vec<graphics::Vertex> {
        if let Some(turtle) = &mut self.turtle {
            if let Some(lsystem) = &mut self.lsystem {
                turtle.interpret(&lsystem.current)
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!()
        }
    }

    fn update(&mut self) {
        if let Some(lsystem) = &mut self.lsystem {
            lsystem.generate();
            self.turtle = Some(Turtle::new());
        }
    }
}
