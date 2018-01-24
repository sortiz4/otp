//! Defines common string fragments.

/// The program name.
pub const NAME: &str = name![];
/// The program version.
pub const VERSION: &str = "1.0";
/// A generic help message.
pub const HELP: &str = concat![
    "Try '", name![], " --help' for more information.",
];
/// A brief description of the program and its usage.
pub const USAGE: &str = concat![
    "A simple Vernam Cipher implementation.\n\n",
    "Usage:\n", indent![name![], "[OPTIONS] [FILE] [KEY]"],
];
