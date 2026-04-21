# FeOx

---

# Literals

## Numbers

FeOx uses 64-bit integers.

Numbers may be written in two forms:

- Normal numbers  
  `123`

- Scientific notation  
  `2e9` → `2000000000`

---

## Strings

Strings are written using double quotes.

```
"hello"
```

Strings can be concatenated using the `+` operator.

```
"hello" + " " + "world"
```

Result:

```
"hello world"
```

---

## Chars

Chars are written using single quotes.

```
'a'
```

Strings are internally represented as arrays of chars.

---

## Arrays

Arrays are written using square brackets.

```
[1, 2, 3]
```

Array elements do not need to have the same type.

```
[1, "abc", true]
```

Array elements can be accessed by index.

```
arr[0]
```

---

## Nil

`nil` represents the absence of a value.

---

## Booleans

Boolean values are:

```
true
false
```

Booleans behave like numbers.

```
true  == 1
false == 0
```

---

# Variables

Variables are declared using the `let` keyword.

```
let x = 5;
let name = "FeOx";
```

Variables can store any value.

```
let a = 10;
let b = "hello";
let c = [1, 2, 3];
let d = |x| x * 2;
```

Variables can be reassigned.

```
let x = 5;
x = x + 1;
```

Array elements can be modified by index.

```
c[1] = 2;
```

Note:

You cannot declare a variable using an index.

Invalid example:

```
let c[0] = 1;
```

---

## Variable Names

Variable names must:

- start with a letter or `_`
- contain letters, numbers, or `_`

Examples:

```
x
value
_my_var
test123
```

---

# Operators

FeOx supports several types of operators:

- arithmetic
- comparison
- logical

Logical operators:

```
&&
||
```

These operators are **short-circuiting**.

This means the second expression is only evaluated if necessary.

Example:

```
a && b
```

`b` will only be evaluated if `a` is true.

---

# Functions

FeOx does not have named function declarations.

Instead, functions are created as anonymous lambdas and stored in variables.

Example:

```
let f = |x| x ** 2;
```

---

## Lambda Syntax

Lambdas use the following syntax:

```
|arg1, arg2, ...| expression
```

Example:

```
let add = |a, b| a + b;
```

### Block Lambdas

A lambda body can also be a block.

```
let i = 0;

let f = |x| {
    i = i + x;
    i
};
```

Example usage:

```
print(f(2)); // 2
print(f(3)); // 5
print(i);    // 5
```

Lambdas can capture and modify variables from outer scopes.

This behavior is known as **closure**.

---

## Methods

FeOx does not implement methods as a separate feature.

Method syntax is **syntactic sugar** for a normal function call.

```
obj.method(arg1, arg2)
```

is equivalent to:

```
method(obj, arg1, arg2)
```

Example:

```
arr.into_iter()
```

is equivalent to:

```
into_iter(arr)
```

---

## Iterators

FeOx provides a functional iterator system based on lambdas.

An **iterator** is a lambda that returns the next value in a sequence each time it is called.

When the sequence is exhausted, the iterator returns `nil`.

Example iterator usage:

```
let it = some_iterator();

it() -> value
it() -> value
it() -> nil
```

The `nil` value signals that no more elements are available.

### Creating Iterators

Functions in FeOx may return iterators.  
Such functions typically take some input and produce a lambda that generates values over time.

Example (conceptual):

```
let range = |start, end| {
    let i = start;

    | | {
        if i > end { nil }
        else {
            let v = i;
            i = i + 1;
            v
        }
    }
};
```

Usage:

```
let it = range(1, 3);

it() -> 1
it() -> 2
it() -> 3
it() -> nil
```

### Transforming Iterators

Functions may also accept an iterator and return another iterator.

These functions usually modify or transform the values produced by the original iterator.

Example:

```
let map = |it, f| {
    | | {
        let v = it();
        if v == nil { nil }
        else { f(v) }
    }
};
```

Usage:

```
let it = range(1, 3);
let doubled = map(it, |x| x * 2);

doubled() -> 2
doubled() -> 4
doubled() -> 6
doubled() -> nil
```

### Consuming Iterators

Some functions consume an iterator and repeatedly call it until it returns `nil`.

Example:

```
let collect = |it| {
    let result = [];

    while true {
        let v = it();
        if v == nil { break }
        push(result, v);
    }

    result
};
```

Usage:

```
let it = range(1, 3);
collect(it) -> [1, 2, 3]
```

### Iterator Contract

All iterators follow the same calling convention:

```
iterator() -> value
iterator() -> value
...
iterator() -> nil
```

Returning `nil` indicates that the iterator is exhausted.

---

## Modular Blocks

FeOx supports **modular blocks**, where all numeric values produced inside the block are taken modulo a specified value.

Syntax:

```
mod MODULUS {
    expression
}
```

All numeric results inside the block are reduced modulo `MODULUS`.

Example:

```
mod 1e9 + 9 {
    1e9 + 10
};
```

Result:

```
1
```

because:

```
(1000000000 + 10) mod (1000000000 + 9) = 1
```

### Behavior

Inside a modular block:

- Arithmetic results are taken modulo the given modulus.
- Division is interpreted as multiplication by a **modular inverse**.

Example:

```
mod 10 {
    7 + 8
};
```

Result:

```
5
```

### Modular Division

Division inside a modular block uses modular inverses.

```
a / b
```

is interpreted as:

```
a * inv(b)
```

where `inv(b)` is the modular inverse of `b` under the current modulus.

If the modular inverse does not exist, the operation results in an error.

### Scope

The modular rule applies only to expressions evaluated inside the block.

Values outside the block are not affected.

Example:

```
let x = mod 10 {
    7 + 8
};

x -> 5
```

Outside the block, normal arithmetic rules apply.


# Notes

- Lambdas are first-class values and can be stored in variables.