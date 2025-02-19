Advent of Code
===

My solutions to the problems posed by the Eric Wastl at [advent of code](https://adventofcode.com). Currently either in Python or Rust.

So far, nothing fancy here. For python solutions, invocation is by typing `python aoc<x>.py` at command line, each file printing the first then second answer to stdout. Challenges are grouped by year, and the data files are in the same directory as the src to keep things flat. For rust solutions, edit src/main.rs at build time and invoke `cargo build`. (Yes, I could add a switch or workspaces, but that's not a priority).

For python, development used >=3.8.6 with no dependencies outside the standard library and numpy. For rust, a slightly less minimal approach is used but it's still kept to common utilities (e.g. regex) plus ndarray.

Tests are discoverable with pytest and cargo test.
