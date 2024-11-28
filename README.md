# Advent of Code 2024

My solutions to (some) of the Advent of Code (AoC) problems for this year.

## Using as a template
Feel free to use this repository as a template. For that just clone the repository and remove all `dayx` directories:

```
git clone https://github.com/michihupf/aoc24
cd aoc24
rm -rf day*
```

You can the use the justfile to create a day project. To create the project for day 3 for example run:

```
just create day3
```

This will create the `day3` binary project and curl the input from `https://adventofcode.com/2024/day/3/input`.
To curl the input please place your AoC session key in `~/.aocrc` (it is stored in a cookie in your browser).
The session key is required for this process as the input is different for every user.
If curling the input is not needed you can also just ~~remove~~ comment out the curl line in the `create` recipe inside the `justfile`.
