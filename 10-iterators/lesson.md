# Lesson 09 — Iterators

In Lesson 07 you learned to store data in `Vec`, `HashMap`, and slices. You even used
a few iterator methods like `.iter().map().collect()`. Now we're going to blow the doors
off. Iterators are the **core data-processing tool** in Rust — once you're comfortable
with them, you'll reach for iterator chains as naturally as you reach for a `for` loop.

Think of it like a railway signal chain: each signal in the line inspects the train and
decides what to do — let it pass, reroute it, tag it with a number. The train only
actually *moves* when the final signal pulls it through.

---

## 1. What Is an Iterator?

An iterator is an object that produces a sequence of values **one at a time**. That's it.

In Rust, any type that implements the `Iterator` trait is an iterator. The trait is
surprisingly simple:

```rust
trait Iterator {
    type Item;                    // The type of value this iterator produces
    fn next(&mut self) -> Option<Self::Item>;  // Give me the next value
}
```

Every call to `.next()` returns:
- `Some(value)` — here's the next element
- `None` — we're done, no more elements

That's the whole contract. Everything else — `.map()`, `.filter()`, `.sum()` — is built
on top of this one method.

### Three Ways to Get an Iterator from a Collection

Every standard collection (`Vec`, `HashMap`, slices, etc.) gives you three ways to
iterate:

| Method         | Yields    | Ownership                          |
|----------------|-----------|------------------------------------|
| `.iter()`      | `&T`      | Borrows — collection still usable  |
| `.iter_mut()`  | `&mut T`  | Mutably borrows — can modify items |
| `.into_iter()` | `T`       | Takes ownership — consumes collection |

```rust
let trains = vec!["Rajdhani", "Shatabdi", "Duronto"];

// Borrow: peek at each train, vec still usable afterward
for name in trains.iter() {
    println!("{}", name);  // name is &str here
}

// Mutable borrow: modify in place
let mut speeds = vec![100, 120, 95];
for speed in speeds.iter_mut() {
    *speed += 10;  // boost every train's speed
}

// Ownership: the vec is consumed, you can't use it after this
let names = vec!["Rajdhani".to_string(), "Shatabdi".to_string()];
for name in names.into_iter() {
    println!("{}", name);  // name is String (owned)
}
// names is gone here — moved into the iterator
```

> **Note:** `for item in &collection` is sugar for `for item in collection.iter()`, and
> `for item in &mut collection` is sugar for `for item in collection.iter_mut()`. You'll
> see both styles in the wild.

---

## 2. Iterator Adaptors (Lazy — Do Nothing Until Consumed)

Adaptors transform an iterator into a *new* iterator. They're **lazy** — they don't
execute anything. They just set up a pipeline.

Think of them as track switches in a rail yard: you can line up a complex route, but no
train moves until someone pulls the signal.

### The Key Adaptors

```rust
let trains = vec![120, 95, 200, 88, 150, 60];

// .map() — transform each element
// "Recalculate each speed from km/h to mph"
trains.iter().map(|&speed| speed as f64 * 0.621)
// Still lazy! Nothing computed yet.

// .filter() — keep elements that match a condition
// "Only fast trains"
trains.iter().filter(|&&speed| speed > 100)

// .enumerate() — attach an index (0, 1, 2, ...)
// "Number each platform"
trains.iter().enumerate()  // yields (0, &120), (1, &95), ...

// .zip() — pair two iterators element by element
let names = vec!["Rajdhani", "Shatabdi", "Duronto"];
let speeds = vec![130, 150, 120];
names.iter().zip(speeds.iter())  // yields (&"Rajdhani", &130), ...

// .take(n) / .skip(n) — first n / skip first n
trains.iter().take(3)   // first 3 speeds
trains.iter().skip(2)   // skip first 2

// .chain() — concatenate two iterators
let express = vec![120, 150];
let local = vec![60, 55];
express.iter().chain(local.iter())  // 120, 150, 60, 55

// .flatten() — flatten nested iterators
let routes: Vec<Vec<&str>> = vec![
    vec!["Chennai", "Katpadi", "Bangalore"],
    vec!["Mumbai", "Pune"],
];
routes.iter().flatten()  // "Chennai", "Katpadi", "Bangalore", "Mumbai", "Pune"

// .inspect() — peek at each element (useful for debugging)
trains.iter()
    .inspect(|speed| println!("  before filter: {}", speed))
    .filter(|&&speed| speed > 100)
    .inspect(|speed| println!("  after filter: {}", speed))
```

### 🔴 KEY INSIGHT: Laziness

This is the #1 thing newcomers miss:

```rust
// THIS DOES NOTHING!
trains.iter().map(|speed| speed * 2);

// The compiler will warn you:
// "unused `Map` that must be used — iterators are lazy and do nothing unless consumed"
```

The adaptor just *describes* a transformation. You need a **consumer** to actually
execute the chain.

---

## 3. Iterator Consumers (Execute the Chain)

Consumers pull values through the chain and produce a final result. Once you call a
consumer, the whole pipeline runs.

```rust
let speeds = vec![120, 95, 200, 88, 150, 60];

// .collect() — gather results into a collection
let fast: Vec<&i32> = speeds.iter().filter(|&&s| s > 100).collect();

// .count() — how many elements?
let fast_count = speeds.iter().filter(|&&s| s > 100).count();
// 3

// .sum() — add them up (works on numeric iterators)
let total: i32 = speeds.iter().sum();
// 713

// .product() — multiply them all
let product: i32 = vec![2, 3, 4].iter().product();
// 24

// .min() / .max() — smallest / largest (returns Option)
let fastest: Option<&i32> = speeds.iter().max();
// Some(&200)

// .min_by_key() / .max_by_key() — custom key for comparison
let trains = vec![("Rajdhani", 130), ("Shatabdi", 150), ("Local", 60)];
let fastest = trains.iter().max_by_key(|t| t.1);
// Some(&("Shatabdi", 150))

// .min_by() / .max_by() — full custom comparator (needed for f64)
let fares = vec![350.0_f64, 755.5, 4250.0];
let cheapest = fares.iter().min_by(|a, b| a.partial_cmp(b).unwrap());
// Some(&350.0)

// .find() — first element matching a condition (returns Option)
let first_fast = speeds.iter().find(|&&s| s > 100);
// Some(&120)

// .any() / .all() — boolean checks
let has_fast = speeds.iter().any(|&s| s > 100);    // true
let all_fast = speeds.iter().all(|&s| s > 100);    // false

// .fold() — reduce to a single value with an accumulator
let total = speeds.iter().fold(0, |acc, &speed| acc + speed);
// 713

// .for_each() — like a for loop (useful at the end of a chain)
speeds.iter()
    .filter(|&&s| s > 100)
    .for_each(|speed| println!("Fast train: {} km/h", speed));
```

---

## 4. Collecting into Different Types

`.collect()` is powerful because it can produce **many different collection types**.
The catch: the compiler needs to know *which* type you want.

### The Turbofish `::<>`

```rust
let speeds = vec![120, 95, 200, 88];

// Collect into a Vec
let doubled = speeds.iter().map(|s| s * 2).collect::<Vec<_>>();
// [240, 190, 400, 176]

// Collect (key, value) tuples into a HashMap
let entries = vec![("Rajdhani", 130), ("Shatabdi", 150)];
let speed_map = entries.into_iter().collect::<HashMap<_, _>>();
// {"Rajdhani": 130, "Shatabdi": 150}

// Collect chars or &strs into a String
let code: String = vec!['I', 'R', 'C', 'T', 'C'].into_iter().collect();
// "IRCTC"
let announcement: String = vec!["Train", " ", "arriving"].into_iter().collect();
// "Train arriving"
```

The `::<>` syntax is affectionately called the **turbofish**. It tells `.collect()` what
type to produce. The `_` means "compiler, you figure out the element type."

### Alternative: Type Annotation

If you don't like the turbofish, you can annotate the variable instead:

```rust
let doubled: Vec<_> = speeds.iter().map(|s| s * 2).collect();
// Same result, no turbofish needed
```

Both styles are common. Use whichever reads better in context.

---

## 5. Chaining — Where It All Comes Together

This is where iterators really shine. You can build complex data transformations as a
single readable pipeline.

```rust
use std::collections::HashMap;

struct Train {
    name: String,
    speed_kmh: f64,
    fare: f64,
    route: String,
}

// Example 1: Filter, transform, join
// "Which express trains go faster than 100 km/h?"
let fast_train_names: String = trains.iter()
    .filter(|t| t.speed_kmh > 100.0)
    .map(|t| t.name.as_str())
    .collect::<Vec<_>>()
    .join(", ");
// "Rajdhani Express, Shatabdi Express, Duronto Express"

// Example 2: Group and aggregate
// "Total fare collected by route"
let fare_by_route: HashMap<&str, f64> = trains.iter()
    .map(|t| (t.route.as_str(), t.fare))
    .fold(HashMap::new(), |mut acc, (route, fare)| {
        *acc.entry(route).or_insert(0.0) += fare;
        acc
    });

// Example 3: Complex pipeline with enumerate
// "Numbered departure board"
let board: String = trains.iter()
    .filter(|t| t.speed_kmh > 80.0)
    .enumerate()
    .map(|(i, t)| format!("{}. {} ({} km/h)", i + 1, t.name, t.speed_kmh))
    .collect::<Vec<_>>()
    .join("\n");

// Example 4: Zip for parallel data
let names = vec!["Platform 1", "Platform 2", "Platform 3"];
let trains_on_platform = vec!["Rajdhani", "Shatabdi", "Duronto"];
let display: Vec<String> = names.iter()
    .zip(trains_on_platform.iter())
    .map(|(platform, train)| format!("{}: {}", platform, train))
    .collect();
```

Read a chain like a sentence: "Take the trains, *filter* by speed, *map* to names,
*collect* into a Vec, *join* with commas."

---

## 6. `for` Loop vs Iterator Chain

Here's the thing that surprises people coming from other languages: **they compile to
the same machine code**. Rust's iterators are a zero-cost abstraction.

```rust
// for loop version
let mut fast_names = Vec::new();
for train in &trains {
    if train.speed_kmh > 100.0 {
        fast_names.push(&train.name);
    }
}

// Iterator chain version
let fast_names: Vec<&String> = trains.iter()
    .filter(|t| t.speed_kmh > 100.0)
    .map(|t| &t.name)
    .collect();

// Same performance. Same machine code. Choose based on readability.
```

### When to Use Which

| Use a `for` loop when...                    | Use an iterator chain when...                    |
|---------------------------------------------|--------------------------------------------------|
| You're doing side effects (printing, I/O)   | You're building a new collection from an old one |
| The logic has complex branching              | The transformation is a clean pipeline           |
| You need to mutate multiple things           | You want to express "what" not "how"             |
| You need early `break` or `continue`        | Chaining reads naturally as a sentence           |

**Rule of thumb:** If you're *transforming data* (filter, map, collect), use iterators.
If you're *doing things* (printing, writing to a file, updating a counter), use a
`for` loop.

---

## 7. Comparison with TypeScript

If you know JavaScript or TypeScript, iterators will feel familiar — with one critical
twist.

| JS / TS                       | Rust                                    | Key Difference                    |
|-------------------------------|-----------------------------------------|-----------------------------------|
| `arr.map(fn)`                 | `arr.iter().map(fn).collect()`          | Rust is lazy, needs `.collect()`  |
| `arr.filter(fn)`              | `arr.iter().filter(fn).collect()`       | Same                              |
| `arr.reduce(fn, init)`        | `arr.iter().fold(init, fn)`             | `.fold()` takes initial value first |
| `arr.find(fn)`                | `arr.iter().find(fn)`                   | Returns `Option<T>` — similar to TS's `T \| undefined` with `strictNullChecks`, but enforced at runtime too |
| `arr.some(fn)` / `arr.every(fn)` | `arr.iter().any(fn)` / `arr.iter().all(fn)` | Same idea                |
| `arr.forEach(fn)`             | `arr.iter().for_each(fn)`               | Same                              |
| `arr.flat()`                  | `arr.iter().flatten().collect()`        | Same idea, lazy                   |
| `arr.slice(0, n)`             | `arr.iter().take(n).collect()`          | Same idea                         |
| Eager (runs immediately)      | Lazy (runs on consume)                  | **Fundamental difference**        |

The laziness is the big conceptual shift. In JS, `[1,2,3].map(x => x * 2)` runs
immediately and returns `[2,4,6]`. In Rust, `.map(|x| x * 2)` returns an iterator
adapter that hasn't done anything yet. You must consume it (`.collect()`, `.sum()`,
`.for_each()`, etc.) to trigger execution.

**Why laziness matters:** It means Rust never allocates intermediate arrays. A chain
like `.filter().map().take(5).collect()` processes elements one at a time — it doesn't
create a filtered array, then a mapped array, then slice it. It pulls each element
through the whole pipeline, one by one, and stops after 5 matches.

---

## 8. Mental Model Summary

```
    Source              Adaptors (lazy)              Consumer (executes)
  ┌─────────┐     ┌──────────────────────┐       ┌─────────────────┐
  │ .iter() │ ──▶ │ .filter() .map() ... │ ──▶   │ .collect()      │
  │         │     │ (just set up the     │       │ .sum()          │
  │ Vec     │     │  pipeline — nothing  │       │ .count()        │
  │ HashMap │     │  happens yet)        │       │ .for_each()     │
  │ slice   │     │                      │       │ .fold()         │
  └─────────┘     └──────────────────────┘       └─────────────────┘
```

1. **Source** — Get an iterator from a collection (`.iter()`, `.iter_mut()`, `.into_iter()`)
2. **Adapt** — Chain zero or more adaptors (`.filter()`, `.map()`, `.enumerate()`, ...)
3. **Consume** — Pull everything through with a consumer (`.collect()`, `.sum()`, `.fold()`, ...)

The whole chain executes **lazily, element by element**, with no intermediate allocations.
It compiles to the same machine code as a hand-written loop.

That's iterators. You have the full toolkit now — go build some data pipelines! 🚂
