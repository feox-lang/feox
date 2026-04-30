# Basic Expressions

## Numbers

FeOx has two syntaxes for representing numbers:

- standard:
    ```
    10
    12
    67676767
    ```

- scientific:
    ```
    1e9
    5e9
    ```
!!! note
    There are no floats in FeOx.
    
---

## Truthiness

In FeOx, any value can be used in a boolean context:

- `0` and `nil` are falsy
- everything else is truthy

So `true` is just `1` and `false` is just `0`, but `if 5 {}` and `if "hello" {}` both work.

---

## Arithmetic

FeOx supports the standard arithmetic, bitwise, boolean, comparison, unary operators:

```feox
1 + 2;    // 3
10 - 3;   // 7
4 * 5;    // 20
10 / 3;   // 3 (floor division)
10 % 3;   // 1
2 ** 10;  // 1024
```

### Bitwise
```feox
10 & 12;  // 8
10 | 12;  // 14
10 ^ 12;  // 6
1 << 3;   // 8
16 >> 2;  // 4
```

### Boolean
```feox
true && false; // false
true || false; // true
!true;         // false
```

### Comparison
```
1 == 1;  // true
1 != 2;  // true
1 < 2;   // true
1 > 2;   // false
1 <= 1;  // true
1 >= 2;  // false
```

### Unary
```
-5; // -5
```

---

## Arrays

Arrays are ordered collections of values and can contain any type:

```feox
[1, 2, 3];
[1, "hello", nil];
```

### Indexing
```
[1, 2, 3][0]; // 1
let arr = [1, 2, 3];
arr[0] = 4;   // arr = [4, 2, 3]
```

### Appending and Extending
`+` behaves differently depending on the type of the right hand side:

```
arr + 4;     // [1, 2, 3, 4] (append)
arr + [4, 5]; // [1, 2, 3, 4, 5] (extend)
```
To append an array as a single element, wrap it or use push:
```
arr + [[4, 5]]; // [1, 2, 3, [4, 5]]
arr.push([4, 5]); // [1, 2, 3, [4, 5]]
```

### Length
```
len(arr);  // 3
arr.len(); // 3
```

---

## Strings and Characters

A string is just an array of characters:

```
"abc" == ['a', 'b', 'c']; // true
```

### Character Literals
```
'a' + 1;    // 'b'
'z' - 'a';  // 25
```

### String Comparison
```
"abc" < "abd";  // true
"abc" == "abc"; // true
```


!!! note
    Since strings are just arrays, all array operations work on strings too.

Next: [Variables and Control Flow](variables-and-control-flow.md)