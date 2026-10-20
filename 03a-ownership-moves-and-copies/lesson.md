# Lesson 03a — Ownership: Moves and Copies

> ⏱️ Estimated reading time: 8 minutes

Lesson 03 introduced ownership. This sub-lesson drills the first half: **who owns what
after each line**, which types move, which copy, and how values travel in and out of
functions, Vecs and structs.

## Recap

| Type | Behaviour on `let b = a;` / passing to fn |
|---|---|
| `i32`, `u8`, `f64`, `bool`, `char`, `&T` | **Copy** — `a` still usable |
| `[T; N]`, tuples, structs of only Copy data (with `#[derive(Clone, Copy)]`) | Copy |
| `String`, `Vec<T>`, `Box<T>`, structs holding them | **Move** — `a` is dead |

```rust
let a = String::from("x");
let b = a;            // move: a is dead
let c = b.clone();    // explicit deep copy: b and c both live
```

| TypeScript | Rust |
|---|---|
| objects are shared references | a move transfers the single owner |
| `structuredClone(x)` | `x.clone()` |
| GC frees eventually | `Drop` runs at end of scope, deterministically |
| `const f = (s) => s + "!"` | `fn f(mut s: String) -> String { s.push('!'); s }` |

## Move in, move out

A function taking `String` consumes it. To "modify" it, return it and re-bind:

```rust
let name = add_bang(name);   // shadowing: new owner, same name
```

## Moves from containers and structs

- `v.remove(i)`, `v.pop()` and `for x in v` move elements **out** of a Vec.
- `v[i]` cannot move out (it would leave a hole) — clone or borrow instead.
- Moving one field out of a struct is a **partial move**: the other fields stay usable,
  but the whole struct cannot be used again (unless it has no `Drop` impl and you only
  touch the remaining fields).
- `Struct { field, ..old }` moves the non-Copy fields you did not override.

## Drop order

Locals drop in **reverse** declaration order. A value moved into a function drops at the
end of *that* function. `drop(x)` is just `fn drop<T>(_: T) {}` — a move into a function
that ends immediately.

## Pitfalls

- Thinking `let b = a` copies a `String` (it moves).
- Forgetting `mut` on a by-value parameter you intend to modify: `fn f(mut s: String)`.
- Putting `Copy` on a struct that holds a `String` (not allowed).
- Cloning "to make the error go away" without asking whether a borrow would do.
- Using a closure that captures by reference when you need it to own: add `move`.

## Compile-error gallery

### 1. Use after move
```rust
let a = String::from("x");
let b = a;
println!("{a}");
```
```
error[E0382]: borrow of moved value: `a`
```
Fix: `let b = a.clone();` or just use `b`.
### 2. Moved into a function
```rust
fn eat(s: String) {}
let s = String::from("x");
eat(s);
eat(s);
```
```
error[E0382]: use of moved value: `s`
```
Fix: pass `s.clone()` the first time, or change `eat` to take `&str`.

### 3. Moving out of a Vec by index
```rust
let v = vec![String::from("a")];
let s = v[0];
```
```
error[E0507]: cannot move out of index of `Vec<String>`
```
Fix: `&v[0]`, `v[0].clone()` or `v.remove(0)`.

### 4. Copy on a non-Copy field
```rust
#[derive(Clone, Copy)]
struct Train { name: String }
```
```
error[E0204]: the trait `Copy` cannot be implemented for this type
```
Fix: drop `Copy` (keep `Clone`), or use `&'static str`.

### 5. Using a struct after a partial move
```rust
struct T { a: String, b: String }
let t = T { a: "1".into(), b: "2".into() };
let x = t.a;
let y = t;
```
```
error[E0382]: use of partially moved value: `t`
```
Fix: move each field separately, or clone `t.a`.

## Your task

Open `exercise/src/main.rs`: 13 steps, write everything including `main()`, match the
output exactly. Reference: `exercise/solution.rs`.
