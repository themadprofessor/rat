extern crate clap;

use std::io;
use std::io::BufReader;
use std::fs::File;

use clap::{Arg, App};

use input::Input;

mod input;

fn main() {
    let matches = App::new("Rat")
        .version("0.1.0")
        .author("Stuart Reilly")
        .about("Rust implementation of cat")
        .arg(Arg::with_name("files")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("tabs")
            .short("T")
            .long("show-tabs"))
        .get_matches();


    let stdout = io::stdout();
    let stdin = io::stdin();
    let mut lock = stdout.lock();

    if !matches.is_present("files") {
        let stdin = io::stdin();
        let mut input = Input::Stdin(stdin.lock());
        write(&mut input, &mut lock);
        return;
    }

    for arg in matches.values_of("files").unwrap() {
        let mut read = BufReader::new(if arg == "-" {
            Input::Stdin(stdin.lock())
        } else {
            Input::File(match File::open(arg) {
                Ok(f) => BufReader::new(f),
                Err(e) => exit(-1, &e.to_string())
            })
        });
        write(&mut read, &mut lock)
    }
}

fn write<T>(input: &mut T, out: &mut io::StdoutLock) where T: io::Read {
    match io::copy(input, out) {
        Ok(_) => (),
        Err(e) => exit(-2, &e.to_string())
    }
}

fn exit(code: i32, msg: &str) -> ! {
    eprintln!("{}", msg);
    ::std::process::exit(code)
}
