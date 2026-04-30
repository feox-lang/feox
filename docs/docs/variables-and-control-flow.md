# Variables and Control Flow

## Variables

```feox
let a = 0;
a = 10;
```

Variables must always have an initial value. There is no declaration without assignment.
To access a variable just write its name: 
```
let b = a;
```

!!! important
    FeOx currently doesnt support references. That means when using a variable a copy is created.

---

## If / Else
```
if a > 0 {
    1
} else {
    -1
};
```

`if/else` is an expression and returns the last expression of the matching branch:

```
let sign = if a > 0 { 1 } else { -1 };
```

For else if, nest:
```
if a > 0 {
    1
} else {
    if a < 0 { -1 } else { 0 }
};
```

---

## For

```
for x in (1..=10) {
    print(x);
};
```

```
for x in [1, 2, 3].into_iter() {
    print(x);
};
```

!!! note
    You cannot iterate over an array directly. Use .into_iter() to convert it first.  

---

## While
```
while a > 0 {
    a = a - 1;
};
```
---

## Break and Continue
```
while true {
    if a > 10 { break };
    a = a + 1;
};
```

`for` and `while` both return `nil`.



Next: [Functions and Closures](functions-and-closures.md)