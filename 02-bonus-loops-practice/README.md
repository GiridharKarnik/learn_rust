# Bonus: Loops Practice 🔁🚂

Six mini-challenges focused purely on loops. Each one is a small function to implement.
They go from easy to tricky. No new concepts — just reps to build muscle memory.

## Quick reference

```rust
// for loop with range (1 to 5 inclusive)
for i in 1..=5 { }

// for loop with range (0 to 4, exclusive end)
for i in 0..5 { }

// iterate an array
for item in array { }

// iterate with index
for (index, item) in array.iter().enumerate() { }

// while loop
while condition { }

// infinite loop with break
loop {
    if done { break; }
}

// loop returning a value
let result = loop {
    if done { break some_value; }
};

// skip an iteration
continue;

// exit the loop
break;
```

## Tips

- **Off-by-one errors** are the #1 loop bug. Double-check: should it be `<` or `<=`? `1..5` or `1..=5`?
- **Print before or after?** Think about whether you modify the counter before or after using it.
- **`continue` skips the rest of the loop body** — anything below it won't run for that iteration.
- **`break` exits immediately** — the loop body stops and execution continues after the loop.
- When you need to count things across iterations, declare a `let mut counter` **before** the loop.
