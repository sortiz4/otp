//! Vernam is a simple cryptographic command line utility offering perfect
//! secrecy. Encryption produces two files of equal size: a key file with the
//! extension `.key` and the encrypted file with the extension `.vnm`.
//! Decryption will produce (or overwrite) the original file without the `.vnm`
//! extension.
extern crate getopts;
extern crate rand;
#[macro_use]
pub mod macros;
pub mod core;
pub mod opts;
pub mod status;
pub mod text;
