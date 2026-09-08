# Lesson 08 — HashMaps

In Lesson 07, you worked with `Vec`, slices, `HashMap`, and iterators all in one big sweep.
Now we slow down and give `HashMap` the spotlight it deserves. HashMaps are *everywhere* in
real programs — configuration tables, caches, counters, indexes — and Rust's Entry API
makes working with them surprisingly elegant once you know the patterns.

By the end of this lesson, you'll be able to build lookup tables, count things, group items,
and wield the Entry API like a seasoned Rustacean.

---

## 1. What Is a HashMap?

A `HashMap<K, V>` stores **key-value pairs**. Think of it like a station directory board:
you look up a station code (the *key*) and get back all the station details (the *value*).

If you're coming from other languages:

- **TypeScript/JavaScript**: like an `Object` or `Map`
- **Python**: like a `dict`
- **Java**: like a `java.util.HashMap`

The big difference? Rust's HashMap is **fully typed** — both keys and values have a fixed
type, and the compiler enforces it.

```rust
use std::collections::HashMap;
```

**HashMap is NOT in the prelude.** You must import it every time. This is the one `use`
statement you'll type more than any other in Rust.

### Key constraints

Keys must implement two traits: `Eq` (equality comparison) and `Hash` (hashing). In
practice, this means:

| Type | Works as a key? |
|------|:---------------:|
| `String`, `&str` | ✅ Yes |
| `i32`, `u64`, etc. | ✅ Yes |
| `bool` | ✅ Yes |
| `f32`, `f64` | ❌ **No!** |

Why not floats? Because `NaN != NaN` — a float can't satisfy `Eq`. If you need a
float-like key, consider wrapping it in an `OrderedFloat` from a crate, or use integer
representations.

---

## 2. Creating and Inserting

### `HashMap::new()`

```rust
use std::collections::HashMap;

let mut stations: HashMap<String, u32> = HashMap::new();
stations.insert("MAS".to_string(), 12);   // Chennai Central has 12 platforms
stations.insert("SBC".to_string(), 10);   // Bangalore City has 10 platforms
stations.insert("NDLS".to_string(), 16);  // New Delhi has 16 platforms
```

### `.insert()` returns `Option<V>`

This is a detail people miss! `.insert()` returns the *old* value if the key already
existed, or `None` if it's new:

```rust
let old = stations.insert("MAS".to_string(), 14); // Upgrade platforms!
println!("{:?}", old); // Some(12) — the previous value
```

This is useful for detecting duplicates:

```rust
if let Some(previous) = stations.insert(code, platforms) {
    println!("Warning: {} already had {} platforms, now updated", code, previous);
}
```

### Creating from vectors of tuples with `.collect()`

If you have a `Vec` of `(key, value)` tuples, you can collect straight into a HashMap:

```rust
let data = vec![
    ("MAS".to_string(), "Chennai Central".to_string()),
    ("SBC".to_string(), "Bangalore City".to_string()),
    ("NDLS".to_string(), "New Delhi".to_string()),
];

let directory: HashMap<String, String> = data.into_iter().collect();
```

The turbofish type annotation on `directory` tells `.collect()` what to build. Rust's type
inference is powerful, but when collecting into a HashMap, you usually need to be explicit.

---

## 3. Accessing Values

### `map[&key]` — the dangerous way

```rust
let platforms = stations["MAS"];  // 12
// let oops = stations["XYZ"];    // PANICS! 💥
```

Just like `vec[index]`, indexing a HashMap panics if the key doesn't exist. **Avoid this
in production code.** It's fine for quick scripts or when you've *just* inserted the key,
but prefer `.get()` for anything else.

### `.get(&key)` — the safe way

```rust
match stations.get("MAS") {
    Some(platforms) => println!("Chennai Central has {} platforms", platforms),
    None => println!("Station not found"),
}

// Or more concisely with if-let:
if let Some(p) = stations.get("BCT") {
    println!("Mumbai Central: {} platforms", p);
}
```

`.get()` returns `Option<&V>` — a *reference* to the value. This is the idiomatic way to
look things up.

### Other useful methods

```rust
// Does this key exist?
if stations.contains_key("HWH") {
    println!("Howrah is in the directory");
}

// Get all keys (unordered!)
for code in stations.keys() {
    print!("{} ", code);
}

// Get all values
let total_platforms: u32 = stations.values().sum();

// How many entries?
println!("{} stations", stations.len());

// Is it empty?
if stations.is_empty() {
    println!("No stations loaded yet");
}
```

---

## 4. Iterating

### Borrow iteration — `for (key, value) in &map`

```rust
let mut routes: HashMap<&str, &str> = HashMap::new();
routes.insert("12001", "Chennai → New Delhi");
routes.insert("12007", "Chennai → Bangalore");
routes.insert("16525", "Bangalore → Kanyakumari");

for (train_no, route) in &routes {
    println!("Train {} runs {}", train_no, route);
}
```

### Mutable iteration — `for (key, value) in &mut map`

```rust
let mut delays: HashMap<String, i32> = HashMap::new();
delays.insert("12001".to_string(), 15);
delays.insert("12007".to_string(), 0);
delays.insert("16525".to_string(), 45);

// Add 5 minutes to every delay (track maintenance)
for (_train, delay) in &mut delays {
    *delay += 5;
}
```

Note: you get `&mut V` for values, but keys are **always immutable** — changing a key
would break the hash.

### ⚠️ Order is NOT guaranteed

Unlike `Vec`, a HashMap makes **no promises** about iteration order. If you need sorted
output, collect keys into a `Vec` and sort:

```rust
let mut codes: Vec<&String> = stations.keys().collect();
codes.sort();
for code in codes {
    println!("{}: {} platforms", code, stations[code.as_str()]);
}
```

---

## 5. Removing

### `.remove(&key)` — remove one entry

```rust
let mut directory: HashMap<String, String> = HashMap::new();
directory.insert("MAS".to_string(), "Chennai Central".to_string());
directory.insert("OLD".to_string(), "Decommissioned Station".to_string());

// Returns Option<V> — the removed value, or None
if let Some(name) = directory.remove("OLD") {
    println!("Removed: {}", name);
}
```

### `.retain(|k, v| condition)` — bulk removal

Keep only stations with names longer than 10 characters:

```rust
directory.retain(|_code, name| name.len() > 10);
```

This is like `.filter()` for iterators, but modifies the HashMap in place.

---

## 6. The Entry API — The Killer Feature

This is the most important section of this lesson. Read it carefully.

### The problem

Imagine you're counting how many trains depart from each city. Without the Entry API, you'd
write something clunky like this:

```rust
let mut counts: HashMap<String, u32> = HashMap::new();

for city in &departures {
    if counts.contains_key(city) {
        // Key exists: increment
        *counts.get_mut(city).unwrap() += 1;
    } else {
        // Key doesn't exist: insert with 1
        counts.insert(city.clone(), 1);
    }
}
```

That's **two** hash lookups per iteration — one to check, one to insert or get. It's
verbose, error-prone, and frankly ugly. Rust can do better.

### `.entry()` to the rescue

The Entry API does it all in **one** lookup:

```rust
let mut counts: HashMap<String, u32> = HashMap::new();

for city in &departures {
    let count = counts.entry(city.clone()).or_insert(0);
    *count += 1;
}
```

Here's what happens step by step:

1. `counts.entry(city.clone())` — look up the key. Returns an `Entry` enum.
2. `.or_insert(0)` — if the key is missing, insert `0` as the default.
3. Either way, return a **mutable reference** `&mut V` to the value.
4. `*count += 1` — dereference and increment.

### The counting pattern — commit this to memory

This is so common, most Rustaceans write it as a one-liner:

```rust
*counts.entry(city.clone()).or_insert(0) += 1;
```

Read it aloud: *"Get the entry for this city, insert 0 if missing, then add 1."*

Real example — counting trains per origin station:

```rust
use std::collections::HashMap;

let trains = vec![
    ("MAS", "Chennai Central"),
    ("MAS", "Chennai Central"),
    ("SBC", "Bangalore City"),
    ("MAS", "Chennai Central"),
    ("NDLS", "New Delhi"),
    ("SBC", "Bangalore City"),
];

let mut departures: HashMap<&str, u32> = HashMap::new();
for (code, _name) in &trains {
    *departures.entry(code).or_insert(0) += 1;
}

// departures: {"MAS": 3, "SBC": 2, "NDLS": 1}
```

### The grouping pattern

Equally powerful — grouping items into categories:

```rust
let trains = vec![
    ("Express", "Rajdhani Express"),
    ("Express", "Shatabdi Express"),
    ("Mail", "Thiruvananthapuram Mail"),
    ("Express", "Duronto Express"),
    ("Mail", "Howrah Mail"),
];

let mut by_type: HashMap<&str, Vec<&str>> = HashMap::new();
for (train_type, name) in &trains {
    by_type.entry(train_type).or_insert_with(Vec::new).push(name);
}

// by_type: {"Express": ["Rajdhani Express", "Shatabdi Express", "Duronto Express"],
//           "Mail": ["Thiruvananthapuram Mail", "Howrah Mail"]}
```

The pattern: `.entry(key).or_insert_with(Vec::new).push(value)`.

### `.or_insert_with(|| ...)` — lazy defaults

`.or_insert(default)` evaluates the default *every time*, even when the key exists.
`.or_insert_with(|| ...)` only evaluates the closure when the key is actually missing:

```rust
// This creates a new Vec every time (wasteful if key usually exists):
map.entry(key).or_insert(Vec::new()).push(value);

// This only creates a Vec when needed (lazy!):
map.entry(key).or_insert_with(Vec::new).push(value);
```

For simple defaults like `0` or `""`, it doesn't matter. For expensive defaults like
`Vec::new()` or database lookups, use `.or_insert_with()`.

### `.or_default()` — even shorter

When the value type implements `Default`, you can skip specifying the default entirely:

```rust
// All equivalent for HashMap<String, Vec<String>>:
map.entry(key).or_insert(Vec::new())
map.entry(key).or_insert_with(Vec::new)
map.entry(key).or_default()               // ← shortest
```

`or_default()` works because `Vec::default()` is an empty Vec, `u32::default()` is `0`,
`String::default()` is `""`, etc.

---

## 7. Ownership

### HashMap takes ownership

When you insert a `String` into a HashMap, the HashMap *owns* it:

```rust
let code = String::from("MAS");
let name = String::from("Chennai Central");

let mut directory: HashMap<String, String> = HashMap::new();
directory.insert(code, name);

// println!("{}", code);  // ❌ ERROR: code has been moved!
// println!("{}", name);  // ❌ ERROR: name has been moved!
```

### Use `.clone()` if you need to keep the original

```rust
let code = String::from("MAS");
directory.insert(code.clone(), name.clone());
println!("{}", code);  // ✅ Still works — we cloned it
```

### References as values: possible, but needs lifetimes

```rust
fn build_lookup<'a>(names: &'a [String]) -> HashMap<&'a str, usize> {
    let mut map = HashMap::new();
    for (i, name) in names.iter().enumerate() {
        map.insert(name.as_str(), i);
    }
    map
}
```

This is valid — the HashMap borrows the strings. But the HashMap can't outlive `names`.
We'll cover lifetimes properly in a later lesson. For now, if the borrow checker complains,
just use owned `String`s.

### Types that implement `Copy` (like `i32`, `bool`, `f64`)

These are *copied* into the HashMap, not moved. You can keep using the originals freely:

```rust
let platform = 5u32;
let mut map: HashMap<String, u32> = HashMap::new();
map.insert("MAS".to_string(), platform);
println!("{}", platform);  // ✅ Fine — u32 is Copy
```

---

## 8. Common Patterns

Here are the HashMap patterns you'll use over and over. Railway-themed, of course.

### Pattern 1: Word / item counting

```rust
let announcements = "The train the express the mail train";
let mut word_counts: HashMap<&str, u32> = HashMap::new();

for word in announcements.split_whitespace() {
    *word_counts.entry(word).or_insert(0) += 1;
}
// {"The": 1, "train": 2, "the": 2, "express": 1, "mail": 1}
```

### Pattern 2: Grouping items by category

```rust
let stations = vec![
    ("Chennai Central", "Tamil Nadu"),
    ("Bangalore City", "Karnataka"),
    ("Coimbatore", "Tamil Nadu"),
    ("Mysuru", "Karnataka"),
    ("Madurai", "Tamil Nadu"),
];

let mut by_state: HashMap<&str, Vec<&str>> = HashMap::new();
for (station, state) in &stations {
    by_state.entry(state).or_insert_with(Vec::new).push(station);
}
// {"Tamil Nadu": ["Chennai Central", "Coimbatore", "Madurai"],
//  "Karnataka": ["Bangalore City", "Mysuru"]}
```

### Pattern 3: Building lookup tables

```rust
let stations = vec![
    ("MAS", "Chennai Central"),
    ("SBC", "Bangalore City"),
    ("NDLS", "New Delhi"),
];

let lookup: HashMap<&str, &str> = stations.into_iter().collect();

if let Some(name) = lookup.get("MAS") {
    println!("MAS is {}", name); // "MAS is Chennai Central"
}
```

### Pattern 4: Inverting a map

Swap keys and values — useful when you need reverse lookups:

```rust
let code_to_name: HashMap<&str, &str> = vec![
    ("MAS", "Chennai Central"),
    ("SBC", "Bangalore City"),
].into_iter().collect();

let name_to_code: HashMap<&str, &str> = code_to_name
    .iter()
    .map(|(k, v)| (*v, *k))
    .collect();

println!("{:?}", name_to_code.get("Chennai Central")); // Some("MAS")
```

⚠️ Warning: if multiple keys have the same value, inverting will lose entries (last one
wins). Make sure values are unique before inverting.

---

## 9. Comparison with TypeScript

| Concept | TypeScript | Rust |
|---------|-----------|------|
| Key-value store | `Map<K, V>` or `{ [key: string]: V }` | `HashMap<K, V>` |
| Import needed? | `Map` is global | `use std::collections::HashMap;` |
| Create empty | `new Map()` or `{}` | `HashMap::new()` |
| Insert | `map.set(k, v)` or `obj[k] = v` | `map.insert(k, v)` |
| Get | `map.get(k)` → `V \| undefined` | `map.get(&k)` → `Option<&V>` |
| Check key | `map.has(k)` or `k in obj` | `map.contains_key(&k)` |
| Delete | `map.delete(k)` | `map.remove(&k)` |
| Size | `map.size` or `Object.keys(obj).length` | `map.len()` |
| Iterate | `for (const [k, v] of map)` | `for (k, v) in &map` |
| Key types | Anything (Map) / strings only (Object) | Must impl `Eq + Hash` |
| Missing key | `undefined` (silent!) | `Option::None` (explicit!) |
| Insert-if-absent | Manual check or `??=` | `entry().or_insert()` 🔥 |

The biggest win in Rust: the Entry API has no real equivalent in JS. You'd write:

```typescript
// TypeScript: awkward
if (!map.has(key)) map.set(key, 0);
map.set(key, map.get(key)! + 1);

// Rust: one expression
*map.entry(key).or_insert(0) += 1;
```

And Rust's `Option` return from `.get()` means you can't accidentally use `undefined`
where you expected a value. The compiler forces you to handle the "not found" case.

---

## 10. Mental Model Summary

```
┌─────────────────────────────────────────────────┐
│              HashMap<K, V>                       │
│                                                  │
│  "MAS" ──→ Station { Chennai Central, 12 plat } │
│  "SBC" ──→ Station { Bangalore City, 10 plat }  │
│  "NDLS" ──→ Station { New Delhi, 16 plat }      │
│                                                  │
│  ⚠️  Unordered — don't rely on iteration order   │
│  ⚠️  Keys must be Eq + Hash (no floats!)         │
│  ⚠️  Owns its keys and values (unless refs)      │
│                                                  │
│  🔑 The Big Three patterns:                      │
│    • .get(&key)              → safe lookup        │
│    • .entry(key).or_insert() → insert-or-update   │
│    • for (k, v) in &map      → iterate            │
└─────────────────────────────────────────────────┘
```

**When to use HashMap vs Vec:**

| Use... | When... |
|--------|---------|
| `Vec<T>` | You need an ordered list, access by index |
| `HashMap<K, V>` | You need fast lookup by key (O(1) average) |
| `Vec<(K, V)>` | You need ordered key-value pairs, or very few entries |

**The Entry API cheat sheet:**

```rust
// Count occurrences:
*map.entry(key).or_insert(0) += 1;

// Group into vectors:
map.entry(key).or_insert_with(Vec::new).push(value);

// Lazy default (only computed if key missing):
map.entry(key).or_insert_with(|| expensive_default());

// Use Default trait:
map.entry(key).or_default();
```

---

## Up Next

In the exercise, you'll build a **Station Directory** — a lookup system for Indian railway
stations using all the HashMap patterns from this lesson. You'll construct directories,
query them safely, count train departures per city, group stations by city, and display
sorted output.

`cd exercise && cargo run` — and don't forget to `use std::collections::HashMap;`!
