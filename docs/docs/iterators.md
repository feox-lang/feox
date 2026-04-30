# Iterators

In FeOx, iterators are just functions that take no arguments and return the next value, or `nil` when exhausted.

```feox
let iter = (1..=3);
iter(); // 1
iter(); // 2
iter(); // 3
iter(); // nil
```

This means you can create custom iterators with just a closure:
```feox
let always_one = || 1; // infinite iterator
```

## Creating Iterators
### Ranges
```feox
(1..10);   // 1 to 9
(1..=10);  // 1 to 10
```

### into_iter
```feox
[1, 2, 3].into_iter(); // iterator over array
"abc".into_iter();     // iterator over chars
```

## Transforming Iterators
### Map
```feox
(1..=5).map(|x| x * x); // 1 4 9 16 25
```
### Filter
```
(1..=10).filter(|x| x % 2 == 0); // 2 4 6 8 10
```
### Take
```
(1..=10).take(3); // 1 2 3
```
### Skip
```
(1..=5).skip(2); // 3 4 5
```
### Enumerate
```
(1..=3).enumerate(); // [0,1] [1,2] [2,3]
```
### Windows
```
(1..=5).windows(3); // [1,2,3] [2,3,4] [3,4,5]
```
### Flatten
```
[[1,2],[3,4]].into_iter().map(|x| x.into_iter()).flatten(); // 1 2 3 4
```
### Zip
```
zip((1..=3), (4..=6)); // [1,4] [2,5] [3,6]
```
### Chain
```
chain((1..=3), (4..=6)); // 1 2 3 4 5 6
```
## Consuming Iterators

### Collect
```
(1..=5).collect(); // [1, 2, 3, 4, 5]
```

### Sum
```
(1..=100).sum(); // 5050
```
### Fold
```
(1..=5).fold(|acc, x| acc * x, 1); // 120
```

### Any
```
(1..=5).any(); // true (all truthy)
```
### All
```
(0..=5).all(); // false
```

### Max
```
(1..=5).max(); // 5
```
### Min
```
(1..=5).min(); // 1
```