# Otp
Otp is a simple cryptographic command line tool offering perfect secrecy.
Encryption produces two files of equal size: a key file with the extension
`.key` and the encrypted file with the extension `.lock`. Decryption will
produce (or overwrite) the original file without the `.lock` extension.

## Usage
Encryption expects one file.

```sh
$ otp -e [FILE]
```

Decryption expects two files.

```sh
$ otp -d [FILE] [KEY]
```
