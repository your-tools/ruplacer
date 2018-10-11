Ruplacer
=========

<a href="https://crates.io/crates/ruplacer"><img src="https://img.shields.io/crates/v/ruplacer.svg"/></a>


Replace text in files

Installation
-------------

Install `rust` and `cargo`, for example with [rustup](https://rustup.rs/)

Then run:

```
cargo install ruplacer
```

Usage
------

```
ruplacer pattern replacement [path]
```

If path is not given, it defaults to the current working directory.

By default, `pattern` will be compiled into a [Rust regex](https://docs.rs/regex/1.0.5/regex/).


Not that it's slightly different from Perl-style regular expressions. Also, it means you must use `$1`, `$2` to reference
groups captures in `pattern` in `replacement`.

For instance, to replace dates looking like `MM/DD/YYYY` to `YYYY-MM-DD`, you would use:

```
ruplacer '(\d{2})/(\d{2}/\d{4}' '$3-$1-$2'
```
