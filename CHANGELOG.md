# 0.2.7

* Fix deployment

# 0.2.6

* Improve README and `--help` message.

# 0.2.5

Deploy Windows pre-compiled binaries as assets.

# 0.2.4

* Rename `--fixed-strings` option to `--no-regex`

# 0.2.3

* Before exiting, print a helpful message containing stats about replacement and hint about using `--go` to actually write the changes to disk.

* Tweak ruplacer output

* Print error and exit with error code 2 if no replacement was made

# 0.2.2

* Implement --subvert option to handle snake_case, CamelCase and so on. Fix #8.

# 0.2.1

* In case binary or non-UTF-8 files are found in the path, just skip them instead of
  aborting the whole process

* Change algorithm used to display diffs. (Fix #4)

# 0.2.0

First public release
