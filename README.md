Advent of Code
===

My solutions to the problems posed by the Eric Wastl at [advent of code](https://adventofcode.com).

So far, nothing fancy here, invocation is by typing `python aoc<x>.py` at command line, each file printing the first then second answer to stdout. Challenges are grouped by year, and the data files are in the same directory as the src to keep things flat.

Tested on python 3.8.6 with no dependencies outside the standard library. Each year, and so far each day within each year, is entirely self-contained (except the special christmas message). Tests are discoverable with pytest.
Also working on an oxidised version: to run that, navigate to that year's directory and run the normal `cargo {run|test --lib}` commands. This version will *not* be depedency-free.