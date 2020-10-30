# doggy_encrypt
## what is it?
A simple CLI application that can casually encrypt(encode actually) or decrypt(or decode) your text file.
- inspired by talk_with_buddha.
- written in rust.
## how to use?
### encrypt(or actually encode)
```bash
doggy_encrypt encrypt <file to encrypt>
```
⬆️️Doing so will straightly print encrypted text in your console.
So I recommend⬇️️:
```bash
touch output.txt
doggy_encrypt encrypt <file to encrypt> > output.txt
```
That will save all encrypted data in output.txt.
### decrypt(or actually decode)
```bash
doggy_encrypt decrypt <file to decrypt>
```