# Getting Started

## Installation
Download the latest release from:

[https://github.com/feox-lang/feox/releases](https://github.com/feox-lang/feox/releases)

## Your first program

Just open the terminal in the directory with `feox` binary downloaded and run:
```bash
feox
```
This will open an interactive REPL.

!!! important
    Remember to always end each line with a `;`

Try typing:
```
2 + 2;
```
```
4
```

Everything in FeOx is an expression and returns a value. Now try:
```
(1..=10).sum();
```
```
55
```

To run a file:
```bash
feox file.fe
```

Next: [Basic Expressions](basic-expressions.md)