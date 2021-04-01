<a href="#readme"><img src="https://tanker.io/images/github-logo.png" alt="Tanker logo" width="180" /></a>

[![crates.io image](https://img.shields.io/crates/v/ruplacer.svg)](https://crates.io/crates/ruplacer)
[![Build](https://img.shields.io/travis/TankerHQ/ruplacer.svg?branch=master)](https://travis-ci.org/TankerHQ/ruplacer)
[![Coverage](https://img.shields.io/codecov/c/github/TankerHQ/ruplacer.svg?label=Coverage)](https://codecov.io/gh/TankerHQ/ruplacer)

# Ruplacer

Find and replace text in source files:

```
$ ruplacer old new src/
Patching src/a_dir/sub/foo.txt
-- old is everywhere, old is old
++ new is everywhere, new is new

Patching src/top.txt
-- old is nice
++ new is nice

Would perform 2 replacements on 2 matching files.
Re-run ruplacer with --go to write these changes to disk
```


## Installing with cargo

Install `rust` and `cargo`, for example with [rustup](https://rustup.rs/).

Then run:

```
cargo install ruplacer
```

## Alternative installation methods

* Pre-compiled binaries for Linux, macOS, and Windows are available as [assets of the latest release](
https://github.com/TankerHQ/ruplacer/releases/tag/v0.4.3).

* `ruplacer` can also be installed from `homebrew`:

```
$ brew install TankerHQ/homebrew-repo/ruplacer
```

* `ruplacer` is also on [the Arch Linux User Repository](https://aur.archlinux.org/packages/ruplacer/)

## Basic usage

```
ruplacer pattern replacement [path]
```

If the path is not given, it defaults to the current working directory.

Ruplacer will then walk through every file in `<path>` while honoring `.gitignore` files found on the way.

Binary files and text files containing non-UTF8 characters will be skipped. Then for
every remaining file, it will read the contents, replace all lines matching the
pattern by the replacement, and print the difference.

If you are OK with the replacements, re-run `ruplacer` with the `--go` option to actually write the changes to disk.

## Regex

By default, `pattern` will be compiled into a [Rust regex](https://docs.rs/regex/1.0.5/regex/).

Note that it's slightly different from Perl-style regular expressions. Also, you must use `$1`, `$2` to reference
groups captured from `pattern` inside `replacement`.

For instance, this replaces 'last, first' by 'first last':

```
$ ruplacer '(\w+), (\w+)' '$2 $1'
```

(note the use of single quotes to avoid any processing by the shell)


If you don't want the pattern to be used as a regex, use the `--no-regex` command line flag.

This makes it possible to look for special characters without escaping them:

```
# This is a regex that matches the letter a
# or the letter o
$ ruplacer '(a|o)' u
- tata toto
+ tutu tutu
- (a|o)
+ (u|u)

# This is the literal string: '(a|o)'
$ ruplacer --no-regex '(a|o)' u
# or
$ ruplacer '\(a\|o|)' u
- (a|o)
+ u

```


## Subvert mode

Ruplacer has a `--subvert` option which works across a variety of case styles (lower case, snake case, and so on):

```
$ ruplacer --subvert foo_bar spam_eggs
Patching src/foo.txt
-- foo_bar, FooBar, and FOO_BAR!
++ spam_eggs, SpamEggs, and SPAM_EGGS!
```

## Filter files by type or glob patterns

Inspired by [ripgrep](https://github.com/BurntSushi/ripgrep), you can also select or ignore certain "file types" or glob patterns:

```
# Select only C++ files
$ ruplacer old new --type cpp
# Select only *.foo files
$ ruplacer old new --type *.foo
# Select only files that match foo*bar.c
$ ruplacer old new --type foo*bar.c

# Ignore all js files
$ ruplacer old new --type-not js
# Ignore all *.bar files
$ ruplacer old new --type-not *.bar
# Ignore all files that match foo*bar.c
$ ruplacer old new --type-not foo*bar.c
```

Each "file type" is just a list of glob pattern. For instance: the `cpp` file type matches `*.C`, `*.H`, `*.cc`, `*.cpp` and so on ...

You can see the whole list by using `ruplacer --file-types`.
