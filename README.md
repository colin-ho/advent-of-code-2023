<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2023

My solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/). First year completing all 25 days of AOC, albeit a bit late and with some help from Reddit and Youtube 🙃. Learned a lot about Rust, Shoelace/Picks theorem, MinCut, LCM applications, and more.

Template provided by https://github.com/fspoettel/advent-of-code-rust. 

<!--- advent_readme_stars table --->
## 2023 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2023/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2023/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2023/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2023/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2023/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2023/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2023/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2023/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2023/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2023/day/10) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2023/day/11) | ⭐ | ⭐ |
| [Day 12](https://adventofcode.com/2023/day/12) | ⭐ | ⭐ |
| [Day 13](https://adventofcode.com/2023/day/13) | ⭐ | ⭐ |
| [Day 14](https://adventofcode.com/2023/day/14) | ⭐ | ⭐ |
| [Day 15](https://adventofcode.com/2023/day/15) | ⭐ | ⭐ |
| [Day 16](https://adventofcode.com/2023/day/16) | ⭐ | ⭐ |
| [Day 17](https://adventofcode.com/2023/day/17) | ⭐ | ⭐ |
| [Day 18](https://adventofcode.com/2023/day/18) | ⭐ | ⭐ |
| [Day 19](https://adventofcode.com/2023/day/19) | ⭐ | ⭐ |
| [Day 20](https://adventofcode.com/2023/day/20) | ⭐ | ⭐ |
| [Day 21](https://adventofcode.com/2023/day/21) | ⭐ | ⭐ |
| [Day 22](https://adventofcode.com/2023/day/22) | ⭐ | ⭐ |
| [Day 23](https://adventofcode.com/2023/day/23) | ⭐ | ⭐ |
| [Day 24](https://adventofcode.com/2023/day/24) | ⭐ | ⭐ |
| [Day 25](https://adventofcode.com/2023/day/25) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `34.7µs` | `111.6µs` |
| [Day 2](./src/bin/02.rs) | `38.9µs` | `43.1µs` |
| [Day 3](./src/bin/03.rs) | `113.9µs` | `115.1µs` |
| [Day 4](./src/bin/04.rs) | `117.4µs` | `113.0µs` |
| [Day 5](./src/bin/05.rs) | `20.3µs` | `24.1µs` |
| [Day 6](./src/bin/06.rs) | `191.0ns` | `286.0ns` |
| [Day 7](./src/bin/07.rs) | `123.3µs` | `122.1µs` |
| [Day 8](./src/bin/08.rs) | `423.5µs` | `2.9ms` |
| [Day 9](./src/bin/09.rs) | `230.4µs` | `224.0µs` |
| [Day 10](./src/bin/10.rs) | `788.6µs` | `1.3ms` |
| [Day 11](./src/bin/11.rs) | `636.8µs` | `638.4µs` |
| [Day 12](./src/bin/12.rs) | `397.7µs` | `1.2ms` |
| [Day 13](./src/bin/13.rs) | `207.5µs` | `199.7µs` |
| [Day 14](./src/bin/14.rs) | `24.9µs` | `18.1ms` |
| [Day 15](./src/bin/15.rs) | `65.0µs` | `234.9µs` |
| [Day 16](./src/bin/16.rs) | `1.3ms` | `43.4ms` |
| [Day 17](./src/bin/17.rs) | `38.5ms` | `62.5ms` |
| [Day 18](./src/bin/18.rs) | `35.5µs` | `53.0µs` |
| [Day 19](./src/bin/19.rs) | `418.8µs` | `590.9µs` |
| [Day 20](./src/bin/20.rs) | `12.6ms` | `52.5ms` |
| [Day 21](./src/bin/21.rs) | `7.6ms` | `22.3ms` |
| [Day 22](./src/bin/22.rs) | `1.3ms` | `11.9ms` |
| [Day 23](./src/bin/23.rs) | `3.7ms` | `1.1s` |
| [Day 24](./src/bin/24.rs) | `388.6µs` | `47.7µs` |
| [Day 25](./src/bin/25.rs) | `336.0ms` | `-` |

**Total: 1723.68ms**
<!--- benchmarking table --->

---


## Usage

### ➡️ Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs the solution against real puzzle inputs. To run an optimized build, append the `--release` flag as with any other rust program.

### ➡️ Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line. Same as for the `solve` command, the `--release` flag runs an optimized build.

### ➡️ Benchmark solutions

```sh
# example: `cargo time 8 --store`
cargo time <day> [--all] [--store]

# output:
# Day 08
# ------
# Part 1: 1 (39.0ns @ 10000 samples)
# Part 2: 2 (39.0ns @ 10000 samples)
#
# Total (Run): 0.00ms
#
# Stored updated benchmarks.
```

The `cargo time` command allows you to benchmark and store timings in the readme. When benching, the runner will run the code between `10` and `10.000` times, depending on execution time of first execution, and print the average execution time.

`cargo time` has three modes of execution:

 1. `cargo time` without arguments incrementally benches solutions that do not have been stored in the readme yet and skips the rest.
 2. `cargo time <day>` benches a single solution.
 3. `cargo time --all` benches all solutions.

By default, `cargo time` does not write to the readme. In order to do so, append the `--store` flag: `cargo time --store`.

> Please note that these are not _scientific_ benchmarks, understand them as a fun approximation. 😉 Timings, especially in the microseconds range, might change a bit between invocations.