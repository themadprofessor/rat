use std::io;
use std::io::{Write, BufReader, BufRead};
use std::fs::File;
use std::borrow::Cow;
use std::ascii::AsciiExt;

use regex::Regex;

use input::Input;
use config::{Config, Numbering};

macro_rules! wrap {
    ($path: ident, $w: expr) => {{
        if let Err(e) = $w {
            eprintln!("{}: {}", $path, e);
            break
        }
    }};
}

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

pub fn slow_write<'a, S, T>(files: T, config: &Config) where S: Iterator<Item=&'a str>,
                                                             T: IntoIterator<Item=&'a str, IntoIter=S> {
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

        let mut line = String::new();
        let mut squeeze = false;
        let mut number: usize = 0;
        loop {
            line.clear();
            match input.read_line(&mut line) {
                Ok(num) => if num == 0 {
                    break
                },
                Err(e) => {
                    eprintln!("{}: {}", path, e);
                    break
                }
            }
            let is_blank = line.trim().is_empty();

            if config.squeeze && is_blank {
                if !squeeze {
                    wrap!(path, stdout.write("\n".as_ref()));
                    squeeze = true;
                }
                continue
            }
            squeeze = false;

            if config.tabs {
                line = show_tabs(&line).into_owned();
            }

            if config.endings {
                line = show_ends(&line).into_owned();
            }

            if config.non_printing {
                line = show_non(&line);
            }

            match config.numbering {
                Numbering::All => {
                    number += 1;
                    if let Err(e) = write!(stdout, "{:>6} ", number) {
                        eprintln!("{}: {}", path, e);
                        break
                    }
                },
                Numbering::NonEmpty => {
                    if !is_blank {
                        number += 1;
                        if let Err(e) = write!(stdout, "{:>6}. ", number) {
                            eprintln!("{}: {}", path, e);
                            break
                        }
                    }
                },
                Numbering::None => ()
            }

            if let Err(e) = stdout.write(line.as_ref()) {
                eprintln!("{}: {}", path, e);
                break
            }
        }
    }
}

fn show_tabs<'a>(line: &'a str) -> Cow<'a, str> {
    lazy_static! {
        static ref REG_TAB: Regex = Regex::new(r"\t").unwrap();
    }
    REG_TAB.replace(line, "^I")
}

fn show_ends<'a>(line: &'a str) -> Cow<'a, str> {
    lazy_static!{
        static ref REG_END: Regex = Regex::new(r"\n").unwrap();
    }
    REG_END.replace(line, "$\n")
}

fn show_non<'a>(line: &'a str) -> String {
    let mut res = String::with_capacity(line.len() * 2);
    for c in line.chars() {
        if c.is_ascii_control() && c != '\n' && c != '\t' {
            res.push('^');
            res.push((c as u8 ^ 0b0100_0000) as char)
        } else {
            res.push(c)
        }
    }
    res
}