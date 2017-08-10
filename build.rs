#[macro_use] extern crate clap;

use clap::Shell;

include!("src/config.rs");

fn main() {
    let out = match ::std::env::var_os("OUT_DIR") {
        Some(path) => path,
        None => return
    };
    let mut app = build_cli();
    app.gen_completions("rat", Shell::Bash, out)
}