# Lesson 08a — Range Type Drills

> ⏱️ Estimated reading time: 10 minutes

Lesson 08 introduced ranges as real values. This sub-lesson is the gym: twelve drills on every
member of the range family, plus the boundary mistakes that bite in practice. In TypeScript you
have nothing comparable (`Array.from({length})`, `slice(a, b)`), so the muscle memory is new.

---

## 1. The six range types

| Literal | Type | Iterable? | Fields / accessors |
|---|---|---|---|
| `a..b` | `Range<T>` | yes (if `T: Step`) | `.start`, `.end` |
| `a..=b` | `RangeInclusive<T>` | yes | `.start()`, `.end()` |
| `a..` | `RangeFrom<T>` | yes, endless | `.start` |
| `..b` | `RangeTo<T>` | no | `.end` |
| `..=b` | `RangeToInclusive<T>` | no | `.end` |
| `..` | `RangeFull` | no | none |

All of them have `.contains(&x)`. Only `Range` and `RangeInclusive` have `.is_empty()`.

## 2. Empty is not an error

```rust
let r = 5..1;
assert!(r.is_empty());          // start >= end
assert_eq!(r.count(), 0);       // iterates zero times; no panic
```

`3..3` is empty, `3..=3` holds exactly one item. A reversed range never counts down: use
`(1..5).rev()`, not `5..1`.

## 3. Adapters

- `.rev()` needs a double-ended iterator: fine for `Range` and `RangeInclusive`.
- `.step_by(k)` panics when `k == 0`.
- **Order matters:** `(0..11).step_by(3).rev()` is `9,6,3,0`, but `(0..11).rev().step_by(3)` is
  `10,7,4,1`.
- `Range<i32>::len()` exists (ExactSizeIterator); for `RangeInclusive<u32>` use `.count()`.

## 4. Slicing

`&v[a..b]` panics when `b > len` or `a > b`. `v.get(a..b)` returns `Option<&[T]>` instead.
`&v[..]`, `&v[a..]`, `&v[..=b]` all work, because slice indexing accepts the whole family.

## 5. Chars and floats

`'a'..='z'` is iterable (chars have a successor). `0.0..1.0` is a valid `Range<f64>`, and
`.contains` works, but you cannot `for` over it: floats have no "next value".

## 6. Boundary arithmetic

Ranges live next to unsigned integers, and `0 - 1` on `usize` panics in debug builds.
Reach for `saturating_sub`, `checked_add`, `.min()` and `.max()` where a boundary can be crossed.
`(250u8..=255)` is safe, because `RangeInclusive` tracks exhaustion itself.

## 7. Pitfalls

- Moving a `Range` into a loop or `.sum()` consumes it; `.clone()` first if you need it again.
- `Range` is not `Copy`, so passing it by value twice needs `.clone()`.
- In `match` patterns, write `a..=b`. Exclusive `a..b` patterns are accepted by newer compilers but `..=` is the portable spelling.
- An overnight shift `22..6` is empty, not "22 through 6".

---

## Compile-error gallery

**1. Iterating a RangeTo**
```rust
for i in ..5 { }
```
```
error[E0277]: `RangeTo<{integer}>` is not an iterator
```
Fix: give it a start, `0..5`.

**2. Iterating floats**
```rust
for x in 0.0..1.0 { }
```
```
error[E0277]: `std::ops::Range<{float}>` is not an iterator
```
Fix: iterate integers and convert, `(0..10).map(|i| i as f64 / 10.0)`.

**3. Reading a private field**
```rust
let r = 1..=5;
println!("{}", r.start);
```
```
error[E0616]: field `start` of struct `RangeInclusive` is private
```
Fix: `r.start()`.

**4. Using a range after it moved**
```rust
let r = 0..3;
let total: i32 = r.sum();
println!("{:?}", r);
```
```
error[E0382]: borrow of moved value: `r`
```
Fix: `r.clone().sum()`.

**5. Mixing integer types**
```rust
let r: std::ops::Range<usize> = 0..3u8;
```
```
error[E0308]: mismatched types: expected `usize`, found `u8`
```
Fix: `0..3usize`, or `0..3` and let inference pick.

---

## Up Next

08b takes ranges into `match` patterns and the `RangeBounds` trait for generic parameters.
