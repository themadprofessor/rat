use clap::{ArgMatches, Arg, App};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config {
    pub numbering: Numbering,
    pub endings: bool,
    pub squeeze: bool,
    pub non_printing: bool,
    pub tabs: bool
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Numbering {
    All,
    NonEmpty,
    None
}

impl Config {
    pub fn from_args(args: &ArgMatches) -> Config {
        let numbering = if args.is_present("number_non") {
            Numbering::NonEmpty
        } else if args.is_present("number_all") {
            Numbering::All
        } else {
            Numbering::None
        };

        Config::default()
    }
}

impl Default for Numbering {
    fn default() -> Self {
        Numbering::None
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            numbering: Numbering::default(),
            endings: false,
            squeeze: false,
            non_printing: false,
            tabs: false
        }
    }
}

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .arg(Arg::with_name("files")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("number_all")
            .short("n")
            .long("number"))
        .arg(Arg::with_name("number_non")
            .short("b")
            .long("number-nonblank")
            .overrides_with("number_all"))
        .arg(Arg::with_name("nonprint")
            .short("v")
            .long("show-nonprinting"))
        .arg(Arg::with_name("tabs")
            .short("T")
            .long("show-tabs"))
        .arg(Arg::with_name("ends")
            .short("E")
            .long("show-ends"))
        .arg(Arg::with_name("show_all")
            .short("A")
            .long("show-all"))
        .arg(Arg::with_name("none_ending")
            .short("e"))
        .arg(Arg::with_name("none_tabs")
            .short("T"))
}
