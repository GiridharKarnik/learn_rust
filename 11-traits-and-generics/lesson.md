# Lesson 11 — Traits & Generics

Traits are Rust's answer to the question: *"How do I write code that works with multiple
types?"* If you've used TypeScript interfaces, you already understand the motivation.
Traits take the idea further — they can carry default implementations, drive compile-time
dispatch, and unlock zero-cost generics.

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

The `&impl Summary` syntax is shorthand for the full **generic** syntax:

```rust
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

Both forms are identical. Use `impl Trait` for simple cases, the generic form when you
need the type parameter `T` in multiple places.

---

## 5. Trait Bounds

What if your function needs a type that implements *multiple* traits? Use `+`:

```rust
fn compare_entries<T: Summary + PartialEq>(a: &T, b: &T) -> bool {
    println!("Comparing: {} vs {}", a.summarize(), b.summarize());
    a == b
}
```

When bounds get long, use a **where clause** for readability:

```rust
fn compare_entries<T>(a: &T, b: &T) -> bool
where
    T: Summary + PartialEq,
{
    println!("Comparing: {} vs {}", a.summarize(), b.summarize());
    a == b
}
```

Both forms are identical — `where` is just easier to read when things get complex.

---

## 6. Generics

Generics let you write code that works with *any* type, optionally constrained by traits.

### Generic structs

```rust
struct Container<T> {
    value: T,
}

let int_box = Container { value: 42 };
let str_box = Container { value: "Rajdhani" };
```

### Generic functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in &list[1..] {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}
```

The `<T>` is a placeholder — Rust fills it in at compile time for each concrete type you
use. This is called **monomorphization**: the compiler generates specialized code for
`largest::<i32>`, `largest::<f64>`, etc. The result is exactly as fast as hand-written
code for each type — **zero-cost abstraction**.

### TypeScript comparison

```typescript
// TypeScript generics — same <T> syntax!
function largest<T>(list: T[]): T { ... }
```

The syntax is almost identical. The difference: TypeScript generics are erased at
runtime. Rust generics are resolved at compile time into specialized machine code.

---

## 7. Common Standard Library Traits

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

## 8. The `Display` Trait — Custom Formatting

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

## 9. The `From` / `Into` Traits — Type Conversion

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

## 10. Comparison with TypeScript

| TypeScript | Rust | Notes |
|-----------|------|-------|
| `interface` | `trait` | Same concept: define shared behavior |
| `implements` | `impl Trait for Type` | Types opt-in to the contract |
| `<T>` generics | `<T>` generics | Same syntax! |
| `extends` (interface) | `T: TraitA + TraitB` | Combine multiple constraints |
| No default method bodies | Default implementations | Traits are more powerful |
| Runtime dispatch (vtable) | Compile-time dispatch (monomorphization) | Rust is zero-cost |
| Type erasure at JS emission | Specialized machine code per type | Rust generics are real |
| `toString()` override | `impl Display` | Same idea, different mechanism |

The mental model transfers well. The biggest adjustment: Rust resolves everything at
compile time. There is no `any` type, no runtime reflection, and no type erasure. If
a generic function compiles, every concrete usage is guaranteed to be type-safe.

---

## 11. Mental Model Summary

```
┌─────────────────────────────────────────────┐
│  trait = "any type that can do X"           │
│  impl Trait for Type = "this type can do X" │
│  <T: Trait> = "give me any T that can do X" │
└─────────────────────────────────────────────┘
```

- **Traits** define *shared behavior* (what methods must exist).
- **impl blocks** connect traits to concrete types.
- **Generics** let you write one function/struct that works with many types.
- **Trait bounds** constrain generics to types that have the behavior you need.
- **Derive macros** auto-implement common traits like `Debug`, `Clone`, `PartialEq`.
- **`Display`** — implement it so your types work with `println!("{}", x)`.
- **`From`/`Into`** — implement `From` for ergonomic type conversions.

When you see `<T: SomeTrait>`, read it as: *"for any type T, as long as T implements
SomeTrait."* That's the whole idea.
