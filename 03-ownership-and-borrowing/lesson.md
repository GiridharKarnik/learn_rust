# Lesson 03 — Ownership & Borrowing

This is **the** concept that makes Rust different from every other language. Once this
clicks, everything else in Rust makes sense.

---

## The Problem Rust Solves

In C/C++, you manually manage memory → bugs (use-after-free, double-free, dangling pointers).
In JS/Python/Java, a garbage collector manages memory → safe but slow, unpredictable pauses.

Rust's answer: **ownership rules checked at compile time**. No garbage collector, no manual
free, no runtime cost. The compiler simply won't let you write buggy memory code.

---

## The Three Rules of Ownership

Memorize these. Everything flows from them.

1. **Every value has exactly one owner** (a variable)
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value is dropped** (memory freed)

```rust
{
    let train = String::from("Rajdhani");  // train owns the String
    println!("{train}");
}   // ← train goes out of scope here — String is dropped, memory freed
```

---

## 1. Move Semantics — Ownership Transfers

When you assign a `String` to another variable, ownership **moves**:

```rust
let train_a = String::from("Shatabdi");
let train_b = train_a;       // ownership MOVED to train_b

println!("{train_b}");        // ✅ works — train_b owns it
println!("{train_a}");        // ❌ ERROR: train_a no longer owns the data
```

This is the #1 surprise for newcomers. In JS, both variables would point to the same
string. In Rust, **there can be only one owner** — so `train_a` becomes invalid.

Think of it like a physical train ticket — if you hand it to someone, you don't have
it anymore.

### Why not just copy?

A `String` lives on the **heap** (dynamically allocated). Copying heap data is expensive,
so Rust makes you do it explicitly:

```rust
let train_a = String::from("Shatabdi");
let train_b = train_a.clone();   // explicit deep copy

println!("{train_a}");   // ✅ both work now
println!("{train_b}");   // ✅
```

### The Copy Exception

Simple types that live on the **stack** (integers, floats, bools, chars) are cheap to copy,
so they implement the `Copy` trait and are copied automatically:

```rust
let platform = 5;
let other = platform;     // COPIED, not moved

println!("{platform}");   // ✅ both work
println!("{other}");      // ✅
```

| Type | Copy or Move? |
|------|---------------|
| `i32`, `u32`, `f64`, `bool`, `char` | **Copy** (cheap, stack-only) |
| `String`, `Vec`, `HashMap` | **Move** (expensive, heap data) |
| `&str` | **Copy** (it's just a pointer + length, no heap ownership) |

---

## 2. Ownership and Functions

Passing a value to a function **moves** it (for heap types):

```rust
fn print_train(name: String) {
    println!("Train: {name}");
}   // name is dropped here

let train = String::from("Duronto");
print_train(train);          // ownership moved into the function
println!("{train}");         // ❌ ERROR: train was moved
```

The function took ownership, used it, and dropped it. The caller can't use it anymore.

This is annoying. You don't always want to give up ownership just to let a function
read the data. That's where **borrowing** comes in.

---

## 3. Borrowing with `&` (Immutable References)

A **reference** lets you use a value **without taking ownership**:

```rust
fn print_train(name: &String) {    // borrows — doesn't take ownership
    println!("Train: {name}");
}

let train = String::from("Duronto");
print_train(&train);              // lend it with &
println!("{train}");              // ✅ still works — we still own it
```

Think of `&` as lending someone your ticket to look at — they give it back when they're done.

### Simpler: use `&str` instead of `&String`

In practice, functions that read strings should take `&str`, not `&String`:

```rust
fn print_train(name: &str) {       // accepts both &String and &str
    println!("Train: {name}");
}

let owned = String::from("Duronto");
let literal = "Shatabdi";

print_train(&owned);     // ✅ &String auto-converts to &str
print_train(literal);    // ✅ &str works directly
```

This is idiomatic Rust. You'll see `&str` in function parameters everywhere.

---

## 4. Mutable Borrowing with `&mut`

To modify borrowed data, use a **mutable reference**:

```rust
fn add_express(name: &mut String) {
    name.push_str(" Express");
}

let mut train = String::from("Rajdhani");   // must be mut!
add_express(&mut train);                     // lend mutably
println!("{train}");                         // "Rajdhani Express"
```

### The Big Restriction

You can have **either**:
- Any number of `&` (immutable references) at the same time, **OR**
- Exactly **one** `&mut` (mutable reference) at a time

**Never both at the same time.**

```rust
let mut train = String::from("Rajdhani");

let r1 = &train;       // ✅ immutable borrow
let r2 = &train;       // ✅ another immutable borrow — fine
let r3 = &mut train;   // ❌ ERROR: can't borrow as mutable while immutable borrows exist
```

Why? This prevents **data races** at compile time. If someone is reading data, nobody
should be writing to it simultaneously.

---

## 5. The Slice Type: `&str` Explained

Now you can finally understand `&str`. It's a **reference to a portion of string data**:

```rust
let full = String::from("Chennai Central");
let city: &str = &full[0..7];     // "Chennai" — a slice of the String
println!("{city}");
```

A string literal `"hello"` is also a `&str` — it points to data baked into the binary.

That's why we have two string types:
- `String` → owns the data, can grow/modify
- `&str` → borrows/views string data, read-only

---

## 6. Returning Ownership

Functions can **give back** ownership:

```rust
fn create_train(name: &str, number: u32) -> String {
    format!("{name} (#{number})")    // creates and returns an owned String
}

let train = create_train("Rajdhani", 12001);
println!("{train}");   // we own it now
```

A common pattern: borrow inputs (`&str`), return owned outputs (`String`).

---

## 7. Scope and Dropping

Values are dropped (freed) when their owner goes out of scope:

```rust
fn main() {
    let outer = String::from("I live in main");

    {
        let inner = String::from("I live in this block");
        println!("{inner}");   // ✅
    }   // inner is dropped here

    // println!("{inner}");   // ❌ inner doesn't exist anymore
    println!("{outer}");      // ✅ outer is still alive
}
```

---

## Mental Model Summary

```
OWNERSHIP:     Every value has one owner. When the owner dies, the value dies.
MOVE:          Assigning heap types transfers ownership. Old variable is dead.
CLONE:         .clone() makes an explicit deep copy. Both variables live.
COPY:          Stack types (i32, bool, etc.) copy automatically. Both live.
BORROW (&):    Lend read access. Owner keeps ownership. Many borrows allowed.
MUT BORROW:    Lend write access. Only one at a time. No simultaneous reads.
```

---

## Up Next

Run the examples, then do the exercise. This one has **compiler error challenges** —
you'll look at broken code, predict why it fails, and fix it. That's how you actually
learn ownership. 🚂
