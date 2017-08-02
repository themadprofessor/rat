use std::io;
use std::io::BufReader;
use std::fs::File;
use std::env;

fn main() {
    for arg in env::args().skip(1) {
        let mut read = match File::open(arg) {
            Ok(f) => BufReader::new(f),
            Err(e) => exit(-1, &e.to_string())
        };
        let stdout = io::stdout();
        let mut lock = stdout.lock();
        match io::copy(&mut read, &mut lock) {
            Ok(_) => (),
            Err(e) => exit(-2, &e.to_string())
        };
    }
}

fn exit(code: i32, msg: &str) -> ! {
    eprintln!("{}", msg);
    ::std::process::exit(code)
}
