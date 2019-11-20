use otp::Otp;
use std::env;
use std::process;

fn main() {
    process::exit(run());
}

fn run() -> i32 {
    if let Ok(mut otp) = Otp::from_iter(env::args()) {
        if let Ok(_) = otp.run() {
            return 0;
        }
    }
    return 1;
}
