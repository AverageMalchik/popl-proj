// Connect to Rust source files.
mod grep;
mod grep_command_builder;

// Imports.
use grep::grep;
use grep_command_builder::GrepOptionsBuilder;
use r3bl_rs_utils::style_error;
use r3bl_rs_utils::utils::with;
use std::env::args;
use std::error::Error;
use std::process::exit;

fn main() {
    let args = args().collect::<Vec<String>>();
    with(run(args), |it| match it {
        Ok(()) => exit(0),
        Err(err) => {
            eprintln!("{}: {}", style_error("Problem encountered"), err);
            exit(1);
        }
    });
}

fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    grep(GrepOptionsBuilder::parse(args)?)
}
