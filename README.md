# entrust

Entrust is a CLI password manager inspired by and semi-compatible with [pass](https://www.passwordstore.org).
It saves passwords as files encrypted with either [gpg](https://gnupg.org) or [age](https://github.com/FiloSottile/age).

It is developed primarily for educational purposes, to learn a bit of Rust and play around with CLI and TUI libraries.
Though it should be just about serviceable, I do not recommend anyone actually use it.

## Installation

Currently only via cargo:

```shell
cargo install --locked entrust
```

## Basic usage

```shell
# add entries
ent add something/username
ent add something/password

# print an entry
ent get something/password

# interactively select an entry to print
ent get

# copy an entry to the clipboard
ent get -c something/password

# autotype into the previously active window
ent autotype 'something/username:{tab}:something/password:{enter}'
```
