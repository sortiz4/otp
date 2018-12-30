# Vernam
Vernam is a simple cryptographic command line utility offering perfect secrecy.
Encryption produces two files of equal size: a key file with the extension
`.key` and the encrypted file with the extension `.vm`. Decryption will
produce (or overwrite) the original file without the `.vm` extension.

## Usage

```
A simple Vernam Cipher implementation.

Usage:
    vernam [OPTIONS] [FILE] [KEY]

Options:
    -d, --decrypt       Decrypt the file with the key
    -e, --encrypt       Encrypt the file with a random key
    -h, --help          Output this message
    -V, --version       Output version information
```
