# Lesson 13: Error Handling with `thiserror`

## 1. The Problem: Too Many Error Types

You're building a railway timetable parser. One function needs to:
- Read a file (might fail with `std::io::Error`)
- Parse JSON (might fail with `serde_json::Error`)
- Validate data (might fail with your own custom reason)

```rust
fn load_timetable(path: &str) -> Result<Timetable, ???> {
    let data = std::fs::read_to_string(path)?;   // io::Error
    let raw: RawTimetable = serde_json::from_str(&data)?;  // serde_json::Error
    if raw.trains.is_empty() {
        return Err(/* some custom error */);  // ???
    }
    Ok(Timetable::from(raw))
}
```

In TypeScript, you'd just `throw new Error("whatever")` and catch it somewhere.
In Rust, `Result<T, E>` needs a **single, concrete type** for `E`. You can't
return three different error types from one function — unless you unify them.

That's where custom error enums come in. And `thiserror` makes building them painless.

---

## 2. Custom Error Enums

The Rust pattern: define an enum where each variant represents a different failure mode.

```rust
#[derive(Debug)]
enum TimetableError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
    EmptyTimetable,
    InvalidTrainNumber(u32),
}
```

Now your function returns `Result<Timetable, TimetableError>` — one type, multiple
failure modes. Callers can `match` on the variant to decide how to recover:

```rust
match load_timetable("schedule.json") {
    Ok(tt) => println!("Loaded {} trains", tt.trains.len()),
    Err(TimetableError::EmptyTimetable) => println!("No trains scheduled!"),
    Err(TimetableError::IoError(e)) => eprintln!("File problem: {e}"),
    Err(e) => eprintln!("Other error: {e:?}"),
}
```

But there's a problem: your enum doesn't implement `Display` or `std::error::Error`
yet, and converting from library errors requires manual `From` implementations.

---

## 3. Implementing `std::error::Error` Manually — The Boilerplate

To be a "proper" Rust error, your type needs:
- `Debug` (usually derived)
- `Display` (for human-readable messages)
- `std::error::Error` (for the error trait, plus optional `source()` chaining)
- `From<OtherError>` for each error type you want to convert with `?`

Here's what that looks like by hand:

```rust
use std::fmt;

#[derive(Debug)]
enum TimetableError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
    EmptyTimetable,
    InvalidTrainNumber(u32),
}

impl fmt::Display for TimetableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimetableError::IoError(e) => write!(f, "IO error: {e}"),
            TimetableError::ParseError(e) => write!(f, "Parse error: {e}"),
            TimetableError::EmptyTimetable => write!(f, "Timetable has no trains"),
            TimetableError::InvalidTrainNumber(n) => {
                write!(f, "Invalid train number: {n}")
            }
        }
    }
}

impl std::error::Error for TimetableError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TimetableError::IoError(e) => Some(e),
            TimetableError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TimetableError {
    fn from(e: std::io::Error) -> Self {
        TimetableError::IoError(e)
    }
}

impl From<serde_json::Error> for TimetableError {
    fn from(e: serde_json::Error) -> Self {
        TimetableError::ParseError(e)
    }
}
```

That's **50+ lines** of pure boilerplate for 4 error variants. Every time you add a
variant, you update `Display`, possibly `source()`, and maybe add another `From`.

Coming from TypeScript, this feels like writing `class MyError extends Error` for
every single thing — except 10x more verbose.

---

## 4. `thiserror` to the Rescue

`thiserror` is a derive macro that generates all that boilerplate for you:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum TimetableError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Timetable has no trains")]
    EmptyTimetable,

    #[error("Invalid train number: {0}")]
    InvalidTrainNumber(u32),
}
```

**That's it.** 12 lines instead of 50+. It generates:
- `impl Display` from the `#[error("...")]` attributes
- `impl std::error::Error` with correct `source()` from `#[from]`/`#[source]`
- `impl From<T>` for each `#[from]` field

The `?` operator now converts automatically:

```rust
fn load_timetable(path: &str) -> Result<Timetable, TimetableError> {
    let data = std::fs::read_to_string(path)?;          // io::Error → TimetableError
    let raw: RawTimetable = serde_json::from_str(&data)?; // serde_json::Error → TimetableError
    if raw.trains.is_empty() {
        return Err(TimetableError::EmptyTimetable);
    }
    Ok(Timetable::from(raw))
}
```

---

## 5. Error Messages with `#[error("...")]`

The `#[error]` attribute uses the same formatting syntax as `format!()`:

```rust
#[derive(Debug, Error)]
enum SignalError {
    // Positional fields: {0}, {1}, etc.
    #[error("Signal {0} is stuck on {1}")]
    Stuck(String, String),

    // Named fields work too
    #[error("Signal at km {position} malfunctioned: {reason}")]
    Malfunction { position: f64, reason: String },

    // You can use Debug formatting
    #[error("Unknown signal state: {0:?}")]
    UnknownState(u8),

    // Simple message, no interpolation
    #[error("Signal system offline")]
    Offline,
}
```

Each `#[error("...")]` becomes the `Display` output — what users see when they
print the error with `{e}` or call `.to_string()`.

Think of it like TypeScript template literals for your error messages:
```typescript
// TypeScript
throw new Error(`Signal ${name} is stuck on ${state}`);

// Rust with thiserror
Err(SignalError::Stuck(name, state))
// Display output: "Signal North-1 is stuck on red"
```

---

## 6. `#[from]` for Automatic Conversion

`#[from]` on a field generates `impl From<FieldType> for YourError`:

```rust
#[derive(Debug, Error)]
enum TicketError {
    #[error("File error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
```

This generates:
```rust
// You get this for free:
impl From<std::io::Error> for TicketError { ... }
impl From<serde_json::Error> for TicketError { ... }
```

Which means the `?` operator works seamlessly:
```rust
fn load_tickets(path: &str) -> Result<Vec<Ticket>, TicketError> {
    let data = std::fs::read_to_string(path)?;   // ? converts io::Error → TicketError::Io
    let tickets = serde_json::from_str(&data)?;   // ? converts serde error → TicketError::Json
    Ok(tickets)
}
```

**Rule:** Each source error type can only appear once with `#[from]`. If you had two
variants wrapping `io::Error`, you'd need to construct one of them manually.

---

## 7. Nested Errors with `#[source]`

Sometimes you want to wrap an error but also add context. `#[source]` marks a field
as the underlying cause (returned by `.source()`) without generating `From`:

```rust
#[derive(Debug, Error)]
enum BookingError {
    // #[from] = generates From + sets source()
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    // #[source] = sets source() only, no From generated
    #[error("Failed to reserve seat {seat} on train {train}")]
    ReservationFailed {
        train: String,
        seat: u32,
        #[source]
        cause: DatabaseError,
    },
}
```

The difference:
| Attribute   | Generates `From`? | Sets `source()`? | Use when...                          |
|-------------|-------------------|-------------------|--------------------------------------|
| `#[from]`   | ✅ Yes            | ✅ Yes            | Direct 1:1 wrapping of another error |
| `#[source]` | ❌ No             | ✅ Yes            | Adding context alongside the cause   |

Error chaining lets you walk the full causal chain:
```rust
fn print_error_chain(err: &dyn std::error::Error) {
    println!("Error: {err}");
    let mut source = err.source();
    while let Some(cause) = source {
        println!("  Caused by: {cause}");
        source = cause.source();
    }
}
```

---

## 8. When to Use `thiserror` vs `anyhow`

Rust has two popular error-handling crates, and they solve different problems:

### `thiserror` — For Libraries & Typed Errors
- Generates `Display`, `Error`, and `From` impls for your custom error types
- Callers can `match` on specific variants
- **Use when:** you're writing a library or module that other code depends on

```rust
// Library code: callers can match on variants
#[derive(Debug, Error)]
pub enum TrainApiError {
    #[error("Train not found: {0}")]
    NotFound(u32),
    #[error("API request failed: {0}")]
    Network(#[from] reqwest::Error),
}
```

### `anyhow` — For Applications & Quick Prototyping
- Provides `anyhow::Result<T>` which can hold *any* error
- Great for `main()`, CLI tools, scripts — anywhere you just want to propagate errors
- **Use when:** you're the top-level consumer and just want errors to bubble up

```rust
// Application code: just propagate everything
use anyhow::Result;

fn main() -> Result<()> {
    let config = load_config()?;     // any error type works
    let db = connect_db(&config)?;   // no need to unify types
    run_server(db)?;
    Ok(())
}
```

### The Rule of Thumb

```
Library code (consumed by others)  →  thiserror
Application code (the final binary) →  anyhow (or thiserror if you want structured errors)
```

You can use both in the same project: `thiserror` in your library modules,
`anyhow` in `main.rs`.

---

## 9. Comparison with TypeScript

### TypeScript: throw/catch, untyped errors

```typescript
class TrainNotFoundError extends Error {
  constructor(public trainId: number) {
    super(`Train ${trainId} not found`);
    this.name = "TrainNotFoundError";
  }
}

function findTrain(id: number): Train {
  const train = trains.find(t => t.id === id);
  if (!train) throw new TrainNotFoundError(id);
  return train;
}

// Caller — no guarantee about what's thrown
try {
  const train = findTrain(42);
} catch (e) {
  if (e instanceof TrainNotFoundError) {
    // handle specifically
  } else {
    // generic fallback — you HOPE this covers everything
  }
}
```

**Problems:**
- `throw` is invisible in the function signature
- `catch` gives you `unknown` — no compiler help
- Nothing stops you from forgetting to handle an error case

### Rust: Result + thiserror, fully typed

```rust
#[derive(Debug, Error)]
enum TrainError {
    #[error("Train {0} not found")]
    NotFound(u32),
    #[error("Service suspended on {0}")]
    Suspended(String),
}

fn find_train(id: u32) -> Result<Train, TrainError> { ... }

// Caller — the compiler enforces handling
match find_train(42) {
    Ok(train) => println!("{}", train.name),
    Err(TrainError::NotFound(id)) => println!("No train #{id}"),
    Err(TrainError::Suspended(line)) => println!("{line} suspended"),
}
```

**Advantages:**
- Error is **in the return type** — can't forget it
- `match` is exhaustive — add a variant, compiler tells you everywhere to update
- No runtime type-checking guesswork
- Errors compose with `?` and `From`

### Side-by-side

| Concept              | TypeScript                     | Rust + thiserror                      |
|----------------------|-------------------------------|---------------------------------------|
| Error definition     | `class extends Error`         | `#[derive(Error)] enum`              |
| Raising an error     | `throw new XError()`          | `Err(XError::Variant(...))`          |
| Propagating          | (implicit — uncaught throws)  | `?` operator                         |
| Handling             | `try/catch` + `instanceof`    | `match` on `Result`                  |
| Type safety          | None (`catch(e: unknown)`)    | Full — compiler-checked              |
| Exhaustiveness       | No                            | Yes — `match` must cover all variants|
| Error chaining       | `cause` option (ES2022)       | `#[source]` / `#[from]`             |

---

## 10. Mental Model: Railway Switches

Think of error handling like a railway junction with switches (points):

```
                    ┌─── Ok(Timetable) ──────── 🚉 Success
    read_file()? ──┤
                    └─── Err(IoError) ────────── ⚠️ Track 1

                    ┌─── Ok(RawData) ────────── 🚉 Success
    parse_json()? ──┤
                    └─── Err(ParseError) ─────── ⚠️ Track 2

                    ┌─── Ok(()) ─────────────── 🚉 Success
    validate()? ────┤
                    └─── Err(EmptyTimetable) ── ⚠️ Track 3
```

Each `?` is a **railway switch**:
- **Ok** → train continues forward on the main line
- **Err** → train gets diverted to the error siding

Your `thiserror` enum is the **marshalling yard** that collects all diverted trains
(errors) into one unified system. The `#[from]` attribute is the automatic switch
that routes each error type to the right siding without manual intervention.

The `?` operator is the key insight — it's not just "unwrap or return error." It's:
1. Check if Ok → continue
2. If Err → call `From::from()` to convert the error → return `Err(converted)`

That's why `#[from]` matters: it generates the `From` impl that `?` calls.

### The Compose Pattern

```rust
fn book_ticket(...) -> Result<Booking, BookingError> {
    let train = find_train(trains, number)?;        // ? switch
    validate_booking(passenger, &class, seats)?;     // ? switch
    let confirmation = process_payment(price)?;      // ? switch
    Ok(Booking { ... })                              // main line
}
```

Each `?` is a switch point. If any step fails, the error is automatically
converted to `BookingError` and returned. The happy path reads like a straight
line — no nesting, no callbacks, no try/catch blocks.

This is the **railway-oriented programming** pattern: your data flows along
the track, and errors divert cleanly to the side. `thiserror` builds the
switches; `?` activates them.

---

## Summary

| Concept                  | What it does                                        |
|--------------------------|-----------------------------------------------------|
| `#[derive(Error)]`       | Implements `Display` + `Error` from attributes      |
| `#[error("...")]`        | Defines the human-readable error message            |
| `#[from]`                | Generates `From<T>` + sets `source()` — enables `?` |
| `#[source]`              | Sets `source()` only — for adding context           |
| `?` operator             | Unwraps `Ok` or converts + returns `Err`            |
| `thiserror`              | Best for libraries — typed, matchable errors        |
| `anyhow`                 | Best for applications — any error, easy propagation |
