# Lesson 04 — Structs & Methods

Structs are how you create **custom data types** in Rust. If you've used classes in
TypeScript/Python/Java, structs fill the same role — but without inheritance.

---

## 1. Defining a Struct

A struct groups related data under one name:

```rust
struct Train {
    name: String,
    number: u32,
    speed_kmh: f64,
    is_express: bool,
}
```

This defines a **type** called `Train` with four **fields**. No values yet — just the shape.

Naming convention:
- Struct names → `PascalCase` (like `Train`, `TicketBooking`, `StationInfo`)
- Field names → `snake_case` (like `train_name`, `is_express`)

---

## 2. Creating an Instance

```rust
let rajdhani = Train {
    name: String::from("Rajdhani Express"),
    number: 12001,
    speed_kmh: 130.0,
    is_express: true,
};
```

**Every field must be set.** Rust doesn't have null or default values (unlike JS where
missing fields are `undefined`). You must explicitly provide every field. This prevents
an entire class of bugs.

---

## 3. Accessing Fields

Use dot notation, just like every other language:

```rust
println!("Train: {}", rajdhani.name);
println!("Speed: {} km/h", rajdhani.speed_kmh);
```

---

## 4. Mutability

To modify fields, the **entire variable** must be `mut`. You can't make individual
fields mutable — it's all or nothing:

```rust
let mut rajdhani = Train {
    name: String::from("Rajdhani Express"),
    number: 12001,
    speed_kmh: 0.0,
    is_express: true,
};

rajdhani.speed_kmh = 130.0;  // ✅ whole struct is mut
rajdhani.is_express = false;  // ✅ can change any field
```

This is different from TypeScript where you can `readonly` individual properties.
Rust's approach is simpler: the variable is either mutable or it's not.

---

## 5. Field Init Shorthand

When variable names match field names, you can skip the repetition:

```rust
let name = String::from("Shatabdi Express");
let number = 12002;
let speed_kmh = 150.0;
let is_express = true;

// Without shorthand:
let train = Train {
    name: name,
    number: number,
    speed_kmh: speed_kmh,
    is_express: is_express,
};

// With shorthand (identical result):
let train = Train {
    name,       // same as name: name
    number,     // same as number: number
    speed_kmh,
    is_express,
};
```

This is exactly like ES6 object shorthand: `{ name, number }` instead of
`{ name: name, number: number }`.

---

## 6. Struct Update Syntax

Create a new struct based on an existing one, overriding some fields:

```rust
let rajdhani = Train {
    name: String::from("Rajdhani Express"),
    number: 12001,
    speed_kmh: 130.0,
    is_express: true,
};

let rajdhani_slow = Train {
    speed_kmh: 80.0,                // override this field
    name: String::from("Rajdhani Slow"),  // override this too
    ..rajdhani                       // copy the rest from rajdhani
};
```

**⚠️ Ownership warning:** The `..rajdhani` syntax **moves** any String/heap fields that
you don't override. After this, `rajdhani.name` would be invalid (moved). But
`rajdhani.number` and `rajdhani.is_express` are fine because integers and bools are `Copy`.

If you need to keep the original intact, `.clone()` it:

```rust
let rajdhani_slow = Train {
    speed_kmh: 80.0,
    ..rajdhani.clone()  // deep copy, rajdhani stays valid
};
```

(This requires `#[derive(Clone)]` on the struct — we'll see that shortly.)

---

## 7. Printing Structs with `#[derive(Debug)]`

By default, you can't print a struct with `println!`:

```rust
println!("{}", rajdhani);  // ❌ ERROR: Train doesn't implement Display
```

The quick fix: add `#[derive(Debug)]` above the struct definition:

```rust
#[derive(Debug)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: f64,
    is_express: bool,
}
```

Now you can use `{:?}` (debug format) or `{:#?}` (pretty debug format):

```rust
println!("{:?}", rajdhani);
// Output: Train { name: "Rajdhani Express", number: 12001, speed_kmh: 130.0, is_express: true }

println!("{:#?}", rajdhani);
// Output:
// Train {
//     name: "Rajdhani Express",
//     number: 12001,
//     speed_kmh: 130.0,
//     is_express: true,
// }
```

`#[derive(...)]` is an **attribute** that asks the compiler to auto-generate code.
Common derives you'll use:

| Derive | What it gives you |
|--------|-------------------|
| `Debug` | Print with `{:?}` |
| `Clone` | `.clone()` method for deep copies |
| `PartialEq` | Compare with `==` and `!=` |
| `Default` | Create with default values |

You can stack them: `#[derive(Debug, Clone, PartialEq)]`

---

## 8. Methods with `impl`

Methods are functions attached to a struct. Define them in an `impl` block:

```rust
impl Train {
    fn display_info(&self) {
        println!("{} (#{}) — {} km/h", self.name, self.number, self.speed_kmh);
    }
}
```

Then call them with dot notation:

```rust
rajdhani.display_info();
// Output: Rajdhani Express (#12001) — 130 km/h
```

### The `self` parameter

The first parameter of a method is always some form of `self` — it's the instance
the method is called on:

| Parameter | Meaning | When to use |
|-----------|---------|-------------|
| `&self` | Immutable borrow of the instance | Reading data (most common) |
| `&mut self` | Mutable borrow of the instance | Modifying data |
| `self` | Takes ownership of the instance | Consuming/transforming (rare) |

```rust
impl Train {
    // Read-only — just inspects the struct
    fn is_fast(&self) -> bool {
        self.speed_kmh > 100.0
    }

    // Mutating — changes the struct
    fn accelerate(&mut self, amount: f64) {
        self.speed_kmh += amount;
    }

    // Consuming — takes ownership, returns something new
    fn into_name(self) -> String {
        self.name  // self is consumed — the Train is gone after this
    }
}
```

**Think of it like this:**
- `&self` → "let me look at this train"
- `&mut self` → "let me modify this train"
- `self` → "give me this train (you can't use it anymore)"

This maps directly to the ownership concepts from Lesson 03!

---

## 9. Associated Functions (Constructors)

Functions in an `impl` block that **don't** take `self` are called **associated functions**.
They're like static methods in other languages. The convention is to use `new` as a
constructor:

```rust
impl Train {
    fn new(name: &str, number: u32) -> Train {
        Train {
            name: String::from(name),
            number,
            speed_kmh: 0.0,
            is_express: false,
        }
    }
}
```

Call them with `::` (not dot notation):

```rust
let train = Train::new("Rajdhani Express", 12001);
```

Notice: `Train::new()` uses `::` because there's no instance yet — you're calling it
on the **type itself**.

You can have multiple constructors:

```rust
impl Train {
    fn new(name: &str, number: u32) -> Train {
        Train {
            name: String::from(name),
            number,
            speed_kmh: 0.0,
            is_express: false,
        }
    }

    fn new_express(name: &str, number: u32, speed: f64) -> Train {
        Train {
            name: String::from(name),
            number,
            speed_kmh: speed,
            is_express: true,
        }
    }
}
```

You've already seen this pattern: `String::from("hello")` is an associated function
on the `String` type!

---

## 10. Multiple `impl` Blocks

You can split methods across multiple `impl` blocks. They're merged by the compiler:

```rust
impl Train {
    fn new(name: &str, number: u32) -> Train { /* ... */ }
}

impl Train {
    fn display_info(&self) { /* ... */ }
    fn accelerate(&mut self, amount: f64) { /* ... */ }
}
```

This is just organizational — sometimes useful when you have many methods and want
to group them logically.

---

## 11. Tuple Structs

A lighter struct where fields have no names — just positions:

```rust
struct Kilometers(f64);
struct TrainId(u32);
struct Coordinate(f64, f64);

let distance = Kilometers(350.0);
let id = TrainId(12001);
let location = Coordinate(13.0827, 80.2707);

println!("Distance: {} km", distance.0);       // access by index
println!("Location: {}, {}", location.0, location.1);
```

These are useful for **type safety**. Without them:

```rust
fn book_ticket(train: u32, distance: f64, price: f64) { }
book_ticket(350.0, 12001, 755.0);  // ❌ oops — swapped train and distance!
                                    // but compiler won't catch it, both are numbers
```

With tuple structs:

```rust
fn book_ticket(train: TrainId, distance: Kilometers, price: Rupees) { }
book_ticket(Kilometers(350.0), TrainId(12001), Rupees(755.0));
// ❌ compiler error! Types are in the wrong order. Bug caught!
```

---

## 12. Methods Returning `Self`

Use `Self` (capital S) as a shorthand for the struct's own type. This is especially
handy in constructors:

```rust
impl Train {
    fn new(name: &str, number: u32) -> Self {  // Self = Train
        Self {
            name: String::from(name),
            number,
            speed_kmh: 0.0,
            is_express: false,
        }
    }
}
```

`Self` always refers to the type the `impl` block is for. If you rename the struct,
`Self` updates automatically.

---

## 13. Structs and Ownership — Putting It All Together

Structs **own** their fields. When a struct is dropped, all its fields are dropped:

```rust
{
    let train = Train::new("Rajdhani", 12001);
    // train owns the String "Rajdhani", the u32, the f64, the bool
}
// train goes out of scope — the String is freed, everything is cleaned up
```

When you pass a struct to a function, the same ownership rules apply:

```rust
fn consume(train: Train) {       // takes ownership
    println!("{}", train.name);
}

fn inspect(train: &Train) {      // borrows
    println!("{}", train.name);
}

fn modify(train: &mut Train) {   // borrows mutably
    train.speed_kmh += 10.0;
}
```

---

## Comparison with TypeScript

| TypeScript | Rust |
|------------|------|
| `class Train { ... }` | `struct Train { ... }` |
| `constructor()` | `fn new() -> Self` in `impl` block |
| `this.name` | `self.name` |
| `train.display()` | `train.display()` — same! |
| `Train.create()` (static) | `Train::new()` (associated fn) |
| `readonly name: string` | No single-field mutability — whole struct is `mut` or not |
| Inheritance (`extends`) | Doesn't exist — use traits (Lesson 08) and composition |

---

## Key Takeaways

1. `struct` defines the shape, `impl` adds behavior
2. `&self` to read, `&mut self` to modify, `self` to consume
3. `Train::new()` is the convention for constructors (associated functions)
4. `#[derive(Debug)]` lets you print structs with `{:?}`
5. Structs own their data — when they die, their fields die
6. No null fields — every field must be set at creation
7. No inheritance — we'll use traits for shared behavior in Lesson 08

---

## Up Next

Run the examples, then build a **Railway Station Management System** in the exercise! 🚂
