//! Defines the options available.
use getopts::Matches;
use getopts::Options;
use std::path::Path;
use super::status;
use super::text;

/// Used to define a new option flag.
pub struct Option<'a> {
    /// A short option (used with `-`).
    pub short: &'a str,
    /// A long option (used with `--`).
    pub long: &'a str,
    /// A brief description.
    pub description: &'a str,
}

/// Defines the 'decrypt' option flag.
pub const DECRYPT: Option<'static> = Option {
    short: "d", long: "decrypt",
    description: "Decrypt the file with the key",
};

/// Defines the 'encrypt' option flag.
pub const ENCRYPT: Option<'static> = Option {
    short: "e", long: "encrypt",
    description: "Encrypt the file with a random key",
};

/// Defines the 'help' option flag.
pub const HELP: Option<'static> = Option {
    short: "h", long: "help",
    description: "Output this message",
};

/// Defines the 'version' option flag.
pub const VERSION: Option<'static> = Option {
    short: "V", long: "version",
    description: "Output version information",
};

/// Registers each option in the list.
macro_rules! optflags {
    ($opts:expr; $($name:ident),*) => {
        $($opts.optflag($name.short, $name.long, $name.description);)*
    };
}

/// Reformats the `getopts` error message.
macro_rules! opts {
    ($var:expr) => ($var.to_string().to_lowercase().trim_right_matches(".").to_owned());
}

/// Appends the help string to the end of the given message.
macro_rules! help {
    ($fmt:expr) => (format!(concat!($fmt, "\n{}"), text::HELP));
    ($fmt:expr, $($arg:tt)*) => (format!(concat!($fmt, "\n{}"), $($arg)*, text::HELP));
}

/// Initializes a set of options from the option definitions.
pub fn create() -> Options {
    let mut options = Options::new();
    optflags![options; DECRYPT, ENCRYPT, HELP, VERSION];
    return options;
}

/// Parses a set of arguments into a set of matches.
pub fn parse(args: &Vec<String>, options: &Options) -> Result<Matches, String> {
    let matches = match options.parse(&args[1..]) {
        Ok(val) => val,
        Err(err) => return Err(help!("{}", opts!(err))),
    };
    return Ok(matches);
}

/// Checks for conflicts in the set of matches.
pub fn validate(matches: &Matches) -> Result<(), String> {
    if matches.opt_present(ENCRYPT.short) && matches.opt_present(DECRYPT.short) {
        return Err(help!("{}: '{}', '{}'", status::MCONFLICT, ENCRYPT.long, DECRYPT.long));
    }
    if matches.opt_present(ENCRYPT.short) {
        if matches.free.len() < 1 {
            return Err(help!("{}", "missing argument(s)"));
        }
        let file = Path::new(&matches.free[0]);
        if !file.exists() || !file.is_file() {
            return Err(help!("'{}' {}", file.display(), "is not a file"));
        }
    } else if matches.opt_present(DECRYPT.short) {
        if matches.free.len() < 2 {
            return Err(help!("{}", "missing argument(s)"));
        }
        let file = Path::new(&matches.free[0]);
        let key = Path::new(&matches.free[1]);
        if !file.exists() || !file.is_file() {
            return Err(help!("'{}' {}", file.display(), "is not a file"));
        }
        if !key.exists() || !key.is_file() {
            return Err(help!("'{}' {}", key.display(), "is not a file"));
        }
    }
    return Ok(());
}
