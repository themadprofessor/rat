use std::io::{Read, BufRead, BufReader, StdinLock};
use std::io;
use std::fs::File;

#[derive(Debug)]
pub enum Input<'a> {
    File(BufReader<File>),
    Stdin(StdinLock<'a>)
}

impl <'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::File(ref mut f) => f.read(buf),
            Input::Stdin(ref mut i) => i.read(buf)
        }
    }
}

impl <'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match *self {
            Input::File(ref mut f) => f.fill_buf(),
            Input::Stdin(ref mut i) => i.fill_buf()
        }
    }

    fn consume(&mut self, amt: usize) {
        match *self {
            Input::File(ref mut f) => f.consume(amt),
            Input::Stdin(ref mut i) => i.consume(amt)
        }
    }
}