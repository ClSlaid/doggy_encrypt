# doggy_encrypt
## What is it?
A simple CLI application that can casually encrypt(encode actually) or decrypt(or decode) your text file.
- Inspired by talk_with_buddha.
- Written in rust.
## How to use?
### Install rust language environment.
1. Install `rustup`.
2. Run `cargo build --release`.
3. Find `doggy` in `./target/release/`.
4. Run `doggy`.
### Encrypt(or actually encode)
```bash
doggy encrypt <file to encrypt>
```
⬆️️ Doing so will straightly print encrypted text in your console.

⬇️️ So I recommend:
```bash
touch output.txt
doggy encrypt <file to encrypt> > output.txt
```
That will save all encrypted data in output.txt.
### Decrypt(or actually decode)
```bash
doggy decrypt <file to decrypt>
```
