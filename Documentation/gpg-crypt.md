# Basics
Due to the current state of development automatic deletion 
after encryption or decryption is deactivated.
Here is an easy to understand example of how encryption
and decryption via GPG works:
```bash
❯ cybercraft --encrypt README.md
methode ==> gpg
keyname ==> foo@bar.foobar
File 'README.md.gpg' exists. Overwrite? (y/N) y
Encrypted: README.md.gpg
❯ rm README.md.gpg
❯ cybercraft --encrypt README.md
methode ==> gpg
keyname ==> foo@bar.foobar
Encrypted: README.md.gpg
❯ cybercraft --decrypt README.md.gpg
methode ==> gpg
gpg: encrypted with ELG key, ID 0x0000000000000000
gpg: anonymous recipient; trying secret key 0x70A62ED0F8A43068 ...
gpg: okay, we are the anonymous recipient.
File 'README.md' exists. Overwrite? (y/N) y
❯ ls
bookkeeping-venv  pre-commit_tools  README.md      uv.lock
main.py.gpg       pyproject.toml    README.md.gpg
❯ cat README.md
───────┬─────────────────────────────────────────────────────────────────────────────────
       │ File: README.md
───────┼─────────────────────────────────────────────────────────────────────────────────
   1 + │ HELLO CYBERCRAFT
   2 + │
   3 + │ If you can read this it worked
```
## WARNING
Also in the current state folder encryption is not implamented yet for GPG.

