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

## 2. The `'a` Syntax

Lifetime annotations use a tick followed by a name: `'a`, `'b`, `'input`, and so on.
They appear *in generic position*, right alongside type parameters.

The canonical example is a function that returns the longer of two string slices:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Read `'a` as "some lifetime that both inputs share". The annotation says:

- Both `a` and `b` must be valid for at least `'a`.
- The returned reference is valid for at most `'a`.

The compiler does not need to know the exact duration — it just needs to know that the
return value cannot outlive either input.

**Without the annotation this function cannot compile.** The compiler sees that the
return borrows from one of two inputs, but it cannot determine which one, so it cannot
verify the caller's usage is safe. The annotation gives it the information it needs.

### What `'a` is not

- It is not a measure of time in nanoseconds.
- It does not change what code runs or when memory is freed.
- It is purely a label that lets the compiler express "the return is tied to the inputs".

---

## 3. Lifetime Elision

Most functions that take and return references do *not* need explicit annotations because
the compiler applies three *elision rules* automatically:

1. **Each reference parameter gets its own lifetime.**
   `fn foo(a: &str, b: &str)` becomes `fn foo<'a, 'b>(a: &'a str, b: &'b str)`.

2. **If there is exactly one input reference lifetime, it is used for all output
   references.**
   `fn first_word(s: &str) -> &str` becomes `fn first_word<'a>(s: &'a str) -> &'a str`.

3. **If one of the parameters is `&self` or `&mut self`, the lifetime of `self` is used
   for all output references.**

Rule 3 is why you can write `display_type` on a trait without any annotation:

```rust
// Why does this work without a lifetime annotation?
fn display_type(&self) -> &str {
    "STATION"
}
// Elision rule 3: if there's a &self parameter, the return lifetime
// is tied to self. The compiler fills in: fn display_type<'a>(&'a self) -> &'a str
// The string literal "STATION" is 'static so it satisfies any lifetime.
```

The string literal `"STATION"` lives for the entire program (`'static`), so it is valid
for any lifetime the caller might infer — including the lifetime of `self`. Elision
handles the annotation; you never have to write it.

---

## 4. Lifetimes on Structs

When a struct holds a reference (rather than an owned value), it needs a lifetime
parameter. The rule is simple: the struct cannot outlive the data it borrows.

```rust
struct Announcement<'a> {
    message: &'a str,
    platform: u8,
}

impl<'a> Announcement<'a> {
    fn display(&self) {
        println!("  Platform {}: {}", self.platform, self.message);
    }
}
```

The `'a` on `Announcement<'a>` tells the compiler: "an `Announcement` is only valid as
long as the `message` it borrows is valid." If you try to use an `Announcement` after
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

- **Lifetimes prevent dangling references** — the compiler checks at compile time.
- **Elision covers the common cases** — you rarely write `'a` on methods with `&self`.
- **Struct lifetimes** — any struct holding a reference needs a lifetime parameter.
- **`'static`** — string literals and "no borrowed data" bounds.
- **No runtime cost** — lifetimes are compile-time only; they do not affect the binary.

Lesson 13 builds on this with generics and how lifetime annotations interact with generic
type parameters.
