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
        let endings = args.is_present("ends") || args.is_present("show_all") || args.is_present("non_ending");
        let squeeze = args.is_present("squeeze");
        let non_printing = args.is_present("nonprint") || args.is_present("non_tabs") || args.is_present("show_all") || args.is_present("non_ending");
        let tabs = args.is_present("tabs") || args.is_present("non_tabs") || args.is_present("show_all");

        Config {
            numbering,
            endings,
            squeeze,
            non_printing,
            tabs
        }
    }

    pub fn is_fast(&self) -> bool {
        self.numbering == Numbering::None && !self.endings && !self.squeeze && !self.non_printing && !self.tabs
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
            .multiple(true)
            .help("files to be concatenated")
            .long_help("List of files to be concatenated. If no files are specified or a - is given, standard input will be read."))
        .arg(Arg::with_name("number_all")
            .short("n")
            .long("number")
            .help("number all outputted lines")
            .long_help("Prepend an incrementing number to the beginning of each line."))
        .arg(Arg::with_name("number_non")
            .short("b")
            .long("number-nonblank")
            .overrides_with("number_all")
            .help("number non-blank lines")
            .long_help("Prepend an incrementing number to the beginning of all non-empty lines."))
        .arg(Arg::with_name("nonprint")
            .short("v")
            .long("show-nonprinting")
            .help("use ^ and M- notation, except for LFD and TAB")
            .long_help("Display all non-printing characters using ^ and M- notation, except LFD and TAB."))
        .arg(Arg::with_name("tabs")
            .short("T")
            .long("show-tabs")
            .help("display TAB characters as ^I")
            .long_help("Display all TAB characters as ^I."))
        .arg(Arg::with_name("ends")
            .short("E")
            .long("show-ends")
            .help("display $ at the end of each line")
            .long_help("Append a $ character at the end of each line, preserving the EOL character."))
        .arg(Arg::with_name("squeeze")
            .short("s")
            .long("squeeze-blank")
            .help("suppress repeated empty lines")
            .long_help("Suppress multiple empty lines into a single empty line."))
        .arg(Arg::with_name("show_all")
            .short("A")
            .long("show-all")
            .help("equivalent to -vET")
            .long_help("Display all non-printing characters using ^ and M- notation, except LFD, and append a $ character at the the end of each line, preserving the EOL character."))
        .arg(Arg::with_name("none_ending")
            .short("e")
            .help("equivalent to -vE")
            .long_help("Display all non-printing characters using ^ and M- notation, except LFD and TAB, and append a $ character at the the end of each line, preserving the EOL character."))
        .arg(Arg::with_name("none_tabs")
            .short("t")
            .help("equivalent to -vT")
            .long_help("Display all non-printing characters using ^ and M- notation, except LFD."))
}
