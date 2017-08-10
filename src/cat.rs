use std::io;
use std::io::{Read, Write, BufReader};
use std::fs::File;

use input::Input;
use config::{Config, Numbering};

pub fn fast_write<'a, S, T>(files: T) where S: Iterator<Item=&'a str>, T: IntoIterator<Item=&'a str, IntoIter=S> {
    let raw_in = io::stdin();
    let raw_out = io::stdout();
    let mut stdout = raw_out.lock();

    for path in files {
        let mut input = if path == "-" {
            Input::Stdin(raw_in.lock())
        } else {
            Input::File(BufReader::new(match File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{}: {}", path, e);
                    continue
                }
            }))
        };
        if let Err(e) = io::copy(&mut input, &mut stdout) {
            eprintln!("{}: {}", path, e)
        };
    }
}

pub fn slow_write<'a, S, T>(files: T) where S: Iterator<Item=&'a str>, T: IntoIterator<Item=&'a str, IntoIter=S> {

}