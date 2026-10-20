# Lesson 07a — Vec Fundamentals Drills

> ⏱️ Estimated reading time: 8 minutes

Lesson 07 introduced `Vec<T>`. This sub-lesson is pure drilling: creating, indexing safely,
the three iteration modes, 2D grids, and your first borrow-checker traps. Everything is on
`Vec<String>` / `Vec<u32>` — no structs yet.

## Recap

```rust
let mut v: Vec<String> = Vec::new();   // empty
let mut w = Vec::with_capacity(10);    // len 0, capacity >= 10
v.push("a".to_string());               // end, O(1) amortized
v.insert(0, "b".to_string());          // anywhere, O(n)
let last: Option<String> = v.pop();    // None when empty
let gone: String = v.remove(0);        // PANICS if out of bounds
```

| TypeScript | Rust |
|---|---|
| `arr[i]` (undefined if OOB) | `v[i]` panics; `v.get(i)` gives `Option<&T>` |
| `arr.push(x)` | `v.push(x)` (needs `mut`) |
| `arr.pop()` -> `T \| undefined` | `v.pop()` -> `Option<T>` |
| `arr.indexOf(x)` | `v.iter().position(\|e\| e == x)` -> `Option<usize>` |
| `arr.length` | `v.len()` |
| `for (const x of arr)` | `for x in &v` / `&mut v` / `v` |

## The three iteration modes

| Syntax | Item type | Vec afterwards |
|---|---|---|
| `for x in &v` / `v.iter()` | `&T` | still yours |
| `for x in &mut v` / `v.iter_mut()` | `&mut T` | still yours, modified |
| `for x in v` / `v.into_iter()` | `T` | **moved**, gone |

Use `*x` to read/write through a reference: `*x += 1`.

## Take slices, not `&Vec`

`fn f(items: &[String])` accepts `&Vec<String>`, `&[String; 3]` and `&v[1..]`.
Only take `&mut Vec<T>` when you must change the length (push/pop/insert/remove).
Mutating elements only? `&mut [T]` is enough.

## Grids: `Vec<Vec<T>>`

`vec![vec![false; cols]; rows]` clones the inner Vec per row (so it needs `T: Clone`).
Index `grid[r][c]`, or stay panic-free with `grid.get_mut(r).and_then(|row| row.get_mut(c))`.

## Pitfalls

1. **Indexing panics.** `v[99]` aborts the thread. Use `.get()` when the index is untrusted.
2. **`remove(i)` panics** out of bounds; check `i < v.len()` first.
3. **`for x in v` moves the Vec.** Iterate `&v` unless you mean it.
4. **`capacity` is not `len`.** Never assert exact capacity; only `>=`.
5. **`vec![v; n]`** gives n independent clones, not n aliases (unlike JS `Array(n).fill([])`!).
6. **`&String` vs `&str`:** compare with `s == name` where both are references to the same kind, or `s.as_str() == name`.

## Compile-error gallery

### 1. Push while holding a reference

```rust
let mut v = vec![String::from("a")];
let first = &v[0];
v.push(String::from("b"));
println!("{first}");
```
```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
```
Why: push may reallocate, leaving `first` dangling. Fix: clone first (`let first = v[0].clone();`)
or finish using `first` before pushing.

### 2. Use after move into a loop

```rust
let v = vec![1, 2, 3];
for x in v { println!("{x}"); }
println!("{}", v.len());
```
```
error[E0382]: borrow of moved value: `v`
```
Fix: `for x in &v`.

### 3. Forgetting `mut`

```rust
let v = Vec::new();
v.push(1);
```
```
error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
```
Fix: `let mut v`.

### 4. Moving out of an index

```rust
let v = vec![String::from("a")];
let s: String = v[0];
```
```
error[E0507]: cannot move out of index of `Vec<String>`
```
Fix: `v[0].clone()`, `&v[0]`, or `v.remove(0)` / `v.pop()` to take ownership.

### 5. Mutating through `&` iteration

```rust
let mut v = vec![1, 2];
for x in &v { *x += 1; }
```
```
error[E0594]: cannot assign to `*x`, which is behind a `&` reference
```
Fix: `for x in &mut v`.

## Your task

Open `exercise/src/main.rs`: 13 steps, write everything including `main()`, match the output exactly.
Reference: `exercise/solution.rs`.
