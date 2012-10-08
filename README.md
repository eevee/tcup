# tcup

`tcup` is a complete replacement for `git`.  That is, it can interact with an
existing git repository—or create a new one—without having `git` installed.

The intention is to have a simpler UI that helps avoid mistakes and explain
what's going on, while still benefiting from git's excellent design under the
hood.

## Building

`tcup` depends on:

* The [Rust](http://www.rust-lang.org/) compiler (0.4+)
* Some arbitrarily recent build of [libgit2](https://github.com/libgit2/libgit2)

`tcup` consists of two parts: Rust bindings for `libgit2`, and the `tcup`
executable itself.  You can build both merely by running `make`.

## Documentation

There's not much here yet, so there's not much to document, either.  See the
[GitHub wiki](https://github.com/eevee/tcup/wiki) for some planning docs.
