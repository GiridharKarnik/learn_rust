# Lesson 01 — Variables & Types

## The Basics

Every Rust program starts in `fn main()`. Let's learn how to store and work with data.

---

## 1. Variables with `let`

```rust
let station_name = "Chennai Central";
```

By default, variables in Rust are **immutable** — you can't change them after assignment.

```rust
let platform = 5;
platform = 6; // ❌ ERROR: cannot assign twice to immutable variable
```

If you want a variable you can change, add `mut`:

```rust
let mut platform = 5;
platform = 6; // ✅ works fine
```

> **Why immutable by default?** It prevents accidental changes and makes code easier to
> reason about. You opt *in* to mutation, not out of it. This is a core Rust philosophy.

---

## 2. Type Inference

Rust is statically typed, but you usually don't need to write the types — the compiler
figures them out:

```rust
let passengers = 250;       // inferred as i32 (default integer type)
let delay_minutes = 4.5;    // inferred as f64 (default float type)
let is_express = true;      // inferred as bool
let train_name = "Rajdhani"; // inferred as &str (string slice)
```

You *can* be explicit when you need to:

```rust
let passengers: u32 = 250;      // unsigned 32-bit integer (no negatives)
let delay_minutes: f32 = 4.5;   // 32-bit float (less precision, less memory)
```

---

## 3. Integer Types

| Size | Signed (+ and −) | Unsigned (+ only) |
|------|-------------------|--------------------|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` (default) | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch-dependent | `isize` | `usize` |

`usize` is special — it's the size of a pointer on your platform. You'll use it for
indexing into collections.

---

## 4. Floating Point Types

```rust
let speed: f64 = 130.5;   // 64-bit (default, more precise)
let temp: f32 = 36.7;     // 32-bit (used in embedded/GPU work)
```

---

## 5. Booleans

```rust
let is_delayed: bool = true;
let on_time = false;  // type inferred
```

---

## 6. Characters

A `char` in Rust is a Unicode scalar value — 4 bytes, not 1 like in C:

```rust
let symbol: char = '🚂';
let letter = 'A';
```

---

## 7. Strings — The Two Kinds

This trips up every Rust beginner. There are two main string types:

| Type | What it is | Mutable? | Where it lives |
|------|------------|----------|----------------|
| `&str` | A reference to string data (a "string slice") | No | Usually in the binary or borrowed from a `String` |
| `String` | An owned, growable string | Yes | Heap-allocated |

```rust
let greeting: &str = "Hello";              // string slice — fixed, baked into the binary
let mut name = String::from("Express");     // owned String — you can modify it
name.push_str(" 2025");                     // now "Express 2025"
```

Don't worry about fully understanding this yet. We'll revisit strings when we cover
**ownership** in Lesson 03. For now just know:
- Use `"quotes"` for quick string literals (`&str`)
- Use `String::from("...")` when you need to own or modify the string

---

## 8. Constants

Constants are *always* immutable and require a type annotation:

```rust
const MAX_PLATFORMS: u32 = 24;
const STATION_CODE: &str = "MAS";
```

They differ from `let` variables:
- Must have an explicit type
- Can be declared in any scope, including global
- Set at compile time — no runtime computation allowed

---

## 9. Shadowing

You can declare a new variable with the same name — this "shadows" the previous one:

```rust
let train = "Shatabdi";
let train = 12001;       // ✅ totally fine — new variable, different type!
```

This is different from `mut` because you're creating a *brand new* variable. The type can
even change. This is useful for transforming a value step by step.

---

## 10. Printing

Use `println!` (it's a macro, hence the `!`):

```rust
let train = "Duronto Express";
let platform = 3;
println!("Train: {}, Platform: {}", train, platform);

// Or with inline variable names (Rust 1.58+):
println!("Train: {train}, Platform: {platform}");
```

---

## Up Next

Run the **examples** (`cargo run` inside `examples/`) to see all of this in action,
then tackle the **exercise** to test yourself!
