# FeOx

FeOx is a lightweight interpreted programming language designed for fast numerical computation and algorithmic experimentation. It is not a general purpose language; instead it focuses on:

- integer and modular arithmetic
- combinatorics and sequence-based computations
- exhaustive search algorithms
FeOx prioritizes simplicity while providing all the necessary features needed for efficient problem solving.

---



## Installation
Download the latest release from:

[https://github.com/feox-lang/feox/releases](https://github.com/feox-lang/feox/releases)

---

## Getting Started
See the [Getting Started](getting-started.md)

---

## Examples

**Sum square difference**

```
(1..=100).sum() ** 2 - (1..=100).map(|x| x * x).sum();
```

**Multiples of 3 or 5**

```
(1..1000).filter(|x| (x % 3 == 0) | (x % 5 == 0)).sum();
```

Find the square root of `56480` modulo `1e9 + 9`

```
mod 1e9 + 9 {
	(1..=1e9).filter(|x| x * x == 56480).first();
};
```