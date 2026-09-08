# Lesson 07 — Collections

In Lesson 06, you mastered `Option`, `Result`, and the `?` operator. You've already used
`Vec` a little — pushing trains onto a list, iterating with `for train in &trains`. Now
we go deeper. Collections are where your data *lives*, and iterators are how you
*process* it.

By the end of this lesson you'll be comfortable with `Vec`, `HashMap`, slices, and — most
importantly — Rust's iterator system, which lets you write expressive, composable data
pipelines that compile down to the same machine code as hand-written loops.

---

## 1. Vec\<T\> — The Growable Array

A `Vec<T>` is a growable, heap-allocated array. It's the workhorse collection — if you
need an ordered list of things, reach for `Vec`.

### Creating

```rust
// Empty vec — you must specify the type (or let Rust infer it from usage)
let mut trains: Vec<String> = Vec::new();

// The vec! macro — creates a Vec with initial values
let speeds = vec![120.0, 95.5, 200.0, 88.0];

// Repeat a value: 10 coaches, all empty
let coaches = vec!["empty"; 10];
```

### Adding elements

```rust
let mut route: Vec<&str> = Vec::new();

// .push() — add to the end (O(1) amortized)
route.push("Edinburgh Waverley");
route.push("Newcastle");
route.push("York");

// .insert(index, value) — insert at a specific position (O(n), shifts everything after)
route.insert(1, "Dunbar");  // Squeeze Dunbar in after Edinburgh

// .extend() — append multiple items from any iterator
route.extend(["Doncaster", "London King's Cross"]);
// route: ["Edinburgh Waverley", "Dunbar", "Newcastle", "York", "Doncaster", "London King's Cross"]
```

### Removing elements

```rust
let mut platforms: Vec<u32> = vec![1, 2, 3, 4, 5, 6];

// .pop() — remove the last element. Returns Option<T>!
let last = platforms.pop();   // Some(6)
let _ = platforms.pop();      // Some(5)
// If the vec is empty: None

// .remove(index) — remove at index, shifts everything after (O(n))
// Panics if index is out of bounds!
let removed = platforms.remove(0);  // removes 1, shifts [2, 3, 4] left
// platforms: [2, 3, 4]

// .retain() — keep only elements matching a predicate (like filter, but in-place)
let mut delays = vec![0, 5, 0, 12, 0, 3];
delays.retain(|&minutes| minutes > 0);
// delays: [5, 12, 3]  — removed all the on-time trains
```

### Accessing elements

This is where `Option` shows up again — remember Lesson 06?

```rust
let stations = vec!["Bristol", "Bath", "Swindon", "Reading", "Paddington"];

// Indexing with [] — panics if out of bounds!
let first = stations[0];      // "Bristol"
// let boom = stations[99];   // 💥 panic: index out of bounds

// .get(index) — returns Option<&T>. Safe!
let maybe = stations.get(2);  // Some(&"Swindon")
let nope = stations.get(99);  // None — no panic

// .first() and .last() — also return Option
let departure = stations.first();  // Some(&"Bristol")
let terminus = stations.last();    // Some(&"Paddington")

// On an empty vec:
let empty: Vec<&str> = vec![];
assert_eq!(empty.first(), None);
assert_eq!(empty.last(), None);
```

**Rule of thumb:** use `.get()` when the index comes from user input or computation.
Use `[i]` only when you're *certain* the index is valid (e.g., you just checked `.len()`).

### Length and capacity

```rust
let mut carriages = vec!["First", "Standard", "Standard", "Quiet"];

carriages.len();       // 4 — number of elements
carriages.is_empty();  // false

// Capacity is how much space the Vec has allocated on the heap.
// It's always >= len. The Vec grows automatically when you push past capacity.
carriages.capacity();  // at least 4, possibly more

// Pre-allocate if you know the size (avoids repeated reallocations)
let mut timetable = Vec::with_capacity(500);
// timetable.len() == 0, but timetable.capacity() >= 500
```

### Ownership: Vec owns its elements

This is the critical thing. A `Vec<String>` *owns* those `String`s. When the `Vec` is
dropped, all its elements are dropped too.

```rust
struct Train {
    name: String,
    speed_kmh: f64,
}

let trains = vec![
    Train { name: "Flying Scotsman".to_string(), speed_kmh: 160.0 },
    Train { name: "Eurostar".to_string(), speed_kmh: 300.0 },
    Train { name: "Shinkansen".to_string(), speed_kmh: 320.0 },
];

// Borrowing — the vec and its elements stay alive
for train in &trains {
    println!("{}: {} km/h", train.name, train.speed_kmh);
}
// trains is still usable here ✅

// Consuming — the vec is moved, you can't use it afterwards
for train in trains {
    println!("Taking ownership of {}", train.name);
}
// trains is gone now — using it here would be a compile error ❌
```

Three ways to iterate:

| Syntax | What you get | Vec afterwards |
|---|---|---|
| `for x in &vec` | `&T` (shared reference) | Still usable |
| `for x in &mut vec` | `&mut T` (mutable reference) | Still usable |
| `for x in vec` | `T` (owned value) | **Consumed — gone** |

---

## 2. Slices &\[T\] — Borrowing Part of a Vec

A slice is a *reference to a contiguous sequence* of elements. Think of it as a window
into a `Vec` (or an array) without copying anything.

```rust
let route = vec!["Edinburgh", "Newcastle", "York", "Doncaster", "London"];

// Slice syntax: &vec[start..end]  (start inclusive, end exclusive)
let middle = &route[1..4];  // ["Newcastle", "York", "Doncaster"]

// From the start:
let first_three = &route[..3];  // ["Edinburgh", "Newcastle", "York"]

// To the end:
let last_two = &route[3..];  // ["Doncaster", "London"]

// Everything:
let all = &route[..];  // the whole thing as a slice
```

### Why slices matter for function signatures

Just like you learned to prefer `&str` over `&String` for string parameters, prefer
`&[T]` over `&Vec<T>` for slice parameters. It's more flexible — the function works
with vecs, arrays, and existing slices.

```rust
// 🔴 Too restrictive — only accepts &Vec<String>
fn print_stations_bad(stations: &Vec<String>) {
    for s in stations {
        println!("  🚉 {}", s);
    }
}

// 🟢 Better — accepts &Vec<String>, &[String], arrays, sub-slices...
fn print_stations(stations: &[String]) {
    for s in stations {
        println!("  🚉 {}", s);
    }
}

let route: Vec<String> = vec![
    "Bristol".to_string(),
    "Bath".to_string(),
    "Paddington".to_string(),
];

print_stations(&route);        // &Vec<String> auto-coerces to &[String]
print_stations(&route[0..2]);  // sub-slice works too
```

Slices work on arrays, too:

```rust
let platform_numbers: [u32; 4] = [1, 5, 12, 14];
let some_platforms = &platform_numbers[1..3];  // &[u32]: [5, 12]
```

---

## 3. HashMap\<K, V\> — Key-Value Pairs

A `HashMap` stores key-value pairs with O(1) average lookup. It's Rust's equivalent of
JavaScript's `Map` (or a plain object used as a dictionary).

```rust
use std::collections::HashMap;  // Not in the prelude — you must import it!
```

### Creating

```rust
use std::collections::HashMap;

// Empty HashMap
let mut departures: HashMap<String, String> = HashMap::new();

// Or let Rust infer the types from usage
let mut departures = HashMap::new();
departures.insert("09:00".to_string(), "Edinburgh → London".to_string());
departures.insert("09:30".to_string(), "Edinburgh → Glasgow".to_string());
departures.insert("10:00".to_string(), "Edinburgh → Aberdeen".to_string());
```

### Creating from an iterator of tuples

```rust
use std::collections::HashMap;

let pairs = vec![
    ("Platform 1", "09:00 to London"),
    ("Platform 5", "09:15 to Glasgow"),
    ("Platform 12", "09:30 to Aberdeen"),
];

let board: HashMap<&str, &str> = pairs.into_iter().collect();
```

### Inserting

```rust
use std::collections::HashMap;

let mut train_speeds: HashMap<&str, f64> = HashMap::new();

// .insert() returns Option<V> — the OLD value if the key already existed
let old = train_speeds.insert("Flying Scotsman", 160.0);
assert_eq!(old, None);  // Key was new

let old = train_speeds.insert("Flying Scotsman", 180.0);  // Overwrites!
assert_eq!(old, Some(160.0));  // Got the old value back
```

### Accessing

```rust
use std::collections::HashMap;

let mut platforms: HashMap<&str, u32> = HashMap::new();
platforms.insert("London Express", 5);
platforms.insert("Highland Chieftain", 11);

// Indexing with [] — panics if key doesn't exist!
let p = platforms["London Express"];  // 5
// let boom = platforms["Ghost Train"];  // 💥 panic!

// .get() — returns Option<&V>. Safe!
let maybe = platforms.get("Highland Chieftain");  // Some(&11)
let nope = platforms.get("Ghost Train");          // None
```

### Removing and checking

```rust
use std::collections::HashMap;

let mut services: HashMap<&str, &str> = HashMap::new();
services.insert("09:00", "London King's Cross");
services.insert("09:30", "Glasgow Central");
services.insert("10:00", "Aberdeen");

// .remove() returns Option<V>
let cancelled = services.remove("09:30");
assert_eq!(cancelled, Some("Glasgow Central"));

// .contains_key()
assert!(services.contains_key("09:00"));
assert!(!services.contains_key("09:30"));  // We just removed it

// .len()
assert_eq!(services.len(), 2);
```

### Iterating

```rust
use std::collections::HashMap;

let mut delays: HashMap<&str, u32> = HashMap::new();
delays.insert("09:00 Edinburgh → London", 5);
delays.insert("09:30 Edinburgh → Glasgow", 0);
delays.insert("10:00 Edinburgh → Aberdeen", 22);

// Iterate over key-value pairs (order is NOT guaranteed!)
for (service, minutes) in &delays {
    if *minutes > 0 {
        println!("⚠️  {} — delayed {} min", service, minutes);
    } else {
        println!("✅ {} — on time", service);
    }
}

// Just keys or just values
for service in delays.keys() {
    println!("Service: {}", service);
}
for minutes in delays.values() {
    println!("Delay: {} min", minutes);
}
```

### The Entry API — the killer feature

The entry API lets you inspect and modify a key's entry in one step. It's the clean way
to handle "insert if missing, update if present."

```rust
use std::collections::HashMap;

let mut passenger_counts: HashMap<&str, u32> = HashMap::new();

// .entry(key).or_insert(default)
// If key doesn't exist → insert default, return mutable reference to value
// If key exists → just return mutable reference to existing value
passenger_counts.entry("Platform 1").or_insert(0);
passenger_counts.entry("Platform 5").or_insert(0);
```

**The classic use case — counting things:**

```rust
use std::collections::HashMap;

let announcements = vec![
    "Platform 1", "Platform 5", "Platform 1", "Platform 12",
    "Platform 1", "Platform 5", "Platform 5", "Platform 1",
];

let mut counts: HashMap<&str, u32> = HashMap::new();

for platform in &announcements {
    let count = counts.entry(platform).or_insert(0);
    *count += 1;  // count is &mut u32, so dereference to increment
}

// counts: {"Platform 1": 4, "Platform 5": 3, "Platform 12": 1}
println!("{:?}", counts);
```

There's also `.or_insert_with(|| ...)` for expensive defaults, and `.and_modify(|v| ...)`
for updating existing values:

```rust
use std::collections::HashMap;

let trains = vec![
    ("Edinburgh", "Scotsman"),
    ("London", "Eurostar"),
    ("Edinburgh", "Highland Chieftain"),
    ("London", "Pendolino"),
    ("Edinburgh", "Caledonian Sleeper"),
];

let mut by_origin: HashMap<&str, Vec<&str>> = HashMap::new();

for (origin, train) in &trains {
    by_origin.entry(origin).or_insert_with(Vec::new).push(train);
}

// by_origin: {"Edinburgh": ["Scotsman", "Highland Chieftain", "Caledonian Sleeper"],
//             "London": ["Eurostar", "Pendolino"]}
```

### Ownership: HashMap takes ownership

For owned types like `String`, the `HashMap` takes ownership of both keys and values:

```rust
use std::collections::HashMap;

let key = String::from("09:00");
let value = String::from("London King's Cross");

let mut map = HashMap::new();
map.insert(key, value);

// key and value are moved — you can't use them anymore!
// println!("{}", key);   // ❌ compile error: value used after move
// println!("{}", value); // ❌ compile error: value used after move
```

For reference types (`&str`, `&T`), the references are copied — no ownership transfer.
Just make sure the referenced data lives long enough.

---

## 4. Iterators — The Power of Rust Collections

This is the big one. Rust's iterator system lets you build processing pipelines that are
expressive, composable, and *zero-cost*. Once this clicks, you'll use iterators everywhere.

### The three iteration methods

Every collection gives you three ways to iterate:

```rust
struct Train {
    name: String,
    speed_kmh: f64,
}

let mut trains = vec![
    Train { name: "Flying Scotsman".to_string(), speed_kmh: 160.0 },
    Train { name: "Eurostar".to_string(), speed_kmh: 300.0 },
    Train { name: "Shinkansen".to_string(), speed_kmh: 320.0 },
];

// .iter() — borrows each element as &T
for train in trains.iter() {
    println!("{}", train.name);  // train is &Train
}

// .iter_mut() — mutably borrows each element as &mut T
for train in trains.iter_mut() {
    train.speed_kmh *= 1.1;  // 10% speed boost! train is &mut Train
}

// .into_iter() — consumes the collection, gives you owned T
for train in trains.into_iter() {
    println!("Now I own {}", train.name);  // train is Train
}
// trains is gone now — consumed by into_iter()
```

Quick reminder: `for train in &trains` is syntactic sugar for `trains.iter()`, and
`for train in trains` is sugar for `trains.into_iter()`.

### Iterator adaptors (lazy!)

Adaptors transform an iterator into a new iterator. They're **lazy** — they don't do
anything until you consume them. You can chain as many as you want.

#### `.map(|x| ...)` — transform each element

```rust
let speeds_kmh = vec![160.0, 300.0, 320.0, 200.0];

// Convert km/h to mph
let speeds_mph: Vec<f64> = speeds_kmh.iter()
    .map(|kmh| kmh * 0.621371)
    .collect();
// [99.4, 186.4, 198.8, 124.3]
```

#### `.filter(|x| ...)` — keep elements matching a condition

```rust
let delays_minutes = vec![0, 5, 0, 22, 3, 0, 45, 0];

let late_trains: Vec<&u32> = delays_minutes.iter()
    .filter(|&&mins| mins > 0)
    .collect();
// [&5, &22, &3, &45]
```

Note: with `.iter()`, the filter closure gets `&&T` (a reference to a reference) because
`.iter()` yields `&T`, and `.filter()` borrows that again. You'll get used to it.

#### `.enumerate()` — attach an index

```rust
let stations = vec!["Edinburgh", "Newcastle", "York", "London"];

for (i, station) in stations.iter().enumerate() {
    println!("Stop {}: {}", i + 1, station);
}
// Stop 1: Edinburgh
// Stop 2: Newcastle
// Stop 3: York
// Stop 4: London
```

#### `.zip(other)` — pair elements from two iterators

```rust
let trains = vec!["Scotsman", "Eurostar", "Pendolino"];
let platforms = vec![1, 5, 12];

let assignments: Vec<(&str, &u32)> = trains.iter()
    .copied()
    .zip(platforms.iter())
    .collect();
// [("Scotsman", &1), ("Eurostar", &5), ("Pendolino", &12)]

// More commonly used directly:
for (train, platform) in trains.iter().zip(platforms.iter()) {
    println!("{} departs from Platform {}", train, platform);
}
```

#### `.take(n)` and `.skip(n)` — slice the iterator

```rust
let all_stops = vec!["Edinburgh", "Dunbar", "Newcastle", "York", "Doncaster", "London"];

// First 3 stops
let first_leg: Vec<&&str> = all_stops.iter().take(3).collect();
// ["Edinburgh", "Dunbar", "Newcastle"]

// Skip first 3, take the rest
let second_leg: Vec<&&str> = all_stops.iter().skip(3).collect();
// ["York", "Doncaster", "London"]
```

#### `.chain(other)` — concatenate two iterators

```rust
let northbound = vec!["London", "York", "Edinburgh"];
let southbound = vec!["Glasgow", "Carlisle", "Preston"];

let all_services: Vec<&&str> = northbound.iter()
    .chain(southbound.iter())
    .collect();
// ["London", "York", "Edinburgh", "Glasgow", "Carlisle", "Preston"]
```

#### `.flat_map(|x| ...)` — map and flatten in one step

```rust
let routes = vec![
    vec!["Edinburgh", "Newcastle", "London"],
    vec!["Glasgow", "Preston", "Birmingham"],
    vec!["Aberdeen", "Dundee", "Edinburgh"],
];

// Get all unique station names (well, all stations — may have duplicates)
let all_stations: Vec<&&str> = routes.iter()
    .flat_map(|route| route.iter())
    .collect();
// ["Edinburgh", "Newcastle", "London", "Glasgow", "Preston", "Birmingham", "Aberdeen", "Dundee", "Edinburgh"]
```

#### `.inspect(|x| ...)` — peek at elements without modifying them (great for debugging)

```rust
let result: Vec<f64> = vec![160.0, 80.0, 300.0, 95.0, 200.0]
    .into_iter()
    .inspect(|speed| println!("  checking: {} km/h", speed))
    .filter(|&speed| speed > 100.0)
    .inspect(|speed| println!("  ✅ passed: {} km/h", speed))
    .collect();
// Prints:
//   checking: 160 km/h
//   ✅ passed: 160 km/h
//   checking: 80 km/h
//   checking: 300 km/h
//   ✅ passed: 300 km/h
//   checking: 95 km/h
//   checking: 200 km/h
//   ✅ passed: 200 km/h
// result: [160.0, 300.0, 200.0]
```

### Consumers (actually run the pipeline)

Adaptors are lazy. **Consumers** pull values through the chain and produce a final result.

#### `.collect()` — gather into a collection

We've been using this one already. It can produce a `Vec`, `HashMap`, `String`, and more.
See Section 5 for the full story.

#### `.count()` — how many elements

```rust
let stations = vec!["Edinburgh", "Newcastle", "York", "London"];
let count = stations.iter().filter(|s| s.starts_with('N')).count();
// 1  (just "Newcastle")
```

#### `.sum()` and `.product()` — numeric aggregation

```rust
let delays: Vec<u32> = vec![5, 12, 3, 22, 0, 8];

let total_delay: u32 = delays.iter().sum();
// 50

let carriages = vec![8, 12, 6, 10];
let total_seats: u32 = carriages.iter().map(|&c| c * 60).sum();
// (8 + 12 + 6 + 10) * 60 = 2160
```

#### `.min()` and `.max()` — returns Option (empty iterator → None)

```rust
let speeds = vec![160.0_f64, 300.0, 95.0, 200.0];

// For f64, use .min_by() / .max_by() because floats don't implement Ord
// (NaN is weird). For integers:
let platform_numbers = vec![1, 5, 12, 3, 8];
let highest = platform_numbers.iter().max();  // Some(&12)
let lowest = platform_numbers.iter().min();   // Some(&1)

let empty: Vec<i32> = vec![];
assert_eq!(empty.iter().max(), None);
```

#### `.find(|x| ...)` — first element matching a condition (returns Option)

```rust
struct Train {
    name: String,
    platform: u32,
}

let trains = vec![
    Train { name: "Scotsman".to_string(), platform: 1 },
    Train { name: "Eurostar".to_string(), platform: 5 },
    Train { name: "Pendolino".to_string(), platform: 12 },
];

let on_platform_5 = trains.iter().find(|t| t.platform == 5);
// Some(&Train { name: "Eurostar", platform: 5 })

let on_platform_99 = trains.iter().find(|t| t.platform == 99);
// None
```

#### `.any(|x| ...)` and `.all(|x| ...)` — boolean checks

```rust
let delays = vec![0, 0, 5, 0, 0];

let any_late = delays.iter().any(|&d| d > 0);    // true — at least one delay
let all_on_time = delays.iter().all(|&d| d == 0); // false — not all are 0
```

#### `.fold(init, |acc, x| ...)` — reduce to a single value

This is the most general consumer. `.sum()`, `.count()`, `.max()` — they're all
special cases of fold.

```rust
struct Train {
    name: String,
    passengers: u32,
}

let trains = vec![
    Train { name: "Scotsman".to_string(), passengers: 350 },
    Train { name: "Eurostar".to_string(), passengers: 750 },
    Train { name: "Pendolino".to_string(), passengers: 420 },
];

// Total passengers across all trains
let total = trains.iter().fold(0u32, |total, train| {
    total + train.passengers
});
// 1520

// Build a departure announcement string
let announcement = trains.iter().fold(String::new(), |mut msg, train| {
    msg.push_str(&format!("🚂 {} ({} passengers)\n", train.name, train.passengers));
    msg
});
```

#### `.for_each(|x| ...)` — like a for loop, but chainable

```rust
let stations = vec!["Edinburgh", "Newcastle", "York", "London"];

stations.iter()
    .enumerate()
    .for_each(|(i, station)| {
        println!("  Stop {}: 🚉 {}", i + 1, station);
    });
```

### Key insight: iterator chains are zero-cost

Here's the thing that makes Rust iterators special. The compiler optimizes iterator
chains into the same machine code as hand-written loops. This isn't an abstraction you
pay for — it's often *faster* than a manual loop because the compiler can vectorize it.

**Before (imperative loop):**

```rust
struct Train {
    name: String,
    speed_kmh: f64,
}

let trains = vec![
    Train { name: "Scotsman".to_string(), speed_kmh: 160.0 },
    Train { name: "Eurostar".to_string(), speed_kmh: 300.0 },
    Train { name: "Local Stopper".to_string(), speed_kmh: 80.0 },
    Train { name: "Shinkansen".to_string(), speed_kmh: 320.0 },
];

// Imperative — totally fine, but verbose
let mut fast_train_names = Vec::new();
for train in &trains {
    if train.speed_kmh > 100.0 {
        fast_train_names.push(train.name.clone());
    }
}
```

**After (iterator chain):**

```rust
// Functional — same performance, more concise
let fast_train_names: Vec<String> = trains.iter()
    .filter(|t| t.speed_kmh > 100.0)
    .map(|t| t.name.clone())
    .collect();
```

Same result. Same machine code. The iterator version is more concise, easier to compose,
and clearly separates the *what* (filter fast trains, extract names) from the *how*
(looping, pushing, allocating).

Use whichever style reads better for the situation. Simple loops are fine. But for
multi-step transformations, iterator chains are almost always cleaner.

---

## 5. Collecting into Different Types

`.collect()` is secretly very powerful. It uses Rust's type system to decide *what*
collection to build. You just tell it the target type.

### Into a Vec

```rust
let names: Vec<String> = vec!["Scotsman", "Eurostar", "Pendolino"]
    .iter()
    .map(|s| s.to_uppercase())
    .collect();
// ["SCOTSMAN", "EUROSTAR", "PENDOLINO"]
```

### Into a HashMap (from tuples)

```rust
use std::collections::HashMap;

let trains = vec!["Scotsman", "Eurostar", "Pendolino"];
let platforms = vec![1, 5, 12];

let board: HashMap<&str, &u32> = trains.iter()
    .copied()
    .zip(platforms.iter())
    .collect();
// {"Scotsman": 1, "Eurostar": 5, "Pendolino": 12}
```

Any iterator of `(K, V)` tuples can be collected into a `HashMap`.

### Into a String

```rust
let stations = vec!["Edinburgh", "Newcastle", "York", "London"];

// Join with an iterator of chars
let initials: String = stations.iter()
    .map(|s| s.chars().next().unwrap())  // first char of each
    .collect();
// "ENYL"

// Join with a separator — use .join() on a slice instead
let route_display = stations.join(" → ");
// "Edinburgh → Newcastle → York → London"
```

### The turbofish `::<>`

Sometimes Rust can infer the target type from context (e.g., the variable's type
annotation). When it can't, you use the *turbofish* syntax:

```rust
let stations = vec!["Edinburgh", "Newcastle", "York", "London"];

// With a type annotation — Rust knows what to collect into
let upper: Vec<String> = stations.iter().map(|s| s.to_uppercase()).collect();

// With turbofish — equivalent, sometimes more convenient inline
let upper = stations.iter().map(|s| s.to_uppercase()).collect::<Vec<String>>();

// You can use _ for parts Rust can figure out
let upper = stations.iter().map(|s| s.to_uppercase()).collect::<Vec<_>>();
```

The turbofish is just a type hint. Use it when the compiler says "type annotations needed"
and you don't have (or want) a `let` binding to annotate.

---

## 6. Common Patterns

These come up constantly. Bookmark this section.

### Counting occurrences

```rust
use std::collections::HashMap;

let destinations = vec![
    "London", "Edinburgh", "London", "Glasgow",
    "London", "Edinburgh", "Aberdeen", "London",
];

let mut counts: HashMap<&str, u32> = HashMap::new();
for dest in &destinations {
    *counts.entry(dest).or_insert(0) += 1;
}
// {"London": 4, "Edinburgh": 2, "Glasgow": 1, "Aberdeen": 1}
```

Or, more concisely with `.fold()`:

```rust
use std::collections::HashMap;

let destinations = vec![
    "London", "Edinburgh", "London", "Glasgow",
    "London", "Edinburgh", "Aberdeen", "London",
];

let counts = destinations.iter().fold(HashMap::new(), |mut acc, dest| {
    *acc.entry(dest).or_insert(0u32) += 1;
    acc
});
```

### Grouping items

```rust
use std::collections::HashMap;

struct Service {
    origin: String,
    destination: String,
    train: String,
}

let services = vec![
    Service { origin: "Edinburgh".into(), destination: "London".into(), train: "Scotsman".into() },
    Service { origin: "Edinburgh".into(), destination: "Glasgow".into(), train: "Shuttle".into() },
    Service { origin: "London".into(), destination: "Edinburgh".into(), train: "LNER".into() },
    Service { origin: "Edinburgh".into(), destination: "Aberdeen".into(), train: "Highland".into() },
];

let mut by_origin: HashMap<&str, Vec<&str>> = HashMap::new();
for svc in &services {
    by_origin.entry(&svc.origin).or_insert_with(Vec::new).push(&svc.destination);
}
// {"Edinburgh": ["London", "Glasgow", "Aberdeen"], "London": ["Edinburgh"]}
```

### Finding min/max with iterators

```rust
struct Train {
    name: String,
    speed_kmh: f64,
}

let trains = vec![
    Train { name: "Scotsman".to_string(), speed_kmh: 160.0 },
    Train { name: "Eurostar".to_string(), speed_kmh: 300.0 },
    Train { name: "Local".to_string(), speed_kmh: 80.0 },
];

// Fastest train (using max_by for f64 comparison)
let fastest = trains.iter()
    .max_by(|a, b| a.speed_kmh.partial_cmp(&b.speed_kmh).unwrap());

if let Some(train) = fastest {
    println!("🏆 Fastest: {} at {} km/h", train.name, train.speed_kmh);
}
// 🏆 Fastest: Eurostar at 300 km/h
```

### Transforming Vec\<A\> into Vec\<B\>

```rust
struct RawSchedule {
    train_id: u32,
    departure: String,
}

struct DisplaySchedule {
    label: String,
}

let raw = vec![
    RawSchedule { train_id: 1001, departure: "09:00".to_string() },
    RawSchedule { train_id: 1002, departure: "09:30".to_string() },
    RawSchedule { train_id: 1003, departure: "10:00".to_string() },
];

let display: Vec<DisplaySchedule> = raw.iter()
    .map(|r| DisplaySchedule {
        label: format!("Train #{} departing {}", r.train_id, r.departure),
    })
    .collect();
```

### Chaining multiple operations

```rust
struct Departure {
    train: String,
    delay_minutes: u32,
    platform: u32,
}

let departures = vec![
    Departure { train: "Scotsman".into(), delay_minutes: 0, platform: 1 },
    Departure { train: "Eurostar".into(), delay_minutes: 15, platform: 5 },
    Departure { train: "Local".into(), delay_minutes: 45, platform: 3 },
    Departure { train: "LNER".into(), delay_minutes: 5, platform: 1 },
    Departure { train: "Highland".into(), delay_minutes: 0, platform: 12 },
];

// "Which delayed trains are on platform 1?"
let delayed_on_p1: Vec<&str> = departures.iter()
    .filter(|d| d.platform == 1)
    .filter(|d| d.delay_minutes > 0)
    .map(|d| d.train.as_str())
    .collect();
// ["LNER"]

// "What's the average delay of delayed trains?"
let delayed: Vec<&Departure> = departures.iter()
    .filter(|d| d.delay_minutes > 0)
    .collect();

if !delayed.is_empty() {
    let avg = delayed.iter()
        .map(|d| d.delay_minutes as f64)
        .sum::<f64>() / delayed.len() as f64;
    println!("Average delay: {:.1} minutes", avg);
    // Average delay: 21.7 minutes
}
```

---

## 7. Comparison with TypeScript

If you're coming from JavaScript/TypeScript, here's your Rosetta Stone:

| TypeScript | Rust | Notes |
|---|---|---|
| `[]` / `Array<T>` | `Vec<T>` | Both growable, heap-allocated |
| `{}` / `Map<K,V>` | `HashMap<K, V>` | Must import `HashMap` |
| `arr.map(fn)` | `arr.iter().map(fn).collect()` | Rust is lazy, needs `.collect()` |
| `arr.filter(fn)` | `arr.iter().filter(fn).collect()` | Same — lazy + collect |
| `arr.reduce(fn, init)` | `arr.iter().fold(init, fn)` | Arguments in different order |
| `arr.find(fn)` | `arr.iter().find(fn)` | Both return optional |
| `arr.some(fn)` / `arr.every(fn)` | `arr.iter().any(fn)` / `arr.iter().all(fn)` | Same semantics |
| `arr.flat()` | `arr.iter().flatten().collect()` | |
| `arr.flatMap(fn)` | `arr.iter().flat_map(fn).collect()` | |
| `arr.slice(1, 3)` | `&arr[1..3]` | Rust slices are zero-copy borrows |
| `arr.forEach(fn)` | `arr.iter().for_each(fn)` | Or just a `for` loop |

### The big difference: laziness

In JavaScript, `.map()` returns a new array immediately. Every step allocates:

```javascript
// JavaScript — each step creates a new array in memory
const result = trains
    .filter(t => t.speed > 100)   // new array allocated
    .map(t => t.name)             // another new array allocated
    .slice(0, 5);                 // yet another new array
```

In Rust, adaptors are **lazy**. Nothing happens until a consumer (like `.collect()`) pulls
values through the chain. The compiler fuses the whole pipeline into a single pass:

```rust
// Rust — one pass through the data, one allocation for the final Vec
let result: Vec<&str> = trains.iter()
    .filter(|t| t.speed > 100.0)   // lazy — just records the filter
    .map(|t| t.name.as_str())      // lazy — just records the transform
    .take(5)                        // lazy — just records the limit
    .collect();                     // NOW it executes — one pass, one allocation
```

No intermediate collections. No wasted allocations. Zero-cost.

---

## 8. Mental Model Summary

> **🧠 Key Concepts**
>
> 1. **`Vec<T>`** is your go-to ordered collection. It owns its elements, grows
>    dynamically, and gives you `.push()`, `.pop()`, `.get()`, and everything
>    you'd expect from a dynamic array.
>
> 2. **Slices `&[T]`** are borrowed views into a `Vec` or array. Prefer `&[T]`
>    over `&Vec<T>` in function signatures — it's more flexible, same as
>    preferring `&str` over `&String`.
>
> 3. **`HashMap<K, V>`** is your key-value store. Always `use std::collections::HashMap`.
>    The **entry API** (`.entry(key).or_insert(default)`) is the idiomatic way to
>    count, group, and conditionally insert.
>
> 4. **Three ways to iterate:**
>    - `.iter()` → `&T` (borrow)
>    - `.iter_mut()` → `&mut T` (mutable borrow)
>    - `.into_iter()` → `T` (owned, consumes the collection)
>
> 5. **Adaptors are lazy.** `.map()`, `.filter()`, `.take()`, `.skip()`, `.chain()`,
>    `.zip()`, `.enumerate()` — none of these execute until a consumer pulls on them.
>
> 6. **Consumers drive the pipeline.** `.collect()`, `.sum()`, `.count()`, `.find()`,
>    `.fold()`, `.any()`, `.all()`, `.for_each()` — these trigger the actual work.
>
> 7. **`.collect()` is polymorphic.** It builds whatever collection the type system
>    demands: `Vec<T>`, `HashMap<K, V>`, `String`, and more. Use turbofish
>    (`::<Vec<_>>()`) when the compiler needs a hint.
>
> 8. **Iterator chains are zero-cost.** The compiler optimizes them into the same
>    machine code as hand-written loops. Write expressive pipelines without guilt.
>
> 9. **Ownership still matters.** `Vec` owns its elements; `HashMap` owns its keys
>    and values. Iterating with `&` borrows, iterating without `&` consumes.
>    This is Rust being Rust — no surprises if you remember Lesson 03.

---

## Up Next

Run the examples in the `examples/` directory to see these collections and iterator
patterns in action, then tackle the exercise — a Railway Departure Board system that
puts `Vec`, `HashMap`, and iterator chains to work! 🚂

