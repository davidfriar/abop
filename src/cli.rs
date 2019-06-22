use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// The number of times to iterate the lsystem
    #[structopt(
        name = "number of iterations",
        short = "n",
        long = "num-iterations",
        default_value = "0"
    )]
    pub iterations: usize,

    /// File from which to read the lsystem.
    /// If a filename is not supplied input will be taken from STDIN.
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: Option<PathBuf>,

    /// Prompt to choose a file from this directory.
    #[structopt(name = "directory", short, long, parse(from_os_str))]
    pub dir: Option<PathBuf>,

    /// Run in an interactive loop, prompting for generation of the next iteration.
    #[structopt(short, long)]
    pub interactive: bool,

    /// Produce verbose output. Currently this option only has an effect in
    /// combination with '-l'.
    #[structopt(short, long)]
    pub verbose: bool,

    /// Output the generated model without rendering graphics
    #[structopt(name = "model", short, long, conflicts_with = "lsystem, graphics")]
    pub output_model: bool,

    /// Output the generated lsystem without interpreting as turtle commands
    /// or rendering graphics. By default, prints the current lsystem string only. To show the
    /// rules, use the '-v' option.
    #[structopt(name = "lsystem", short, long, conflicts_with = "model, graphics")]
    pub output_lsystem: bool,

    /// Input a model and render it as graphics. (Skip the lsystem generation and turtle interpretation stages.)
    #[structopt(
        name = "graphics",
        short,
        long,
        conflicts_with = "lsystem, turtle, model"
    )]
    pub input_graphics_model: bool,
}

impl Opt {
    pub fn use_graphics(&self) -> bool {
        !(self.output_model || self.output_lsystem)
    }

    pub fn use_turtle(&self) -> bool {
        !self.output_lsystem && !self.input_graphics_model
    }

    pub fn use_lsystem(&self) -> bool {
        !self.input_graphics_model
    }
}
