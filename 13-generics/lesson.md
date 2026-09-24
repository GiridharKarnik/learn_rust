# Lesson 12 — Generics

> ⏱️ Estimated reading time: 12 minutes

Lesson 11 introduced traits — shared contracts between types. Generics take that idea
further: write one function or struct that works with *any* type, optionally constrained
by traits. This is Rust's primary tool for reuse without runtime cost.

Think of it this way: the display board at a railway station shows trains, stations, and
routes using the same formatting logic — one piece of code, many types. That's generics.

---

## 1. Generic Functions

You've already used `impl Trait` in function parameters:

```rust
fn print_item(item: &impl Displayable) {
    println!("{}", item.display_full());
}
```

The full form uses a **type parameter** `<T>`:

```rust
fn print_item<T: Displayable>(item: &T) {
    println!("{}", item.display_full());
}
```

Both are equivalent. The `<T>` form becomes necessary when you need to refer to the same
type in multiple places — for example, when the return type must match the input:

```rust
// impl Trait can't do this cleanly — T appears twice
fn first_of_two<T: Displayable>(a: &T, b: &T) -> &T {
    if a.display_line().len() >= b.display_line().len() { a } else { b }
}
```

Use `impl Trait` for simple one-off parameters. Use `<T>` when `T` appears more than once
or when you need the type name elsewhere in the function.

---

## 2. Generic Structs

Structs can also be generic over a type parameter:

```rust
struct Pair<T> {
    first: T,
    second: T,
}
```

You implement methods on generic structs with `impl<T>`:

```rust
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }
}
```

You can add trait bounds inside the `impl` when the method needs them:

```rust
impl<T: Displayable> Pair<T> {
    fn display_both(&self) {
        println!("{}", self.first.display_full());
        println!("{}", self.second.display_full());
    }
}
```

This means: "for any `T` that implements `Displayable`, `Pair<T>` has a `display_both`
method." Types that don't implement `Displayable` still get `Pair<T>::new` — they just
don't get `display_both`.

---

## 3. Trait Bounds

A **trait bound** constrains which types a generic function accepts:

```rust
fn print_all<T: Displayable>(items: &[T]) {
    for item in items {
        println!("  {}", item.display_full());
    }
}
```

`T: Displayable` reads as: *"T must implement Displayable."* The compiler enforces this
— if you pass a type that doesn't implement `Displayable`, it's a compile error.

This is the connection back to `impl Trait`: `fn f(x: &impl Displayable)` is exactly
`fn f<T: Displayable>(x: &T)` — same constraint, different syntax.

---

## 4. Multiple Bounds

Use `+` to require a type to implement more than one trait:

```rust
fn compare_and_print<T: Displayable + PartialEq>(a: &T, b: &T) {
    println!("{}", a.display_full());
    if a == b {
        println!("  (same as above)");
    }
}
```

This accepts only types that implement both `Displayable` and `PartialEq`. Passing a
type that satisfies only one of them is a compile error.

---

## 5. Where Clauses

When trait bounds grow long, a **where clause** moves them out of the angle brackets for
readability:

```rust
// Inline — gets cramped with multiple bounds
fn find_and_display<T: Displayable + PartialEq + Clone>(items: &[T], target: &T) { ... }

// Where clause — same thing, easier to read
fn find_and_display<T>(items: &[T], target: &T)
where
    T: Displayable + PartialEq + Clone,
{
    ...
}
```

Both forms compile to exactly the same code. Prefer `where` when you have more than one
or two bounds, or when the signature line would exceed 80 characters.

---

## 6. Monomorphization — Zero-Cost Generics

When you call a generic function with a concrete type, the compiler generates a
*specialized* version of that function for that type. This process is called
**monomorphization**.

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in &list[1..] {
        if item > biggest { biggest = item; }
    }
    biggest
}

let speeds: Vec<u32> = vec![130, 150, 160];
let platforms: Vec<u8> = vec![17, 16, 18];

largest(&speeds);     // compiler generates largest::<u32>
largest(&platforms);  // compiler generates largest::<u8>
```

The compiler generates two separate functions — one for `u32`, one for `u8` — each
as efficient as if you had written them by hand. There is no runtime overhead for
the generics machinery: **zero-cost abstraction**.

### TypeScript comparison

```typescript
// TypeScript generics are erased at runtime — there is only one JS function
function largest<T>(list: T[]): T { ... }
```

In TypeScript, the generic type parameter exists only during type-checking; the emitted
JavaScript has no trace of `T`. In Rust, generics produce real specialized machine code
for each concrete type used.

---

## 7. Comparison with TypeScript

| TypeScript | Rust | Notes |
|-----------|------|-------|
| `<T>` generics | `<T>` generics | Same syntax |
| `extends` (interface constraint) | `T: Trait` | Constrain the type parameter |
| Multiple constraints: `T extends A & B` | `T: TraitA + TraitB` | Same idea, `+` instead of `&` |
| Runtime dispatch (vtable) | Compile-time dispatch (monomorphization) | Rust is zero-cost |
| Type erasure at JS emission | Specialized machine code per type | Rust generics are real |
| No `where` keyword | `where` clause | Rust readability feature |

The mental model carries over well. The key difference: TypeScript resolves generics at
the type-checker level and erases them at emit. Rust resolves them at compile time and
generates specialized code — no runtime overhead, no boxing, no vtable.

---

## 8. Mental Model Summary

```
┌───────────────────────────────────────────────────────┐
│  <T> = "fill in the type at compile time"             │
│  T: Trait = "T must implement this trait"             │
│  monomorphization = "one copy of the code per type"   │
└───────────────────────────────────────────────────────┘
```

- **Generic functions** use `<T>` to accept any type (with optional bounds).
- **Generic structs** use `<T>` to store any type; `impl<T>` to add methods.
- **Trait bounds** (`T: Trait`) restrict which types are accepted.
- **Multiple bounds** (`T: TraitA + TraitB`) require satisfying more than one trait.
- **Where clauses** are just readable syntax for complex bounds — no semantic difference.
- **Monomorphization** means generics are zero-cost: the compiler generates specialized
  code for each concrete type you use.

When you see `<T: SomeTrait>`, read it as: *"for any type T, as long as T implements
SomeTrait."* The compiler fills in the concrete type at compile time and generates
machine code as efficient as a hand-written non-generic function.
