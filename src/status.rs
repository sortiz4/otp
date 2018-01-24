//! Defines common exit codes (`E`) and error messages (`M`).

/// Successful execution.
pub const ESUCCESS: i32 = 0x00;
/// Invalid usage.
pub const EUSAGE: i32 = 0x01;
/// I/O error.
pub const EIO: i32 = 0x02;

/// Not enough arguments were provided.
pub const MARGS: &str = "missing argument(s)";
/// The file or directory cannot be found.
pub const MNOTFOUND: &str = "cannot not be found";
/// A usage error where conflicting options are present.
pub const MCONFLICT: &str = "conflicting options";
