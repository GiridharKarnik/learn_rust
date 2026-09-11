# Lesson 13: Error Handling with `thiserror`

## 1. The Problem

You have a function that can fail in more than one way:

```rust
fn find_and_validate(trains: &[Train], number: u32) -> Result<&Train, ???> {
    // might fail: train not found
    // might fail: invalid train number (zero, too large, etc.)
}
```

In TypeScript you'd just `throw` — any error, any time. Callers `catch`
whatever they get and hope for the best.

In Rust, `Result<T, E>` needs **one** concrete type for `E`. You can't return
a "not found" error on line 3 and a different "invalid number" error on line 5
unless both are the **same type**.

So how do you unify multiple failure modes into one type? **An enum.**

---

## 2. The Manual Way (and Why It's Painful)

You can define an error enum and implement `Display` yourself:

```rust
#[derive(Debug)]
enum TrainError {
    NotFound(u32),
    InvalidName(String),
    NoSeats(CoachClass),
}

impl std::fmt::Display for TrainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrainError::NotFound(n) => write!(f, "Train #{n} not found"),
            TrainError::InvalidName(s) => write!(f, "Invalid name: {s}"),
            TrainError::NoSeats(c) => write!(f, "No seats in {c:?}"),
        }
    }
}

impl std::error::Error for TrainError {}
```

That's 18 lines of boilerplate for 3 variants. Every time you add a variant
you update the `match` arm too. It gets old fast.

---

## 3. `thiserror` Makes It Easy

The `thiserror` crate generates all that boilerplate from a single derive:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum TrainError {
    #[error("Train #{0} not found")]
    NotFound(u32),

    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("No seats in {0:?}")]
    NoSeats(CoachClass),
}
```

That's it. Three things to remember:

- **`#[derive(Debug, Error)]`** on the enum — tells thiserror to generate
  `Display` and `std::error::Error` for you.
- **`#[error("...")]`** on each variant — this becomes the human-readable
  error message (what prints when you do `println!("{e}")`).
- **`{0}`** inside the string — refers to the first field, just like
  `format!()`. Named fields work too: `{name}`, `{position}`. Use `{0:?}`
  for Debug formatting.

---

## 4. Using Your Custom Error

### Returning errors from functions

```rust
fn find_train<'a>(trains: &'a [Train], number: u32) -> Result<&'a Train, TrainError> {
    trains
        .iter()
        .find(|t| t.number == number)
        .ok_or(TrainError::NotFound(number))
}

fn validate_name(name: &str) -> Result<(), TrainError> {
    if name.is_empty() {
        return Err(TrainError::InvalidName(name.to_string()));
    }
    Ok(())
}
```

### Matching on errors

```rust
match find_train(&trains, 999) {
    Ok(t) => println!("Found: {}", t.name),
    Err(TrainError::NotFound(n)) => println!("No train #{n}"),
    Err(e) => println!("Other error: {e}"),
}
```

Because errors are enum variants, the compiler can check that you've
handled each case. This is the big upgrade over TypeScript's `catch(e: unknown)`.

---

## 5. `#[from]` — Auto-Converting Other Errors

When you call a library function that returns its own error type, you need a
way to convert it into *your* error type. `#[from]` does this automatically:

```rust
#[derive(Debug, Error)]
enum TrainError {
    #[error("Train #{0} not found")]
    NotFound(u32),

    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
```

The `#[from]` attribute generates an `impl From<serde_json::Error> for TrainError`,
which means the `?` operator can now convert it for you:

```rust
fn load_trains(data: &str) -> Result<Vec<Train>, TrainError> {
    let trains: Vec<Train> = serde_json::from_str(data)?;  // auto-converts!
    Ok(trains)
}
```

Without `#[from]`, that `?` would be a compile error because Rust wouldn't
know how to turn a `serde_json::Error` into a `TrainError`.

---

## 6. Chaining with `?`

This is where it all comes together. Each `?` is an early-exit point — if
the call fails, the error converts and returns automatically. The happy path
reads like a straight line:

```rust
fn book(
    trains: &[Train],
    number: u32,
    passenger: &str,
) -> Result<Booking, TrainError> {
    let train = find_train(trains, number)?;   // might return NotFound
    validate_name(passenger)?;                  // might return InvalidName
    let seat = train.reserve_seat()?;           // might return NoSeats

    Ok(Booking {
        train: train.number,
        passenger: passenger.to_string(),
        seat,
    })
}
```

No nesting, no callbacks, no try/catch. If any step fails, the function
exits with the right error variant. If everything succeeds, you get a `Booking`.

---

## 7. `thiserror` vs `anyhow`

You'll see two popular error crates in Rust:

- **`thiserror`** — Define your own error types. Callers can `match` on
  specific variants. Use this when other code needs to react differently to
  different errors.
- **`anyhow`** — Propagate *any* error without defining custom types. Great
  for `main()`, scripts, and prototyping where you just want errors to
  bubble up with a message.

You can use both in the same project: `thiserror` in your library modules,
`anyhow` in `main.rs`.

---

## 8. Comparison with TypeScript

| Concept           | TypeScript                    | Rust + thiserror                     |
|-------------------|-------------------------------|--------------------------------------|
| Define an error   | `class extends Error`         | `#[derive(Error)] enum`             |
| Raise an error    | `throw new XError()`          | `Err(XError::Variant(...))`         |
| Propagate         | implicit (uncaught throws)    | `?` operator                        |
| Handle            | `try/catch` + `instanceof`   | `match` on `Result`                 |
| Type safety       | None (`catch(e: unknown)`)    | Full — compiler-checked             |
| Exhaustive check  | No                            | Yes — miss a variant, won't compile |

---

## 9. Mental Model

- **An error enum** = one type, multiple failure modes. Like a union type in
  TS, but enforced.
- **`#[error("...")]`** = the message string. Think template literals.
- **`#[from]`** = "auto-convert this library error into my error when `?` is
  used."
- **`?`** = "if this failed, convert the error and return early."
- **`match`** on the error = handle each failure mode explicitly. The compiler
  has your back.
