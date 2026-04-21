# Getting Started

## Running a Program

After installing FeOx, run a .fe file using the command line:
```bash
feox program.fe
```
or open the interactive REPL:
```bash
feox
```

## Example
Create a file called `example.fe`:
```feox
print((1..=10).sum());
```
and run it:
```bash
feox example.fe
```
Output:
```
55
```

This program creates a range from 1 to 10, computes the sum of all values, and prints the result.