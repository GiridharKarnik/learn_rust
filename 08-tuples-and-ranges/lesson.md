# Lesson 08 — Tuples and Ranges

> ⏱️ Estimated reading time: 15 minutes

In TypeScript, arrays do almost everything — they're your tuples, your lists, your
sequences, your loop bounds. Rust splits that single job across several distinct types:
`Vec` and slices for homogeneous lists (Lesson 07), and two types we haven't given proper
attention yet — **tuples** (fixed-size, mixed-type groupings) and **ranges** (`1..5`,
`1..=5`), which you've been using as `for` loop syntax without realizing they're real,
nameable types with their own fields and trait implementations.

By the end of this lesson, you'll know when to reach for a tuple instead of a struct, and
you'll stop thinking of `1..5` as "loop magic" and start thinking of it as a value you can
store, pass around, and match on.

---

## 1. Tuples

A tuple groups a fixed number of values, each with its own type, without needing a named
struct.

```rust
let stop: (&str, u8, u32) = ("Chennai Central", 3, 80);
```

### Accessing fields with `.0`, `.1`, `.2`

Tuple fields have no names — you access them by position, using `.N`:

```rust
let stop = ("Chennai Central", 3, 80);
println!("Station: {}", stop.0);   // "Chennai Central"
println!("Platform: {}", stop.1); // 3
println!("Speed limit: {}", stop.2); // 80
```

### Destructuring

The more common way to work with a tuple is to pull it apart immediately:

```rust
let (name, platform, speed_limit) = ("Chennai Central", 3, 80);
println!("{name} is on platform {platform}, speed limit {speed_limit} km/h");
```

This works in function parameters too:

```rust
fn describe_stop((name, platform, speed): (&str, u8, u32)) -> String {
    format!("{name} (platform {platform}) — limit {speed} km/h")
}
```

### Tuples as multi-value returns

Rust functions return exactly one value — tuples are how you return "several things" without
declaring a struct just for the occasion:

```rust
fn min_max(speeds: &[u32]) -> (u32, u32) {
    let min = *speeds.iter().min().unwrap();
    let max = *speeds.iter().max().unwrap();
    (min, max)
}

let (slowest, fastest) = min_max(&[80, 130, 110, 160]);
```

### Recap: tuple structs

Back in Lesson 04 you met **tuple structs** — `struct Point(f64, f64)` — which pair a tuple's
positional field access with a named, distinct type. That's the same `.0`/`.1` access you just
saw, just wrapped in a type name so it can't be confused with other tuples of the same shape.
We won't re-teach that here — just note the connection.

### Tuple vs struct: readability tradeoff

| | Tuple `(&str, u8, u32)` | Struct `Stop { name, platform, speed }` |
|---|---|---|
| Setup cost | None — just write the literal | Must declare the struct first |
| Field access | Positional (`.0`, `.1`) — easy to misorder | Named (`.name`) — self-documenting |
| Good for | Short-lived, local groupings (2-3 fields, obvious order) | Anything returned across function boundaries, or with more than ~3 fields |
| Refactor safety | Adding a field shifts every `.N` after it | Adding a field doesn't touch existing field names |

Rule of thumb: if you're explaining what `.1` means in a comment, it should probably be a
struct.

---

## 2. Ranges Are Real Types

You've written `for i in 1..5` since Lesson 02. That `1..5` is not special loop syntax — it's
a value of type `Range<i32>`, constructed the moment you write it. You can store it in a
variable, pass it to a function, or print its debug form, same as any other value.

```rust
let r: std::ops::Range<i32> = 1..5;
println!("{:?}", r);        // 1..5
println!("{} {}", r.start, r.end); // 1 5
```

### The range family

| Syntax | Type | Includes end? |
|---|---|---|
| `1..5` | `Range<T>` | No — `1,2,3,4` |
| `1..=5` | `RangeInclusive<T>` | Yes — `1,2,3,4,5` |
| `..5` | `RangeTo<T>` | No — used only as a slice index, not iterable |
| `5..` | `RangeFrom<T>` | n/a — used only as a slice index, not iterable |
| `..` | `RangeFull` | n/a — used only as a slice index |

`Range<T>` and `RangeInclusive<T>` are the two you'll actually iterate over or do math with.
The other three only show up as slice-indexing shorthand (see §2.3) — they don't implement
`Iterator`.

`.start` and `.end` are plain public fields — nothing magic:

```rust
let platforms = 1..=12;
println!("First platform: {}", platforms.start()); // RangeInclusive uses methods, not fields
let r = 1..5;
println!("start={} end={}", r.start, r.end); // Range uses plain fields
```

(`RangeInclusive` hides its fields behind `.start()`/`.end()` methods instead of public
fields — a historical API wrinkle. `Range` exposes `.start`/`.end` directly.)

### Ranges implement `Iterator`

Because `Range<T>` implements `Iterator`, every iterator method you'll meet properly in
**Lesson 11 (Iterators)** already works on it:

```rust
for n in (0..20).step_by(5) {  // 0, 5, 10, 15
    println!("{n}");
}

for n in (1..=5).rev() {       // 5, 4, 3, 2, 1
    println!("{n}");
}

let doubled: Vec<i32> = (1..5).map(|n| n * 2).collect(); // [2, 4, 6, 8]
```

### Ranges as function parameters

A range is just a value — you can accept one as an argument:

```rust
use std::ops::RangeInclusive;

fn sum_range(r: RangeInclusive<u32>) -> u32 {
    r.sum()
}

let total = sum_range(1..=100); // 5050
```

### Ranges for slice indexing (recap)

Lesson 07 already showed you this — `&stops[1..4]`, `&stops[..3]`, `&stops[2..]`. That's the
exact same `Range`/`RangeTo`/`RangeFrom` family from the table above, used as a slice index
instead of an iterator. Nothing new to learn here, just the connection: slice indexing and
`for` loops are reading the *same* type.

### Ranges in `match` guards

Ranges also work as match **patterns**, not just values:

```rust
fn classify_speed(speed_kmh: u32) -> &'static str {
    match speed_kmh {
        0..=59 => "slow",
        60..=120 => "moderate",
        121..=200 => "fast",
        _ => "very fast",
    }
}
```

**Gotcha:** in a match *pattern*, only `..=` (inclusive) is allowed — `0..60 =>` is a syntax
error. That's the opposite of what you'd expect from `for` loops, where `..` (exclusive) is
the common case. Pattern-position ranges must spell out both ends inclusively.

---

## 3. Decision Table: Tuple vs Array vs Vec vs Slice vs Range

| Type | Fixed size? | Mixed types? | Taught in | Use when |
|---|---|---|---|---|
| `(T1, T2, ...)` tuple | Yes | Yes | Lesson 08 (here) | Grouping a few values locally, or returning multiple values |
| `[T; N]` array | Yes | No | Lesson 07 | You know the exact count at compile time (rare) |
| `Vec<T>` | No | No | Lesson 07 | A growable list — most common collection |
| `&[T]` slice | No (borrowed) | No | Lesson 07 | Function parameters over any Vec/array |
| `Range`/`RangeInclusive` | n/a | n/a (single `T`) | Lesson 08 (here) | Loop bounds, slice indices, match guards, "between X and Y" |

---

## 4. Comparison with TypeScript

| Feature | TypeScript | Rust |
|---|---|---|
| Tuple | `[string, number]` — still an `Array` at runtime, no real distinction | `(&str, u8)` — a genuinely distinct type, stack-allocated, fixed arity |
| Tuple access | `t[0]`, `t[1]` | `t.0`, `t.1` |
| Destructuring | `const [name, age] = t` | `let (name, age) = t;` |
| Range type | Doesn't exist — `for (let i = 1; i < 5; i++)` or `Array.from({length: 5})` | `1..5` is a real `Range<T>` value |
| Range as a value | N/A | Can be stored, passed to functions, matched on |
| "Between X and Y" check | `x >= 1 && x <= 5` | `(1..=5).contains(&x)` or `1..=5 => ...` in a match |

The biggest mindset shift: TS tuples are a *type-level fiction* over a real array — slicing,
spreading, and `.length` all still apply. Rust tuples are a genuinely different runtime
shape with a fixed arity; there's no `.len()`, no iterating over a tuple's fields, no
`for...of`.

---

## 5. Mental Model Summary

```
Tuple:  (&str,  u8,  u32)
         │      │     │
       field0 field1 field2    ← fixed count, fixed types, positional access

Range:  1 .. 5          Range<i32>      { start: 1, end: 5 }      — end excluded
        1 ..= 5         RangeInclusive  { start: 1, end: 5 }      — end included
```

- **Tuple** = a fixed-size, mixed-type grouping. Reach for a struct once it needs names.
- **Range** = a real value with `.start`/`.end`, not just loop syntax — it implements
  `Iterator`, can be a function parameter, and can appear in `match` patterns.
- In `for` loops and slice indices: `..` is exclusive, `..=` is inclusive.
- In `match` patterns: only `..=` is valid — plain `..` is a syntax error there.

---

## Up Next

Now that tuples and ranges are proper tools in your kit, we'll meet **HashMap** — Rust's
key-value store — and the Entry API that makes working with it elegant.
