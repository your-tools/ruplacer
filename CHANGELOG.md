# 0.2.3

Before exiting, print a helpful message containing stats about replacement and hint about
using `--go` to actually write the changes to disk.


# 0.2.2

* Implement --subvert option to handle snake_case, CamelCase and so on. Fix #8.


# 0.2.1

* In case binary or non-UTF-8 files are found in the path, just skip them instead of
  aborting the whole process

* Change algorithm used to display diffs. (Fix #4)

# 0.2.0

First public release
