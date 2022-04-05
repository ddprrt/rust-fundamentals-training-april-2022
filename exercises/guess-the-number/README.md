# Exercise: Guess the number game

Write a little command line application that creates a number from 0 to 50, and you have to guess which one it is. Your output is either lower, higher, or found. End after the number has been found.

## Goals

You learn:
- Reading from `stdin`
- Loops
- Conditionals

## Material you need

### Random numbers

The `rand` crate. Use this to generate random numbers. https://crates.io/crates/rand

Add the crate to your `Cargo.toml`

Import the crate:

```rs
use rand::{thread_rng, Rng};
```

use the crate

```rs
let number_to_guess = rng.gen_range(0..=50)
```

### Reading from stdin

Import io

```rs
use std::io;
```

Read into a buffer

```rs
let mut buffer = String::new();
let _read_bytes = io::stdin().read_line(&mut buffer).unwrap()
```

### Parsing

```rs
let guessed_number: u32 = buffer.trim().parse().unwrap()
```

## Stretch goals

1. Make your CL app configurable:
    2. Add a CL argument for the max number
    3. Add a CL argument for a maximum number of tries

You need `std::env` and `let args: Vec<String> = env::args().collect();` to get to those numbers