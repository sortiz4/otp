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
macro_rules! reformat {
    ($var:expr) => ($var.to_string().to_lowercase().trim_right_matches(".").to_owned());
}

/// Appends the help string to the end of the given message.
macro_rules! help {
    ($fmt:expr) => (format!(concat!($fmt, "\n{}"), text::HELP));
    ($fmt:expr, $($arg:tt)*) => (format!(concat!($fmt, "\n{}"), $($arg)*, text::HELP));
}

/// Checks if the path exists and is a file.
fn validate_file(file: &String) -> Result<(), String> {
    let file = Path::new(file);
    if !file.exists() || !file.is_file() {
        return Err(help!("'{}' {}", file.display(), status::MNOTFOUND));
    }
    return Ok(());
}

/// Checks if the number of free arguments matches the length.
fn validate_len(matches: &Matches, len: usize) -> Result<(), String> {
    if matches.free.len() < len {
        return Err(help!("{}", status::MARGS));
    }
    return Ok(());
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
        Err(err) => return Err(help!("{}", reformat!(err))),
    };
    return Ok(matches);
}

/// Checks for conflicts and issues in the set of matches.
pub fn validate(matches: &Matches) -> Result<(), String> {
    if matches.opt_present(ENCRYPT.short) && matches.opt_present(DECRYPT.short) {
        return Err(help!("{}: '{}', '{}'", status::MCONFLICT, ENCRYPT.long, DECRYPT.long));
    }
    if matches.opt_present(ENCRYPT.short) {
        validate_len(matches, 1)?;
        validate_file(&matches.free[0])?;
    } else if matches.opt_present(DECRYPT.short) {
        validate_len(matches, 2)?;
        validate_file(&matches.free[0])?;
        validate_file(&matches.free[1])?;
    }
    return Ok(());
}
