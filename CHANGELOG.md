# 0.6.4 (2022-02-28)

* Fix bug where the printed number of matching files was generally too big.
* Improve documentation for `FilePatcher`

# 0.6.3 (2022-02-02)

* Update dependencies
* Bump to Rust 2021 edition

# 0.6.2 (2021-07-11)

* Fix regression in ruplacer diff output introduced in 0.6.1.

# 0.6.1 (2021-06-08)

* Handle trailing newlines consistently. Previously, `ruplacer` would
  always write file with a trailing new line. Patch by @LawnGnome.

# 0.6.0 (2021-05-15)

## Bug fixes

* Fix panic when using incorrect globs for file and type selections.

## New features

* Also replace `Ada_Case` (also known as `Mixed_Case`) variants when using `--subvert`

## New output

The output has changed, going from:
```
# Using version < 0.6
Patching foo/bar.js
--- old is old
+++ new is new
```

to:

```
# Using version >= 0.6
foo/bar.js:3 - old is old
foo/bar.js:3 + new is new
```
* ruplacer now displays the path and line number of each line that changed
* the coloring of patches is more precise. See #15 for details.
* ruplacer reports the total number of replacements, rather than the
  number of lines that changed

## Internal changes

* Drop dependency on the `difference` crate
* Improve public API
* Default branch is now called `main`

# 0.5.0 (2020-05-09)

## New features


* Add support for glob pattern for the `-t, type` and `-T, --type-not`
  options. Patch by @ndtoan96

* Implement `--hidden` and `--ignored` flags, to force patching of
  hidden and ignored files, respectively.

* If the last argument is `-`, read from stdin and write to stdout.

## Internals

* Switch to 2018 edition
* Switch to GitHub Actions for CI
* Switch to `anyhow` for error handling
* Move out of TankerHQ GitHub organization

# 0.4.3 (2020-05-13)

* Bump smallvec

# 0.4.2 (2020-05-13)

* Fix metadata in Cargo.toml

# 0.4.1 (2019-03-29)

* Fix release scripts

# 0.4.0 (2019-03-29)

* Add `-w, --word-regex` to match regex only inside words. Note that
  `ruplacer -w old new` is *exactly* the same as `ruplacer '\bold\b' new`.

# 0.3.0 (2018-12-05)

* Implement #18: Add `-t, --type`, `-T, --type-not` and `--type-list` options. Suggested by @Dowwie.

# 0.2.7 (2018-11-20)

* Fix deployment from travis

# 0.2.6 (2018-11-20)

* Improve README and `--help` message.

# 0.2.5 (2018-10-19)

Deploy Windows pre-compiled binaries as assets.

# 0.2.4 (2018-10-16)

* Rename `--fixed-strings` option to `--no-regex`

# 0.2.3 (2018-10-15)

* Before exiting, print a helpful message containing stats about replacement and hint about using `--go` to actually write the changes to disk.

* Tweak ruplacer output

* Print error and exit with error code 2 if no replacement was made

# 0.2.2 (2018-10-12)

* Implement --subvert option to handle snake_case, CamelCase and so on. Fix #8.

# 0.2.1 (2018-10-09)

* In case binary or non-UTF-8 files are found in the path, just skip them instead of
  aborting the whole process

* Change algorithm used to display diffs. (Fix #4)

# 0.2.0 (2018-10-09)

First public release
