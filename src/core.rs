//! Defines the core functionality.
use rand;
use rand::Rng;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Result;
use std::io::Read;
use std::io::Write;

/// Key file extension.
const EXT_KEY: &str = ".key";
/// Ciphertext file extension.
const EXT_CIPHER: &str = ".vnm";

pub fn encrypt_file(src: &String) -> Result<()> {
    let mut rng = rand::thread_rng();
    let src_file = File::open(src)?;
    let src_buf = BufReader::new(src_file);
    let mut key_file = OpenOptions::new().write(true).create(true)
                            .open(format!("{}{}", src, EXT_KEY))?;
    let mut key_buf = BufWriter::new(key_file);
    let mut dest_file = OpenOptions::new().write(true).create(true)
                            .open(format!("{}{}", src, EXT_CIPHER))?;
    let mut dest_buf = BufWriter::new(dest_file);

    for src_byte in src_buf.bytes() {
        let src_byte = src_byte?;
        let dest_byte = rng.gen::<u8>();
        key_buf.write(&[dest_byte])?;
        dest_buf.write(&[src_byte ^ dest_byte])?;
    }

    // Flush the key buffer and write it to the disk
    key_file = key_buf.into_inner()?;
    key_file.sync_all()?;

    // Flush the destination buffer and write it to the disk
    dest_file = dest_buf.into_inner()?;
    dest_file.sync_all()?;

    return Ok(());
}

pub fn decrypt_file(src: &String, key: &String) -> Result<()> {
    let src_file = File::open(src)?;
    let src_buf = BufReader::new(src_file);
    let key_file = File::open(key)?;
    let key_buf = BufReader::new(key_file);
    let mut dest_file = OpenOptions::new().write(true).create(true)
                            .open(src.trim_right_matches(EXT_CIPHER))?;
    let mut dest_buf = BufWriter::new(dest_file);

    for (src_byte, key_byte) in src_buf.bytes().zip(key_buf.bytes()) {
        let src_byte = src_byte?;
        let key_byte = key_byte?;
        dest_buf.write(&[src_byte ^ key_byte])?;
    }

    // Flush the destination buffer and write it to the disk
    dest_file = dest_buf.into_inner()?;
    dest_file.sync_all()?;

    return Ok(());
}
