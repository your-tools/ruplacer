# Ruplacer

<a href="https://crates.io/crates/ruplacer"><img src="https://img.shields.io/crates/v/ruplacer.svg"/></a>
[![Build](https://img.shields.io/travis/SuperTanker/ruplacer.svg?branch=master)](https://travis-ci.org/SuperTanker/ruplacer)


Find and replace text in source files.

![ruplacer screenshot](https://dmerej.info/blog/pics/ruplacer.png)

## Installing with cargo

Install `rust` and `cargo`, for example with [rustup](https://rustup.rs/).

Then run:

```
cargo install ruplacer
```

## Alternative installation methods

* Pre-compiled binaries for Linux, macOS and Windows are available as [assests of the latest release](
https://github.com/SuperTanker/ruplacer/releases/tag/v0.2.4).

* `ruplacer` can also be installed from `homebrew`:

```
$ brew install supertanker/homebrew-repo/ruplacer
```

* `ruplacer` is also on [the Arch Linux User Repository](https://aur.archlinux.org/packages/ruplacer/)

## Basic usage

```
ruplacer pattern replacement [path]
```

If path is not given, it defaults to the current working directory.

Ruplacer will then walk through every file in `<path>`, while honoring `.gitignore` files found on the way.

Binary files and text files containing non-UTF8 characters will be skipped. Then for
every remaining file, it will read the contents, replace all lines matching the
pattern by the replacement, and print the difference:

```
$ replacer old new src/
Patching src/a_dir/sub/foo.txt
-- old is everywhere, old is old
++ new is everywhere, new is new

Patching src/top.txt
-- old is nice
++ new is nice
```

If you are OK with the replacements, re-run `ruplacer` with the `--go` option to actually write the files.

By default, `pattern` will be compiled into a [Rust regex](https://docs.rs/regex/1.0.5/regex/).

Note that it's slightly different from Perl-style regular expressions. Also, you must use `$1`, `$2` to reference
groups captures in `pattern` in `replacement`.

For instance, to replace dates looking like `MM/DD/YYYY` to `YYYY-MM-DD`, you would use:

```
$ ruplacer '(\d{2})/(\d{2})/(\d{4})' '$3-$1-$2'
```

## Customizing the replacement algorithm

* Use `--no-regex` to prevent `ruplacer` from interpreting the pattern as a regex.
* Use `--subvert` to perform replacements across a variety of case styles:

```
$ ruplacer --subvert foo_bar spam_eggs
Patching src/foo.txt
-- foo_bar, FooBar, and FOO_BAR!
++ spam_eggs, SpamEggs, and SPAM_EGGS!
```
