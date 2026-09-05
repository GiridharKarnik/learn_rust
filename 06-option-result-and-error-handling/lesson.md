# Lesson 06 — Option, Result & Error Handling

In Lesson 05, you learned how enums can carry data and how `match` lets you extract it.
Now we meet Rust's two most important enums: `Option<T>` and `Result<T, E>`. They handle
the two most common situations in programming: "maybe there's no value" and "this
operation can fail."

This is where Rust's safety story really clicks. Once you're fluent with `Option`,
`Result`, and the `?` operator, you'll wonder how you ever lived without them.

---

## 1. The Problem: Null is a Billion-Dollar Mistake

Every mainstream language has some version of "no value":

**JavaScript:**
```javascript
let train = findTrain(12345);
console.log(train.name);  // 💥 TypeError: Cannot read property 'name' of undefined
```

**Python:**
```python
train = find_train(12345)
print(train.name)  # 💥 AttributeError: 'NoneType' object has no attribute 'name'
```

**C:**
```c
Train* train = find_train(12345);
printf("%s", train->name);  // 💥 segfault — NULL pointer dereference
```

These all share the same flaw: **nothing forces you to check** whether you actually
got a value back. The code compiles (or runs) just fine, and the crash happens later,
in production, at 3 AM.

Tony Hoare, who invented null references in 1965, calls it his
["billion-dollar mistake."](https://en.wikipedia.org/wiki/Tony_Hoare#Apologies_and_retractions)

**Rust's answer:** make absence explicit. There is no null. If a value might not exist,
the type system says so — and the compiler forces you to handle that possibility before
you can use the value.

---

## 2. Option\<T\> — "Maybe there's a value"

Let's look at it properly.

It's just an enum:

```rust
// Defined in the standard library — you never write this yourself.
enum Option<T> {
    Some(T),   // there IS a value of type T
    None,      // there is NO value
}
```

`T` is a generic type parameter. Read `Option<T>` as "maybe a T":

- `Option<String>` → maybe a String
- `Option<u32>` → maybe a u32
- `Option<&Train>` → maybe a reference to a Train

### It's in the prelude — no import needed

`Option`, `Some`, and `None` are used so often that Rust puts them in the
*prelude* — automatically imported into every file. You write `Some(42)` and `None`
directly. No `use` statement, no `Option::Some(42)`.

### When to use Option

Use `Option` whenever an operation might not produce a value — but this isn't
an error, it's just a normal possibility:

- **Searching:** "Find a train with number 12345" — maybe there is one, maybe there isn't.
- **Looking up:** "What's the platform for this train?" — might not be assigned yet.
- **Accessing by index:** "Get the 5th element" — the list might have only 3 elements.
- **Optional fields:** "What's the delay reason?" — the train might not be delayed.

### Examples with railways

**Find a train by number:**

```rust
fn find_train(trains: &[Train], number: u32) -> Option<&Train> {
    for train in trains {
        if train.number == number {
            return Some(train);
        }
    }
    None  // searched everything, not found
}
```

**Get a station from a route:**

```rust
fn get_station_at(route: &[String], index: usize) -> Option<&String> {
    if index < route.len() {
        Some(&route[index])
    } else {
        None
    }
}
```

Note: you'd normally just use `route.get(index)` which returns `Option<&T>` — but
this shows the pattern.

---

## 3. Working with Option

This is where it gets practical. You have an `Option<T>`. How do you get the value
out? Here's your toolbox, from most explicit to most concise.

### `match` — most explicit, always works

```rust
let result = find_train(&trains, 12049);

match result {
    Some(train) => println!("Found: {}", train.name),
    None => println!("No train found"),
}
```

Use `match` when you need to do different things for `Some` and `None`. It's
exhaustive — the compiler makes sure you handle both cases.

### `if let Some(x) = ...` — when you only care about Some

```rust
if let Some(train) = find_train(&trains, 12049) {
    println!("Found: {}", train.name);
}
// If None — nothing happens, we move on
```

Use this when you want to act on `Some` but have nothing to do for `None`.

You can also add an `else`:

```rust
if let Some(train) = find_train(&trains, 12049) {
    println!("Found: {}", train.name);
} else {
    println!("Not found");
}
```

### `.unwrap()` — get the value or PANIC

```rust
let train = find_train(&trains, 12049).unwrap();
// If it's Some → gives you the inner value
// If it's None → PANICS! Your program crashes.
```

⚠️ **Avoid `.unwrap()` in real code.** It's only appropriate when:
- You're writing a quick prototype
- You've already verified the value exists
- A panic is genuinely the right response (very rare)

### `.expect("msg")` — like unwrap but with a custom panic message

```rust
let train = find_train(&trains, 12049)
    .expect("Train 12049 should exist in the schedule");
// Panics with YOUR message instead of a generic one
```

Better than `.unwrap()` because when it crashes, you get a meaningful error.
Still panics though — still avoid in production code.

### `.unwrap_or(default)` — get the value or use a default

```rust
let platform = get_platform(&train).unwrap_or(0);
// If Some(5) → 5
// If None → 0
```

No panic, no crash. You get a value either way.

### `.unwrap_or_else(|| ...)` — compute a default lazily

```rust
let name = find_train(&trains, 99999)
    .map(|t| t.name.clone())
    .unwrap_or_else(|| String::from("Unknown Train"));
```

The closure only runs if the value is `None`. Use this when computing the
default is expensive, or when you need to build a `String` or do other work.

### `.is_some()` / `.is_none()` — just check, don't extract

```rust
if find_train(&trains, 12049).is_some() {
    println!("Train exists!");
}

if find_train(&trains, 99999).is_none() {
    println!("Train not found");
}
```

Use these when you only need a boolean check.

### `.map(|x| ...)` — transform the inner value

```rust
let name: Option<String> = find_train(&trains, 12049)
    .map(|train| train.name.clone());
// If Some(train) → Some(train.name.clone())
// If None → None
```

`map` applies a function to the value inside `Some`, and passes `None` straight
through. It's like `Array.map()` in JavaScript, but for a single optional value.

### `.and_then(|x| ...)` — chain operations that also return Option

```rust
fn find_train(number: u32) -> Option<Train> { /* ... */ }
fn get_platform(train: &Train) -> Option<u32> { /* ... */ }

// Chaining: find a train, THEN get its platform
let platform: Option<u32> = find_train(12049)
    .as_ref()
    .and_then(|train| get_platform(train));
```

`and_then` is like `map`, but the function you pass also returns an `Option`.
Without `and_then`, you'd get `Option<Option<u32>>` — a nested Option. `and_then`
flattens it.

Think of it as JavaScript's optional chaining (`?.`) but for functions:
- JS: `findTrain(12049)?.getPlatform()`
- Rust: `find_train(12049).as_ref().and_then(|t| get_platform(t))`

### `.or(other_option)` — fallback to another Option

```rust
let primary = find_train(&trains, 12049);
let backup = find_train(&trains, 12050);

let train = primary.or(backup);
// If primary is Some → use it
// If primary is None → try backup
```

---

## 4. Result\<T, E\> — "It worked or it failed"

`Option` says "maybe a value." `Result` says "success or failure — and here's
*why* it failed":

```rust
// Also defined in the standard library
enum Result<T, E> {
    Ok(T),    // success — here's the value
    Err(E),   // failure — here's the error
}
```

Also in the prelude — `Result`, `Ok`, and `Err` are always available.

### When to use Result vs Option

- **`Option<T>`** — "This might not exist." Not finding something isn't an error,
  it's a normal possibility. (No train with that number, no available seat.)
- **`Result<T, E>`** — "This operation can fail." Something went wrong, and the
  caller needs to know *what*. (Parse error, invalid input, I/O failure.)

### Examples

**Parsing a string to a number:**

```rust
let input = "12049";
let number: Result<u32, _> = input.parse();

match number {
    Ok(n) => println!("Train number: {n}"),
    Err(e) => println!("Invalid input: {e}"),
}
```

**Validating input:**

```rust
fn validate_passenger_name(name: &str) -> Result<String, String> {
    if name.is_empty() {
        Err(String::from("Passenger name cannot be empty"))
    } else if name.len() < 2 {
        Err(String::from("Name must be at least 2 characters"))
    } else {
        Ok(name.to_string())
    }
}
```

**Custom error types:**

```rust
#[derive(Debug)]
enum TicketError {
    TrainNotFound,
    NoSeatsAvailable,
    InvalidPassenger(String),
}

fn book_ticket(train_number: u32, passenger: &str) -> Result<String, TicketError> {
    if passenger.is_empty() {
        return Err(TicketError::InvalidPassenger("Name cannot be empty".to_string()));
    }
    // ... more logic ...
    Ok(format!("Ticket booked for {} on train {}", passenger, train_number))
}
```

---

## 5. Working with Result

The methods on `Result` mirror those on `Option` — you already know the patterns.

### `match` on Ok/Err

```rust
match book_ticket(12049, "Giridhar") {
    Ok(confirmation) => println!("{confirmation}"),
    Err(TicketError::TrainNotFound) => println!("No such train"),
    Err(TicketError::NoSeatsAvailable) => println!("Train is full"),
    Err(TicketError::InvalidPassenger(msg)) => println!("Bad input: {msg}"),
}
```

### `if let Ok(x) = ...`

```rust
if let Ok(number) = "12049".parse::<u32>() {
    println!("Parsed: {number}");
}
```

### `.unwrap()` / `.expect()` — panic on Err

```rust
let n: u32 = "12049".parse().unwrap();           // panics if Err
let n: u32 = "12049".parse().expect("must parse"); // panics with message
```

Same rules as `Option`: avoid in production code.

### `.unwrap_or(default)`

```rust
let n: u32 = "not_a_number".parse().unwrap_or(0);
// Err → uses default value 0
```

### `.map(|x| ...)` — transform the Ok value

```rust
let result: Result<u32, _> = "42".parse::<u32>();
let doubled = result.map(|n| n * 2);
// Ok(42) → Ok(84)
// Err(e) → Err(e) (unchanged)
```

### `.map_err(|e| ...)` — transform the Err value

```rust
let result: Result<u32, _> = "abc".parse::<u32>();
let better_error = result.map_err(|e| format!("Failed to parse train number: {e}"));
// Err(ParseIntError) → Err("Failed to parse train number: invalid digit...")
```

This is useful for converting between error types.

### `.is_ok()` / `.is_err()`

```rust
if "42".parse::<u32>().is_ok() {
    println!("Valid number!");
}
```

---

## 6. The `?` Operator — Propagating Errors

Here's the problem: functions that call other fallible functions end up with
deeply nested matches:

```rust
fn process_booking(input: &str) -> Result<String, String> {
    let train_number = match input.parse::<u32>() {
        Ok(n) => n,
        Err(e) => return Err(format!("Bad number: {e}")),
    };

    let train = match find_train(train_number) {
        Some(t) => t,
        None => return Err(format!("Train {train_number} not found")),
    };

    let ticket = match book_ticket(train.number) {
        Ok(t) => t,
        Err(e) => return Err(format!("Booking failed: {e}")),
    };

    Ok(ticket)
}
```

That's a lot of ceremony. Every step is the same pattern: "if it failed, return
the error; otherwise, give me the value."

### Enter `?`

The `?` operator does exactly that pattern in one character:

```rust
fn process_booking(input: &str) -> Result<String, String> {
    let train_number: u32 = input.parse()
        .map_err(|e| format!("Bad number: {e}"))?;

    let train = find_train(train_number)
        .ok_or(format!("Train {train_number} not found"))?;

    let ticket = book_ticket(train.number)
        .map_err(|e| format!("Booking failed: {e}"))?;

    Ok(ticket)
}
```

What `?` does:
- If the value is `Ok(v)` → unwraps it, gives you `v`, and continues.
- If the value is `Err(e)` → returns `Err(e)` from the **current function** immediately.

### The rules

1. **The function must return `Result`** (or `Option`) for `?` to work.
2. The error type must be compatible — either the same type, or convertible via `From`.
3. You can chain multiple `?` operations for clean, linear error propagation.

### `?` with Option

The `?` operator also works with `Option`:

```rust
fn get_first_station(route: &[String]) -> Option<String> {
    let station = route.get(0)?;  // None → returns None from function
    Some(station.to_uppercase())
}
```

If the `Option` is `None`, `?` returns `None` from the function immediately.

### Chaining `?` — the real power

```rust
fn get_train_platform_display(
    trains: &[Train],
    number: u32,
) -> Result<String, String> {
    let train = find_train(trains, number)
        .ok_or(format!("Train {} not found", number))?;

    let platform = get_platform(train)
        .ok_or(format!("No platform assigned for {}", train.name))?;

    Ok(format!("{} departs from Platform {}", train.name, platform))
}
```

Each `?` is a potential early return, but the code reads top-to-bottom like a
script. No nesting, no rightward drift.

---

## 7. `if let` and `while let` — Concise Pattern Matching

Here's the full picture for working with `Option` and `Result`.

### `if let` — when you only care about one variant

```rust
// Instead of:
match find_train(&trains, 12049) {
    Some(train) => println!("Found: {}", train.name),
    None => {},  // do nothing — awkward
}

// Write:
if let Some(train) = find_train(&trains, 12049) {
    println!("Found: {}", train.name);
}
```

### `if let` with else

```rust
if let Ok(number) = "12049".parse::<u32>() {
    println!("Valid train number: {number}");
} else {
    println!("Invalid input");
}
```

### `while let` — loop until pattern fails

Great for processing queues, draining iterators, or popping from vectors:

```rust
let mut stations = vec!["Chennai", "Katpadi", "Bangalore", "Mysore"];

while let Some(station) = stations.pop() {
    println!("Now arriving at: {station}");
}
// Prints in reverse order (pop takes from the end)
// When the vec is empty, pop() returns None, and the loop ends.
```

### When to use `if let` vs `match`

- **1 variant** you care about → `if let`
- **2+ variants** you need to handle → `match`
- **Need exhaustiveness checking** → `match`

---

## 8. Converting Between Option and Result

Sometimes you have an `Option` but need a `Result`, or vice versa.

### `.ok_or("error msg")` — Option → Result

```rust
let platform: Option<u32> = Some(3);
let result: Result<u32, &str> = platform.ok_or("No platform assigned");
// Some(3) → Ok(3)
// None → Err("No platform assigned")
```

This is especially useful with `?`:

```rust
fn get_train_info(trains: &[Train], number: u32) -> Result<String, String> {
    let train = find_train(trains, number)
        .ok_or(format!("Train {} not found", number))?;  // Option → Result, then ?
    Ok(train.name.clone())
}
```

### `.ok_or_else(|| ...)` — lazy version

```rust
let train = find_train(&trains, number)
    .ok_or_else(|| format!("Train {} not found", number))?;
```

Use `ok_or_else` when building the error message is expensive (involves allocation
or computation). `ok_or` evaluates the error eagerly even if the Option is `Some`.

### `.ok()` — Result → Option (discards error)

```rust
let result: Result<u32, String> = Ok(42);
let option: Option<u32> = result.ok();
// Ok(42) → Some(42)
// Err(_) → None (error info is lost!)
```

Use this when you don't care *why* something failed, just whether it succeeded.

### `.transpose()` — Option\<Result\> ↔ Result\<Option\>

```rust
let x: Option<Result<u32, String>> = Some(Ok(42));
let y: Result<Option<u32>, String> = x.transpose();
// Some(Ok(42)) → Ok(Some(42))
// Some(Err(e)) → Err(e)
// None → Ok(None)
```

This is niche but comes up when you have a collection of optional operations
that can each fail.

---

## 9. Comparison with TypeScript

If you're coming from TypeScript, here's how the concepts map:

| TypeScript | Rust | Notes |
|---|---|---|
| `string \| undefined` | `Option<String>` | Explicit "maybe no value" |
| `value ?? default` | `.unwrap_or(default)` | Nullish coalescing |
| `value?.property` | `.map(\|v\| v.property)` | Optional chaining on properties |
| `value?.method()` | `.map(\|v\| v.method())` | Optional chaining on methods |
| `a?.b?.c` | `.and_then(\|a\| a.b).and_then(\|b\| b.c)` | Deep optional chaining |
| `try { ... } catch (e) { ... }` | `match result { Ok(v) => ..., Err(e) => ... }` | Error handling |
| `throw new Error(...)` | `Err(...)` | Creating an error |
| `async function` + `try/catch` | `Result<T, E>` + `?` | Error propagation |
| No equivalent | `?` operator | Auto-propagate — TS has nothing like this |

### The big difference

In TypeScript, error handling is **opt-in**. You can ignore errors, skip `try/catch`,
and the code still compiles. Errors surface at runtime, in production.

In Rust, error handling is **mandatory**. If a function returns `Result<T, E>`, you
**must** handle the `Err` case before you can use the `Ok` value. The compiler
refuses to compile until you do.

```rust
// This won't compile:
let name: String = find_train(&trains, 12049); // ❌ type mismatch: expected String, got Option<&Train>

// You MUST handle the Option:
let name: String = find_train(&trains, 12049)
    .map(|t| t.name.clone())
    .unwrap_or_else(|| String::from("Unknown"));  // ✅
```

---

## 10. Mental Model Summary

> **🧠 Key Concepts**
>
> 1. **Rust has no null.** If a value might be absent, the type system says so
>    with `Option<T>`. You cannot accidentally use a "null" value — the compiler
>    won't let you.
>
> 2. **`Option<T>`** = "maybe a value." `Some(v)` has one, `None` doesn't.
>    Use it for searching, lookups, optional fields — anywhere "not found" is
>    a normal possibility, not an error.
>
> 3. **`Result<T, E>`** = "success or failure." `Ok(v)` succeeded, `Err(e)` failed.
>    Use it for parsing, validation, I/O — anywhere an operation can fail and
>    the caller needs to know *why*.
>
> 4. **The `?` operator** propagates errors up the call stack in one character.
>    `Ok(v)` unwraps to `v`; `Err(e)` returns from the function immediately.
>    The function must return `Result` (or `Option`) for `?` to work.
>
> 5. **Your toolbox** for both `Option` and `Result`:
>    - `match` — handle every case explicitly
>    - `if let` — handle one case concisely
>    - `.unwrap()` / `.expect()` — get the value or panic (avoid in production)
>    - `.unwrap_or()` / `.unwrap_or_else()` — get the value or use a default
>    - `.map()` — transform the inner value
>    - `.and_then()` — chain operations that also return Option/Result
>    - `?` — propagate errors (Result) or absence (Option)
>
> 6. **Converting between them:**
>    - `Option` → `Result`: `.ok_or()` / `.ok_or_else()`
>    - `Result` → `Option`: `.ok()` (discards error)
>
> 7. **The core guarantee:** the compiler forces you to handle absence and failure
>    **before** you can use the value. No null pointer exceptions. No unhandled
>    errors. If it compiles, you dealt with it.

---

## Up Next

Run the examples in the `examples/` directory to see these concepts in action,
then tackle the exercise — a Railway Lost & Found System that puts `Option`,
`Result`, and `?` to work! 🚂
