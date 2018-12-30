//! Defines the core functionality.
use rand;
use rand::Rng;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Result;
use std::io::Write;

/// Key file extension.
const EXT_KEY: &str = ".key";
/// Ciphertext file extension.
const EXT_CIPHER: &str = ".vm";

/// Encrypts the source file. Two files will be created when this function
/// returns: a key file `{src}.key` and the encrypted file `{src}.vm`.
pub fn encrypt_file(src: &String) -> Result<()> {
    // Cache the thread-local random generator
    let mut rng = rand::thread_rng();

    // Open the source file and wrap it in a buffer
    let src_file = File::open(src)?;
    let src_buf = BufReader::new(src_file);

    // Open the destination file and wrap it in a buffer
    let mut dest_file = OpenOptions::new()
        .write(true).create(true)
        .open([src, EXT_CIPHER].concat())?;
    let mut dest_buf = BufWriter::new(dest_file);

    // Open the key file and wrap it in a buffer
    let mut key_file = OpenOptions::new()
        .write(true).create(true)
        .open([src, EXT_KEY].concat())?;
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

/// Decrypts the source file with the key file. The unencrypted file will be
/// created (or overwritten) when this function returns and the `.vm`
/// extension will be removed (if it exists).
pub fn decrypt_file(src: &String, key: &String) -> Result<()> {
    // Open the source file and wrap it in a buffer
    let src_file = File::open(src)?;
    let src_buf = BufReader::new(src_file);

    // Open the key file and wrap it in a buffer
    let key_file = File::open(key)?;
    let key_buf = BufReader::new(key_file);

    // Open the destination file and wrap it in a buffer
    let mut dest_file = OpenOptions::new()
        .write(true).create(true)
        .open(src.trim_right_matches(EXT_CIPHER))?;
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
