# Lesson 11 — Traits

> ⏱️ Estimated reading time: 12 minutes

Traits are Rust's answer to the question: *"How do I write code that works with multiple
types?"* If you've used TypeScript interfaces, you already understand the motivation.
Traits take the idea further — they can carry default implementations, drive compile-time
dispatch, and define shared contracts across types.

Think of it this way: every train, station, and route on a railway network has a
*display board entry*. They're completely different types, but they all share the ability
to describe themselves on the board. A trait is that shared contract.

---

## 1. What Are Traits?

A **trait** defines a set of methods that a type *must* implement.

```rust
trait Displayable {
    fn display_line(&self) -> String;
}
```

Any type can implement this trait — it just needs to provide the method body:

```rust
struct Train { number: u32, name: String }

impl Displayable for Train {
    fn display_line(&self) -> String {
        format!("#{} {}", self.number, self.name)
    }
}
```

### TypeScript comparison

```typescript
// TypeScript — an interface is the same idea
interface Displayable {
    displayLine(): string;
}

class Train implements Displayable {
    displayLine(): string { return `#${this.number} ${this.name}`; }
}
```

The key difference: Rust traits can include *default implementations* (Section 3),
and trait dispatch happens at compile time rather than runtime.

---

## 2. Implementing Traits

You can implement the same trait for as many types as you like:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Train {
    number: u32,
    name: String,
    speed_kmh: u32,
}

struct Station {
    name: String,
    code: String,
    platforms: u8,
}

impl Summary for Train {
    fn summarize(&self) -> String {
        format!("Train #{}: {} ({}km/h)", self.number, self.name, self.speed_kmh)
    }
}

impl Summary for Station {
    fn summarize(&self) -> String {
        format!("Station {} [{}] — {} platforms", self.name, self.code, self.platforms)
    }
}
```

Two different types, one shared interface. Each type provides its own implementation.

---

## 3. Default Implementations

Unlike TypeScript interfaces, traits can provide a **default method body**. Types can
override it or just use the default for free.

```rust
trait Summary {
    fn summarize(&self) -> String;   // required — every type MUST implement this

    fn headline(&self) -> String {   // default — types get this for free
        format!("(Read more about {}...)", self.summarize())
    }
}
```

When you implement `Summary` for a type, you only *need* to implement `summarize()`.
The `headline()` method comes along for free, but you can override it if you want:

```rust
impl Summary for Train {
    fn summarize(&self) -> String {
        format!("#{} {}", self.number, self.name)
    }
    // headline() uses the default — no need to write it
}

impl Summary for Station {
    fn summarize(&self) -> String {
        format!("{} [{}]", self.name, self.code)
    }

    fn headline(&self) -> String {  // override the default
        format!("🚉 Station: {}", self.name)
    }
}
```

---

## 4. Traits as Parameters

Here's where traits become powerful. You can write functions that accept *any type* that
implements a given trait:

```rust
fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

This accepts a `&Train`, a `&Station`, or *any* type that implements `Summary`.

The `impl Trait` syntax has a generic equivalent — covered in Lesson 12.

---

## 5. Returning Traits (`-> impl Trait`)

Just as you can use `impl Trait` in a *parameter* position, you can use it in *return*
position too. The function returns "some type that implements this trait" — the caller
gets the trait interface, but the concrete type is hidden.

```rust
fn make_placeholder_train() -> impl Displayable {
    Train {
        number: 99999,
        name: "Ghost Train".to_string(),
        speed_kmh: 0,
    }
}

let entry = make_placeholder_train();
println!("{}", entry.display_full()); // works — Displayable methods available
// entry.speed_kmh                    // ❌ compiler error: concrete type is hidden
```

### The critical rule: one concrete type per function

`impl Trait` in return position is resolved **at compile time**. Every `return` path must
produce the **same** concrete type:

```rust
// DOES NOT COMPILE — two different concrete types
fn make_entry(flag: bool) -> impl Displayable {
    if flag {
        Train { ... }    // Train
    } else {
        Station { ... }  // Station — compiler error!
    }
}
```

If you need to return different concrete types at runtime, use `Box<dyn Trait>` (dynamic
dispatch) — that's a later topic.

### `-> impl Trait` vs `-> Box<dyn Trait>`

| | `-> impl Trait` | `-> Box<dyn Trait>` |
|---|---|---|
| Dispatch | Compile-time (zero-cost) | Runtime (small heap allocation) |
| Multiple concrete types | No — all paths same type | Yes |
| Common use | iterators, futures | mixed collections, plugins |

### Connection to async

This is exactly how `async fn` works under the hood:

```rust
async fn fetch_status() -> String { ... }
// desugars to:
fn fetch_status() -> impl Future<Output = String> { ... }
```

`impl Future<Output = T>` is `-> impl Trait` in return position. The compiler generates
the concrete future type — you never need to name it.

### TypeScript comparison

```typescript
// TypeScript — return types are structural, erased at runtime
function makeEntry(): Displayable { return new Train(...); }
```

Rust's `-> impl Trait` is zero-cost: the concrete type is baked in at compile time with
no vtable lookup. The caller sees only the trait, but there is no runtime overhead.

---

## 6. Common Standard Library Traits

You'll use these constantly. Some you derive, some you implement manually.

| Trait | What It Does | How You Get It |
|-------|-------------|----------------|
| `Display` | Format with `{}` | Implement manually |
| `Debug` | Format with `{:?}` | `#[derive(Debug)]` |
| `Clone` | `.clone()` — explicit deep copy | `#[derive(Clone)]` |
| `PartialEq` | `==` and `!=` comparison | `#[derive(PartialEq)]` |
| `Default` | `Type::default()` — sensible zero value | `#[derive(Default)]` |
| `From`/`Into` | Type conversion | Implement `From`, get `Into` free |
| `Iterator` | Powers `for` loops and chains | Implement `.next()` |

### Deriving traits

When you write `#[derive(Debug, Clone, PartialEq)]`, the compiler auto-generates
the trait implementations for you. It works as long as every field in your struct
also implements those traits.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Train {
    number: u32,
    name: String,
}
```

---

## 7. The `Display` Trait — Custom Formatting

`Display` controls what happens when you use `{}` in `println!`. Unlike `Debug`, you
must implement it by hand — the compiler can't guess your preferred human-readable format.

```rust
use std::fmt;

struct Train {
    number: u32,
    name: String,
    speed_kmh: u32,
}

impl fmt::Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {} ({}km/h)", self.number, self.name, self.speed_kmh)
    }
}

// Now this works:
let train = Train { number: 12001, name: "Rajdhani Express".to_string(), speed_kmh: 130 };
println!("{}", train);   // #12001 Rajdhani Express (130km/h)
```

This is one of the most useful traits to implement. Once a type has `Display`, it also
gets `.to_string()` for free.

---

## 8. The `From` / `Into` Traits — Type Conversion

`From` defines how to create a type from another type. Implement `From`, and you get
`Into` for free (Rust provides a blanket implementation).

```rust
#[derive(Debug)]
enum TicketClass {
    FirstAC,
    SecondAC,
    Sleeper,
    General,
}

impl From<&str> for TicketClass {
    fn from(s: &str) -> Self {
        match s {
            "1A" => TicketClass::FirstAC,
            "2A" => TicketClass::SecondAC,
            "SL" => TicketClass::Sleeper,
            _ => TicketClass::General,
        }
    }
}

// Two ways to use it:
let class1 = TicketClass::from("1A");     // explicit From
let class2: TicketClass = "SL".into();    // Into (free from implementing From)
```

---

## 9. Comparison with TypeScript

| TypeScript | Rust | Notes |
|-----------|------|-------|
| `interface` | `trait` | Same concept: define shared behavior |
| `implements` | `impl Trait for Type` | Types opt-in to the contract |
| No default method bodies | Default implementations | Traits are more powerful |
| `: Interface` return type | `-> impl Trait` | Hide concrete return type |
| `toString()` override | `impl Display` | Same idea, different mechanism |

The mental model transfers well. Traits are Rust's primary tool for shared behavior.
Generics (covered in Lesson 12) build on this foundation.

---

## 10. Mental Model Summary

```
┌─────────────────────────────────────────────┐
│  trait = "any type that can do X"           │
│  impl Trait for Type = "this type can do X" │
│  impl Trait (param) = "accepts any type X"  │
└─────────────────────────────────────────────┘
```

- **Traits** define *shared behavior* (what methods must exist).
- **impl blocks** connect traits to concrete types.
- **Default implementations** let traits provide free method bodies.
- **`impl Trait` parameters** accept any type that satisfies the trait.
- **`-> impl Trait` returns** hide the concrete type behind a trait interface.
- **Derive macros** auto-implement common traits like `Debug`, `Clone`, `PartialEq`.
- **`Display`** — implement it so your types work with `println!("{}", x)`.
- **`From`/`Into`** — implement `From` for ergonomic type conversions.

Lesson 12 builds on this foundation with generics and trait bounds.
