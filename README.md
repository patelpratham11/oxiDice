[![crates.io](https://img.shields.io/crates/d/oxiDice.svg)](https://crates.io/crates/oxiDice)
![oxiDice Logo](/imgs/full.png)

# oxiDice

A simple CLI to generate passwords as either codes or phrases with customizable features to pick a password suited to your need. 

## Features

- Pass Code
    - Alphanumeric passcodes
    - Can add/remove numbers and/or special characters
- Pass Phrase
    - Based on the concept of diceware
    - Select words from 5 dice
- Entropy 
    - Calculates the associated entropy of the password generated
    - $E = log_{2}(R^L)$
        - E $\to$ Entropy of password
        - R $\to$ Number of *unique* characters
        - L $\to$ Length of the password generated
## Installation
1. Setup Rust
2. Navigate to whatever directory 
3. Install via Cargo
    ```
    cargo install oxiDice
    ```
4. See usage below!

## Usage

```
Usage: oxiDice [OPTIONS]

Options:
  -t, --type <type>            What would you like to generate [default: phrase] [possible values: code, phrase]
  -l, --length <length>        Specify the length of the passcode. [default: 10]
  -n, --numbers                Specify whether to use numbers [0-9] or not. [default: true]
  -s, --special                Specify whether to use special characters or not. [default: true]
  -w, --words <words>          Number of words to generate for diceware [default: 5]
  -d, --delimiter <delimiter>  Delimiter to use between words. [default: -]
  -#, --count <count>          Number of passes to generate. [default: 10]
  -h, --help                   Print help (see more with '--help')

```

