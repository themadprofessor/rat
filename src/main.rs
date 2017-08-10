#[macro_use] extern crate clap;

use std::io;
use std::io::BufReader;
use std::fs::File;
use std::iter;

use clap::ArgMatches;

use input::Input;

mod input;
mod config;
mod cat;

fn main() {
    let matches = config::build_cli().get_matches();
    let config = config::Config::from_args(&matches);

    if config.is_fast() {
        if matches.is_present("files") {
            cat::fast_write(matches.values_of("files").unwrap())
        } else {
            cat::fast_write(iter::once("-"))
        }
    } else {
        println!("TODO");
    }
}

fn exit(code: i32, msg: &str) -> ! {
    eprintln!("{}", msg);
    ::std::process::exit(code)
}
