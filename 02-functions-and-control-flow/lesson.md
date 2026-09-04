# Lesson 02 — Functions & Control Flow

## 1. Functions

Functions are declared with `fn`. Rust uses **snake_case** for function names.

```rust
fn announce_departure() {
    println!("The train is now departing!");
}
```

### Parameters

Specify the type of every parameter — Rust never infers parameter types:

```rust
fn announce_train(name: &str, platform: u32) {
    println!("{name} is at platform {platform}");
}
```

### Return Values

Use `->` to declare the return type. The **last expression** (without a semicolon)
is the return value:

```rust
fn ticket_price(distance_km: f64) -> f64 {
    distance_km * 0.75
}
```

Notice: **no semicolon** on the last line, and **no `return` keyword**. This is idiomatic
Rust. The last expression *is* the return value.

You *can* use `return` for early exits:

```rust
fn ticket_price(distance_km: f64) -> f64 {
    if distance_km <= 0.0 {
        return 0.0;  // early return — needs semicolon
    }
    distance_km * 0.75  // implicit return — no semicolon
}
```

> **Expressions vs Statements:**
> - An *expression* produces a value: `5 + 3`, `distance * 0.75`, `if x { 1 } else { 2 }`
> - A *statement* performs an action but returns nothing: `let x = 5;`, `println!(...);`
> - Adding a semicolon to an expression turns it into a statement (and discards the value!)
> - This is why `distance * 0.75` returns the value, but `distance * 0.75;` returns nothing

---

## 2. `if` / `else`

Standard conditional branching. **No parentheses** needed around the condition (unlike C/Java):

```rust
let delay = 15;

if delay == 0 {
    println!("On time!");
} else if delay < 10 {
    println!("Slightly delayed");
} else {
    println!("Significantly delayed");
}
```

### `if` as an expression

In Rust, `if` returns a value — you can use it on the right side of `let`:

```rust
let status = if delay == 0 { "On Time" } else { "Delayed" };
```

Both branches must return the **same type**.

---

## 3. `loop` — infinite loop

Runs forever until you `break`:

```rust
let mut countdown = 5;
loop {
    if countdown == 0 {
        println!("🚂 Departed!");
        break;
    }
    println!("{countdown}...");
    countdown -= 1;
}
```

`loop` can also return a value via `break`:

```rust
let mut tries = 0;
let result = loop {
    tries += 1;
    if tries == 3 {
        break "connected";  // this value is returned from the loop
    }
};
println!("Status: {result} after {tries} tries");
```

---

## 4. `while` — conditional loop

Loops while a condition is true:

```rust
let mut speed = 0;
while speed < 120 {
    speed += 20;
    println!("Accelerating... {speed} km/h");
}
println!("Cruising speed reached!");
```

---

## 5. `for` — iterating

The workhorse loop in Rust. Used with **ranges** and **iterators**:

```rust
// Range: 1 to 5 (inclusive start, exclusive end)
for platform in 1..6 {
    println!("Checking platform {platform}");
}

// Inclusive range: 1 to 5
for platform in 1..=5 {
    println!("Checking platform {platform}");
}

// Iterating over an array
let stations = ["Chennai", "Katpadi", "Bangalore"];
for station in stations {
    println!("Stopping at: {station}");
}
```

### `for` with index using `.enumerate()`

```rust
let coaches = ["S1", "S2", "A1", "B1", "B2"];
for (index, coach) in coaches.iter().enumerate() {
    println!("Position {}: Coach {}", index + 1, coach);
}
```

---

## 6. `match` — pattern matching

Think of it as a supercharged `switch`. Rust ensures you cover **every** possibility:

```rust
let platform = 3;

match platform {
    1 => println!("Rajdhani Express"),
    2 => println!("Shatabdi Express"),
    3 => println!("Duronto Express"),
    _ => println!("Local service"),    // _ is the catch-all (like "default")
}
```

### `match` with ranges and multiple values

```rust
let coach_number = 5;

match coach_number {
    1 | 2 => println!("First Class"),          // 1 or 2
    3..=5 => println!("AC Chair Car"),          // 3, 4, or 5
    6..=12 => println!("Sleeper"),              // 6 through 12
    _ => println!("General"),
}
```

### `match` as an expression (returns a value)

```rust
let delay_min = 25;

let announcement = match delay_min {
    0 => "On time",
    1..=10 => "Slight delay",
    11..=30 => "Delayed",
    _ => "Severely delayed",
};

println!("Status: {announcement}");
```

---

## 7. `break` and `continue`

- `break` — exit the loop
- `continue` — skip to the next iteration

```rust
for train_id in 1..=10 {
    if train_id == 5 {
        continue;  // skip train 5 (cancelled)
    }
    if train_id == 8 {
        break;     // stop processing after train 7
    }
    println!("Dispatching train {train_id}");
}
```

---

## Key Takeaways

1. Functions use `fn`, always annotate parameter types, return with the last expression (no semicolon)
2. `if/else` doesn't need parentheses and can be used as an expression
3. `for` with ranges (`1..5`, `1..=5`) is the most common loop
4. `match` is powerful — Rust forces you to handle all cases
5. Semicolons matter! They're the difference between returning a value and not

---

## Up Next

Run the examples, then tackle the exercise — you'll be building a **train dispatch system**! 🚂
