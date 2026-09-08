# Lesson 07 — Vec and Slices

You've already pushed items into a `Vec` and iterated over `&vec`. Now let's go deeper.
`Vec<T>` is Rust's most-used collection — the growable, heap-allocated array. And *slices*
(`&[T]`) are how you borrow pieces of one.

> HashMap and iterators get their own dedicated lessons next. This one is 100% Vec + slices.

---

## 1. Vec\<T\> — The Growable Array

Like JavaScript arrays but typed — every element must be the same type, and the Vec *owns*
its contents on the heap.

### Creating

```rust
let mut trains: Vec<String> = Vec::new();       // empty, typed explicitly
let platforms = vec![1, 2, 3, 4, 5];            // vec![] macro — most common
let mut schedule = Vec::with_capacity(100);      // pre-allocate (perf hint, not a limit)
schedule.push("06:00 Rajdhani Express");
```

### Adding elements

```rust
let mut trains = vec!["Shatabdi Express"];
trains.push("Rajdhani Express");                        // push to end — O(1) amortized
trains.insert(0, "Duronto Express");                    // insert at index — O(n), shifts elements
trains.extend(vec!["Kovai Express", "Island Express"]); // append another collection
```

### Removing elements

```rust
let mut trains = vec!["Shatabdi", "Rajdhani", "Duronto", "Kovai"];

let last = trains.pop();             // Some("Kovai") — returns Option<T>!
let removed = trains.remove(1);      // "Rajdhani" — panics if out of bounds
trains.retain(|t| *t != "Duronto");  // keep only matching elements (in place)
// trains: ["Shatabdi"]
```

`.pop()` returns `Option<T>` — the pattern you learned in Lesson 06. Empty vec? You get `None`, not a crash.

### Ownership: Vec owns its elements

```rust
let name = String::from("Rajdhani Express");
let mut trains = Vec::new();
trains.push(name);
// println!("{}", name); // ❌ name was moved into the Vec
```

When the Vec is dropped, it drops every element it owns. Same ownership rules from Lesson 03, applied to a collection.

---

## 2. Accessing Elements

```rust
let trains = vec!["Shatabdi", "Rajdhani", "Duronto"];

// Indexing — panics if out of bounds!
let first = trains[0];       // "Shatabdi"
// let oops = trains[99];    // 💥 panic!

// .get() — returns Option<&T> — safe!
let maybe = trains.get(0);   // Some(&"Shatabdi")
let nope  = trains.get(99);  // None (no panic)

// Convenience methods
trains.first();                    // Some(&"Shatabdi")
trains.last();                     // Some(&"Duronto")
trains.len();                      // 3
trains.is_empty();                 // false
trains.capacity();                 // at least 3
trains.contains(&"Rajdhani");      // true
```

**Rule of thumb:** Use `.get()` when the index is uncertain. Use `[i]` only when you *know* it's valid.

---

## 3. Iterating

Three ways to iterate, differing by **ownership**:

```rust
// Borrow each item — most common
for train in &trains { /* train: &T */ }     // trains still usable after

// Mutably borrow — modify in place
for speed in &mut speeds { *speed += 10; }   // speeds still usable after

// Consume — takes ownership (Vec is gone!)
for train in trains { /* train: T (owned) */ }
// trains is moved — can't use it anymore

// With index
for (i, train) in trains.iter().enumerate() {
    println!("Platform {}: {}", i + 1, train);
}
```

**Cheat sheet:**

| Syntax              | What you get      | Vec after loop? |
|---------------------|-------------------|-----------------|
| `for x in &vec`     | `&T` (shared ref) | Still usable    |
| `for x in &mut vec` | `&mut T` (mut ref)| Still usable    |
| `for x in vec`      | `T` (owned)       | **Moved/gone**  |

---

## 4. Slices &\[T\]

A *slice* is a reference to a contiguous sequence of elements — a "window" into a Vec (or array).

### Creating slices

```rust
let trains = vec!["Shatabdi", "Rajdhani", "Duronto", "Kovai", "Island Express"];

let first_two  = &trains[0..2];  // ["Shatabdi", "Rajdhani"]
let middle     = &trains[1..4];  // ["Rajdhani", "Duronto", "Kovai"]
let from_third = &trains[2..];   // ["Duronto", "Kovai", "Island Express"]
let up_to_3    = &trains[..3];   // ["Shatabdi", "Rajdhani", "Duronto"]
let everything = &trains[..];    // all five
```

### Why slices matter for function signatures

Same lesson as `&str` vs `&String` — but for collections:

```rust
// ❌ Unnecessarily restrictive — only accepts &Vec<String>
fn print_trains_bad(trains: &Vec<String>) { ... }

// ✅ Accepts &Vec<String>, &[String], sub-slices, and arrays
fn print_trains(trains: &[String]) { ... }
```

`&Vec<T>` auto-coerces to `&[T]`, but `&[T]` also accepts sub-slices and fixed arrays — strictly more flexible.

### Example: a function that takes a slice

```rust
struct Train { name: String, speed_kmh: u32 }

fn fastest_train(trains: &[Train]) -> Option<&Train> {
    if trains.is_empty() { return None; }
    let mut fastest = &trains[0];
    for train in &trains[1..] {   // iterate a sub-slice!
        if train.speed_kmh > fastest.speed_kmh { fastest = train; }
    }
    Some(fastest)
}
```

---

## 5. Useful Vec Methods

### Sorting

```rust
let mut speeds = vec![160, 110, 200, 130];
speeds.sort();                          // ascending: [110, 130, 160, 200]
speeds.sort_by(|a, b| b.cmp(a));        // descending: [200, 160, 130, 110]

// Sort structs by a field
trains.sort_by_key(|t| t.speed_kmh);           // ascending by speed
trains.sort_by(|a, b| b.speed_kmh.cmp(&a.speed_kmh)); // descending by speed
```

### Dedup, reverse, truncate

```rust
let mut stops = vec!["Chennai", "Chennai", "Katpadi", "Salem", "Salem", "Erode"];
stops.dedup();      // ["Chennai", "Katpadi", "Salem", "Erode"]
                    // ⚠️ Only consecutive duplicates! Sort first for all dupes.

stops.reverse();    // ["Erode", "Salem", "Katpadi", "Chennai"]
stops.truncate(2);  // ["Erode", "Salem"] — keep only first n
```

### Splitting and windowing

```rust
let platforms = vec![1, 2, 3, 4, 5, 6];

let (left, right) = platforms.split_at(3);   // [1,2,3] and [4,5,6]

// Sliding windows — great for comparing adjacent elements
for w in platforms.windows(3) { }  // [1,2,3], [2,3,4], [3,4,5], [4,5,6]

// Non-overlapping chunks — great for batch processing
for c in platforms.chunks(2) { }   // [1,2], [3,4], [5,6]
```

---

## 6. Vec and Ownership — Common Patterns

```rust
// Moving — function takes ownership, caller loses the Vec
fn cancel_all(trains: Vec<String>) { /* trains consumed */ }

// Borrowing — preferred! Caller keeps the Vec.
fn print_schedule(trains: &[String]) { /* read-only view */ }

// Returning — ownership transfers to caller (no copy, just a move)
fn build_roster() -> Vec<String> {
    vec!["Shatabdi".into(), "Rajdhani".into()]
}

// Cloning — when you need to keep the original AND give it away
let backup = trains.clone();  // explicit deep copy
cancel_all(trains);           // original is gone...
// ...but backup is fine
```

Rust never silently copies heap data — `.clone()` is always explicit.

---

## 7. Comparison with TypeScript

| Feature | TypeScript `Array` | Rust `Vec<T>` |
|---|---|---|
| Type safety | `any[]` allowed | Single type `T` enforced at compile time |
| Bounds check | `arr[99]` → `undefined` | `vec[99]` → **panic**, `.get(99)` → `None` |
| Push/pop | `.push()` / `.pop()` | `.push()` / `.pop()` (returns `Option<T>`!) |
| Remove at index | `.splice(i, 1)` | `.remove(i)` |
| Filter in place | `arr = arr.filter(...)` (new array) | `.retain(\|x\| ...)` (in place) |
| Slicing | `.slice(1, 3)` → new array (copy!) | `&vec[1..3]` → slice (zero-copy borrow!) |
| Ownership | Garbage collected | Vec owns elements, dropped when out of scope |
| Sort | `.sort()` mutates | `.sort()` mutates in place |
| Memory | Hidden | `.len()`, `.capacity()`, `Vec::with_capacity()` |

Biggest mindset shift: JS `.slice()` copies. Rust `&vec[1..3]` borrows — zero allocation, but borrow rules apply.

---

## 8. Mental Model Summary

```
Vec<T> = pointer + length + capacity
         ┌───┐
         │ • │──→ [ T, T, T, T, _, _, _, _ ]
         │ 4 │     ↑ len=4      ↑ capacity=8
         │ 8 │
         └───┘
       (on stack)         (on heap)
```

- **Vec owns** its elements. Push moves data in; drop frees it all.
- **`&[T]` (slice)** = pointer + length. Borrows part (or all) of a Vec or array.
- Use `.get()` for safe access, `[i]` only when you're sure.
- Take `&[T]` in function params, not `&Vec<T>`.
- `for x in &vec` borrows, `for x in vec` consumes.
- Sort, dedup, retain, reverse — all modify in place.

---

## Up Next

Now that you're comfortable with `Vec` and slices, we'll explore **HashMap** — Rust's
key-value store — and then **iterators**, where Rust collections really shine.
