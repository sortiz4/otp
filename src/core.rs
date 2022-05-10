use rand;
use rand::Rng;
use std::ffi::OsString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Stderr;
use std::io::Write;
use structopt::StructOpt;
use super::Error;
use super::Result;

#[derive(Debug, StructOpt)]
#[structopt(about = "A simple one-time pad implementation.")]
struct Options {
    /// Encrypt the file with a random key.
    #[structopt(short = "e", long = "encrypt")]
    encrypt: bool,

    /// Decrypt the file with a key.
    #[structopt(short = "d", long = "decrypt")]
    decrypt: bool,

    /// Show this message.
    #[structopt(short = "h", long = "help")]
    help: bool,

    /// Show the version.
    #[structopt(short = "v", long = "version")]
    version: bool,

    /// The files to be read by this tool.
    #[structopt(name = "FILES", parse(from_str))]
    files: Vec<String>,
}

pub struct Otp {
    options: Options,
    stderr: Stderr,
}

impl Otp {
    const EXT_KEY: &'static str = ".key";
    const EXT_LOCK: &'static str = ".lock";

    /// Constructs this program from an iterable of arguments.
    pub fn from_iter<I>(iter: I) -> Result<Self>
    where
        Self: Sized,
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        return Ok(
            Self {
                options: Options::from_iter_safe(iter)?,
                stderr: io::stderr(),
            }
        );
    }

    /// Replaces the standard error stream for this program.
    pub fn stderr(&mut self, stderr: Stderr) -> &mut Self {
        self.stderr = stderr;
        return self;
    }

    /// Runs this program and writes all errors.
    pub fn run(&mut self) -> Result<()> {
        let mut run = || -> Result<()> {
            // Write the help or version message
            if self.options.help {
                return self.help();
            }
            if self.options.version {
                return self.version();
            }

            // Validate the options
            self.validate()?;

            // Encrypt or decrypt the file
            return if self.options.encrypt {
                self.encrypt_file()
            } else if self.options.decrypt {
                self.decrypt_file()
            } else {
                Ok(())
            };
        };

        match run() {
            Ok(val) => {
                return Ok(val);
            },
            Err(err) => {
                writeln!(self.stderr, "Error: {}", err)?;
                return Err(err);
            },
        }
    }

    /// Writes the help message to the standard error stream.
    fn help(&mut self) -> Result<()> {
        Options::clap().write_help(&mut self.stderr)?;
        writeln!(self.stderr, "")?;
        return Ok(());
    }

    /// Writes the version message to the standard error stream.
    fn version(&mut self) -> Result<()> {
        Options::clap().write_version(&mut self.stderr)?;
        writeln!(self.stderr, "")?;
        return Ok(());
    }

    /// Validates the options.
    fn validate(&self) -> Result<()> {
        let len = if self.options.encrypt {
            1
        } else {
            2
        };
        return if self.options.encrypt && self.options.decrypt {
            Err(Error::Conflict)
        } else if self.options.files.len() < len {
            Err(Error::Length)
        } else {
            Ok(())
        };
    }

    /// Encrypts the source file. Two files will be created when this function
    /// returns: a key file `{src}.key` and the encrypted file `{src}.lock`.
    fn encrypt_file(&self) -> Result<()> {
        let src = &self.options.files[0];
        let mut rng = rand::thread_rng();

        // Open the source file and wrap it in a buffer
        let src_file = File::open(src)?;
        let src_buf = BufReader::new(src_file);

        // Open the destination file and wrap it in a buffer
        let mut dest_file = {
            OpenOptions::new()
                .write(true)
                .create(true)
                .open([&src, Self::EXT_LOCK].concat())?
        };
        let mut dest_buf = BufWriter::new(dest_file);

        // Open the key file and wrap it in a buffer
        let mut key_file = {
            OpenOptions::new()
                .write(true)
                .create(true)
                .open([&src, Self::EXT_KEY].concat())?
        };
        let mut key_buf = BufWriter::new(key_file);

        // Encrypt the file and save the key
        for src_byte in src_buf.bytes() {
            let key_byte = rng.gen::<u8>();
            key_buf.write(&[key_byte])?;
            dest_buf.write(&[src_byte? ^ key_byte])?;
        }

        // Flush the key buffer to the disk
        key_file = key_buf.into_inner()?;
        key_file.sync_all()?;

        // Flush the destination buffer to the disk
        dest_file = dest_buf.into_inner()?;
        dest_file.sync_all()?;
        return Ok(());
    }

    /// Decrypts the source file with the key file. The unencrypted file will
    /// be created (or overwritten) when this function returns and the `.lock`
    /// extension will be removed (if it exists).
    fn decrypt_file(&self) -> Result<()> {
        let src = &self.options.files[0];
        let key = &self.options.files[1];

        // Open the source file and wrap it in a buffer
        let src_file = File::open(src)?;
        let src_buf = BufReader::new(src_file);

        // Open the key file and wrap it in a buffer
        let key_file = File::open(key)?;
        let key_buf = BufReader::new(key_file);

        // Open the destination file and wrap it in a buffer
        let mut dest_file = {
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(src.trim_end_matches(Self::EXT_LOCK))?
        };
        let mut dest_buf = BufWriter::new(dest_file);

        // Decrypt the file with the key
        for (src_byte, key_byte) in src_buf.bytes().zip(key_buf.bytes()) {
            dest_buf.write(&[src_byte? ^ key_byte?])?;
        }

        // Flush the destination buffer to the disk
        dest_file = dest_buf.into_inner()?;
        dest_file.sync_all()?;
        return Ok(());
    }
}
