# Lesson 12 — Lifetimes

> Estimated reading time: 10 minutes

Every reference in Rust has a *lifetime* — a span of time during which the reference is
valid. Most of the time the compiler works this out for you and you never think about it.
When it cannot, you annotate lifetimes yourself. That is all a lifetime annotation does:
it communicates information the compiler needs to verify that no reference outlives the
data it points to.

Think of it this way: a railway station's departure board borrows information from the
live train database. If the database goes offline the board must stop showing that data.
Lifetimes ensure the board can never outlive the database it borrows from.

---

## 1. Why Lifetimes Exist

Rust's borrow checker needs to confirm that references are always valid. Without that
guarantee, you get *dangling references* — pointers to memory that has already been freed.

```rust
// This does not compile — the borrow checker rejects it.
fn dangling_station() -> &String {
    let name = String::from("Chennai Central"); // name is created here
    &name                                       // ERROR: name is dropped at the end
}                                               //        of this function
```

The function tries to return a reference to `name`, but `name` is destroyed when the
function returns. The caller would receive a reference to freed memory. Rust prevents
this entirely at compile time — no runtime crash, no undefined behaviour.

Lifetimes are the mechanism the borrow checker uses to track "how long does this
reference remain valid?" across function boundaries.

---

## 2. Lifetime Annotations

Lifetime annotations use a tick followed by a name: `'shared`, `'input`, `'a`, and so on.
They appear in generic position, right alongside type parameters.

Before seeing when you *need* them, it helps to see when you *don't*:

```rust
// One input reference, one output reference.
// The compiler can reason: the output must borrow from the input — what else could it be?
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

No annotation needed. There is only one input reference, so the output must come from it.

Now consider a function that picks the longer of two station names:

```rust
fn longest(a: &str, b: &str) -> &str {   // ERROR — will not compile
    if a.len() >= b.len() { a } else { b }
}
```

This fails. The compiler sees two input references and one output reference, but the
output could come from *either* input depending on runtime values. It cannot figure out
the relationship on its own, so it rejects the code.

You fix it by introducing a lifetime name and applying it to all three references:

```rust
fn longest<'shared>(a: &'shared str, b: &'shared str) -> &'shared str {
    if a.len() >= b.len() { a } else { b }
}
```

Read `'shared` as "some lifetime — I'll call it `shared`". The annotation says:
- Both `a` and `b` must be valid for at least `'shared`.
- The returned reference is valid for at most `'shared`.

The compiler does not need to know the exact duration. It just needs a *label* so it can
reason: "both inputs share `'shared`, and the return is tied to `'shared`, so the return
cannot outlive either input."

### The `'a` convention

In real Rust code you will almost always see `'a` rather than `'shared`. They mean
exactly the same thing — `'a` is just a convention, the same way `T` is the conventional
name for a generic type parameter. Once you understand that a lifetime annotation is
simply a label, `'a` is less noise to read. This lesson uses descriptive names in new
examples to make the concept clear, then switches to `'a` in the elision section where
you will encounter it most often in practice.

### What lifetime annotations are not

- They are not a duration in nanoseconds.
- They do not change what code runs or when memory is freed.
- They are purely labels the compiler uses to verify references don't outlive their data.

---

## 3. Lifetime Elision

In the early days of Rust, **every** function that took or returned a reference required
explicit lifetime annotations. Even trivial methods looked like this:

```rust
// Old Rust — you had to write this
fn display_type<'a>(&'a self) -> &'a str {
    "STATION"
}

fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}
```

This was extremely noisy. The Rust team noticed that the same patterns came up over and
over again, and in those patterns the compiler could work out the annotation itself.
They baked those patterns into the compiler as **elision rules** — places where you are
allowed to omit the annotation because the answer is unambiguous.

Today you write:

```rust
fn display_type(&self) -> &str { "STATION" }
fn first_word(s: &str) -> &str { ... }
```

The compiler silently expands these to the verbose form. Nothing changes about what the
code does — elision is purely a shorthand.

### The three elision rules

The compiler applies these rules in order. If they produce an unambiguous answer, no
annotation is needed. If not, the compiler asks you to write one.

**Rule 1** — each input reference gets its own independent lifetime.

```rust
fn foo(a: &str, b: &str)
// compiler expands to:
fn foo<'a, 'b>(a: &'a str, b: &'b str)
```

**Rule 2** — if there is exactly *one* input lifetime after rule 1, use it for all
output references.

```rust
fn first_word(s: &str) -> &str
// after rule 1: fn first_word<'a>(s: &'a str) -> &str
// after rule 2: fn first_word<'a>(s: &'a str) -> &'a str  ✅ unambiguous
```

This is why `first_word` compiles without annotation — one input, one output, rule 2
connects them automatically.

**Rule 3** — if one of the inputs is `&self` or `&mut self`, the lifetime of `self` is
used for all output references.

```rust
fn display_type(&self) -> &str
// after rule 1: fn display_type<'a>(&'a self) -> &str
// after rule 3: fn display_type<'a>(&'a self) -> &'a str  ✅ unambiguous
```

Rule 3 is why you never need to annotate methods that return a reference. When a method
returns a reference, it almost always borrows from `self` — so tying the output to
`self`'s lifetime is nearly always correct.

### When elision is not enough

Elision fails when none of the three rules produce an unambiguous answer. This is exactly
what happened with `longest`:

```rust
fn longest(a: &str, b: &str) -> &str
// after rule 1: fn longest<'a, 'b>(a: &'a str, b: &'b str) -> &str
// rule 2: more than one input lifetime — doesn't apply
// rule 3: no &self — doesn't apply
// result: output lifetime still unknown → compiler asks you to annotate
```

Two input lifetimes, one output: the compiler cannot guess which input the output borrows
from. You have to tell it explicitly with `<'a>`.

### The `"STATION"` case

```rust
fn display_type(&self) -> &str {
    "STATION"   // a string literal — lives for the entire program
}
```

After elision, the compiler sees `-> &'a str` where `'a` is tied to `self`. The literal
`"STATION"` has type `&'static str`, which lives longer than any `'a`. A reference that
lives *longer* than required always satisfies a shorter lifetime, so this compiles fine.
Elision handles the annotation; the `'static` nature of the literal handles the value.

---

## 4. Lifetimes on Structs

When a struct holds a reference (rather than an owned value), it needs a lifetime
parameter. The rule is simple: the struct cannot outlive the data it borrows.

```rust
struct Announcement<'msg> {
    message: &'msg str,
    platform: u8,
}

impl<'msg> Announcement<'msg> {
    fn display(&self) {
        println!("  Platform {}: {}", self.platform, self.message);
    }
}
```

The `'msg` on `Announcement<'msg>` tells the compiler: "an `Announcement` is only valid
as long as the `message` it borrows is valid." In practice you will see `'a` here, but
`'msg` makes the intent obvious on first read. If you try to use an `Announcement` after
`message` has been dropped, the compiler rejects it.

```rust
let ann;
{
    let msg = String::from("Rajdhani Express departing in 5 minutes");
    ann = Announcement { message: &msg, platform: 3 };
    ann.display(); // fine — msg is alive here
}
// ann.display(); // would not compile — msg is dropped above
```

Compare this to a struct that owns its data: no lifetime annotation needed because the
struct carries the data with it.

```rust
struct OwnedAnnouncement {
    message: String, // owned — no lifetime needed
    platform: u8,
}
```

---

## 5. `'static`

`'static` is the one named lifetime you will see often. It means "valid for the entire
duration of the program". String literals have type `&'static str`:

```rust
let greeting: &'static str = "Welcome aboard"; // baked into the binary
```

When you see `'static` as a *bound* — for example `T: 'static` — it means "this type
contains no borrowed references that could expire". It does not mean the value actually
lives forever; it means it *could*.

Common appearances:

- String literals: `"text"` is always `&'static str`.
- Error types in `Box<dyn Error + Send + Sync + 'static>` — a frequent pattern in
  async code.
- Thread spawn: `std::thread::spawn` requires `'static` because threads can outlive the
  scope they were created in.

---

## 6. TypeScript Comparison

TypeScript (and JavaScript) use a garbage collector. References can point to anything;
the runtime tracks when objects are no longer reachable and frees them. There are no
dangling references because the GC ensures referenced objects are never freed while a
reference to them exists.

Rust achieves the same safety guarantee without a GC — purely through compile-time
analysis. The cost is that you occasionally need to annotate lifetimes to give the
compiler enough information to verify correctness.

| | TypeScript | Rust |
|---|---|---|
| Memory safety | Garbage collector at runtime | Borrow checker at compile time |
| Dangling references | Impossible (GC prevents it) | Impossible (borrow checker prevents it) |
| Lifetime annotations | None needed | Occasionally needed; usually elided |
| Runtime cost | GC pauses, heap scanning | Zero — lifetimes are erased at compile time |
| Struct holding a reference | No special syntax | Requires lifetime parameter |
| `'static` equivalent | All string literals | String literals + any GC-managed value |

The key insight: TypeScript pays for memory safety at runtime every time the GC runs.
Rust pays for it once, at compile time, and then the binary runs with no overhead.

---

## 7. Mental Model Summary

```
┌──────────────────────────────────────────────────────────┐
│  'a = "some lifetime" — a label, not a duration          │
│  fn f<'a>(x: &'a T) -> &'a U  = return borrows from x   │
│  struct S<'a> { field: &'a T } = S cannot outlive T      │
│  'static = valid for the whole program                    │
└──────────────────────────────────────────────────────────┘
```

- **Lifetimes prevent dangling references** — the compiler checks at compile time, not at runtime.
- **You need `'a` when** there are multiple input references and the compiler can't tell which one the output borrows from.
- **Elision = the compiler writes the annotation for you** — it applies three rules; if the answer is unambiguous, no annotation needed.
- **Rule 3 (`&self`)** — method return references are almost always tied to `self`, so the compiler assumes that and you never have to write it.
- **Struct lifetimes** — any struct holding a reference needs `<'a>`; the struct cannot outlive the data it borrows.
- **`'static`** — string literals and any value with no borrowed data; satisfies any shorter lifetime requirement.
- **No runtime cost** — lifetimes are compile-time only; they are completely erased from the binary.

Lesson 13 builds on this with generics and how lifetime annotations interact with generic
type parameters.
