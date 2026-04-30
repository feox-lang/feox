# Functions and Closures

## Defining Functions

In FeOx, functions are just closures assigned to variables:

```feox
let f = || 2;
let square = |x| x * x;
```

--- 
## Calling Functions
```
f();        // 2
square(3);  // 9
```
---
## Multi-line Closures
```
let f = |x| {
    let y = x * x;
    y + 1
};
```

---
## Return
```
let f = |x| {
    if x > 0 { return x };
    -x
};
```

---
## Dynamic Scoping

FeOx uses dynamic scoping. This means closures see variables from the scope they are **called** in, not where they are **defined**:

```
let g = || x;

let f = || {
    let x = 2;
    g();
};

f(); // 2
```
!!! warning
    This is different from most languages. Closures do not capture variables at definition time.
---
## Methods

FeOx uses normal functions as methods. `obj.f(args) == f(obj, args)`

```
let double = |arr| arr.into_iter().map(|x| x * 2).collect();
[1, 2, 3].double(); // [2, 4, 6]
```

!!! warning
    This is different from most langauges.

Next: [Iterators](iterators.md)