#[macro_use]
extern crate vernam;
use std::env;
use std::process;
use vernam::core;
use vernam::opts;
use vernam::status;
use vernam::text;

fn main() {
    process::exit(vernam(env::args().collect()));
}

fn vernam(args: Vec<String>) -> i32 {
    // Parse the command line options
    let options = opts::create();
    let matches = match opts::parse(&args, &options) {
        Ok(val) => val,
        Err(err) => {
            sprintln!("{}", err);
            return status::EUSAGE;
        },
    };

    // Display the help message or version and exit (optional)
    if matches.opt_present(opts::HELP.short) {
        print!("{}", options.usage(text::USAGE));
        return status::ESUCCESS;
    } else if matches.opt_present(opts::VERSION.short) {
        println!("{} {}", text::NAME, text::VERSION);
        return status::ESUCCESS;
    }

    // Check for conflicting options
    if let Err(err) = opts::validate(&matches) {
        sprintln!("{}", err);
        return status::EUSAGE;
    }

    // Encrypt or decrypt the file
    if matches.opt_present(opts::ENCRYPT.short) {
        if let Err(err) = core::encrypt_file(&matches.free[0]) {
            sprintln!("{}", err);
        }
    } else if matches.opt_present(opts::DECRYPT.short) {
        if let Err(err) = core::decrypt_file(&matches.free[0], &matches.free[1]) {
            sprintln!("{}", err);
        }
    }
    return status::ESUCCESS;
}
