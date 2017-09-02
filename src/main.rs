#![feature(ascii_ctype)]

#[macro_use] extern crate clap;

use std::iter;

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
    } else if matches.is_present("files") {
        cat::slow_write(matches.values_of("files").unwrap(), &config)
    } else {
        cat::slow_write(iter::once("-"), &config)
    }

}

