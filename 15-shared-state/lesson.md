# Lesson 15 — Shared State: Arc, Mutex, and Friends

In TypeScript, you never worry about two pieces of code writing to the same object at
the same time — JavaScript is single-threaded, so data races simply can't happen. In Rust,
the moment you spawn threads or async tasks, the compiler *forces* you to prove that shared
data is safe. No runtime prayer required.

Think of it like a railway control room: multiple signal operators (threads) need to update
the same departure board (shared state). Without coordination, one operator could overwrite
another's update mid-write. Rust's type system makes you install proper locks on the board
before the code even compiles.

---

## 1. The Problem

Imagine two async tasks both trying to update a train's delay status:

```rust
let mut delay_minutes = 0;

// Task 1: update delay
delay_minutes = 15;  // ← who owns this?

// Task 2: read delay
println!("Delay: {}", delay_minutes);  // ← stale? partially written?
```

Rust won't let you share `delay_minutes` across threads or tasks — ownership rules forbid
it. You need two things:

1. **Shared ownership** — multiple tasks hold a reference to the same data
2. **Interior mutability** — a way to mutate data behind a shared reference, safely

That's where `Arc` and `Mutex` come in.

---

## 2. Arc\<T\> — Shared Ownership Across Threads

`Arc` stands for **A**tomically **R**eference **C**ounted. It wraps a value and tracks
how many owners exist. When the last owner drops, the data is freed.

```rust
use std::sync::Arc;

let train_name = Arc::new(String::from("Rajdhani Express"));

let name1 = Arc::clone(&train_name);  // ref count: 2
let name2 = Arc::clone(&train_name);  // ref count: 3

// All three point to the SAME string on the heap
println!("{}", train_name);  // Rajdhani Express
println!("{}", name1);       // Rajdhani Express
println!("{}", name2);       // Rajdhani Express
```

Key facts:
- `Arc::clone()` is **cheap** — it increments an atomic counter, not a deep copy
- `Arc<T>` implements `Deref<Target = T>`, so you use it like a regular reference
- `Arc` gives you **read-only** shared access. You can't mutate through an `Arc` alone.
- Use `Rc<T>` for single-threaded code, `Arc<T>` when crossing thread boundaries

**TypeScript parallel:** Think of `Arc` like multiple variables pointing to the same
object — `const a = obj; const b = obj;`. The difference is Rust tracks this at compile
time and frees memory automatically when the last reference is gone.

---

## 3. Mutex\<T\> — Mutual Exclusion

`Mutex` wraps data and ensures only **one thread at a time** can access it. You must
`lock()` the mutex to get access:

```rust
use std::sync::Mutex;

let counter = Mutex::new(0u32);

{
    let mut guard = counter.lock().unwrap();  // acquire lock
    *guard += 1;                               // mutate through the guard
    println!("Counter: {}", *guard);           // read through the guard
}   // ← guard dropped here → lock released automatically

// Another thread can now lock it
```

The `lock()` call:
- **Blocks** until the lock is available
- Returns a `MutexGuard<T>` — a smart pointer that auto-releases the lock when dropped
- Returns `Result` because a mutex can be "poisoned" if a thread panicked while holding it
  (`.unwrap()` is fine for most cases)

**The guard pattern** is beautiful: you can't forget to unlock because Rust's drop semantics
handle it. No `try/finally` needed. The lock lives exactly as long as the guard variable.

```rust
// Short-lived lock — good practice
let value = {
    let guard = mutex.lock().unwrap();
    *guard  // copy the value, then guard drops → lock released
};
// Lock is already released here
```

---

## 4. Arc\<Mutex\<T\>\> — The Combo

This is the standard pattern for shared mutable state across threads:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let departure_count = Arc::new(Mutex::new(0u32));
let mut handles = vec![];

for platform in 1..=4 {
    let count = Arc::clone(&departure_count);
    let handle = thread::spawn(move || {
        for _ in 0..10 {
            let mut guard = count.lock().unwrap();
            *guard += 1;
        }
        println!("Platform {} done", platform);
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Total departures: {}", *departure_count.lock().unwrap());
// Always prints: Total departures: 40
```

Why it works:
- `Arc` gives each thread its own reference-counted pointer to the mutex
- `Mutex` ensures only one thread mutates at a time
- The compiler verifies this at compile time — no data races possible

**TypeScript parallel:** There's no equivalent because JS is single-threaded. The closest
concept is a database transaction lock — multiple requests share a resource, and the lock
ensures they don't conflict.

---

## 5. RwLock\<T\> — Multiple Readers, One Writer

`Mutex` is pessimistic — even readers block each other. `RwLock` (Read-Write Lock) is
more efficient when reads vastly outnumber writes:

```rust
use std::sync::RwLock;

let schedule = RwLock::new(vec!["Rajdhani 06:00", "Shatabdi 07:30"]);

// Multiple readers can hold the lock simultaneously
{
    let reader1 = schedule.read().unwrap();
    let reader2 = schedule.read().unwrap();  // ✅ both read at the same time
    println!("Reader 1 sees {} trains", reader1.len());
    println!("Reader 2 sees {} trains", reader2.len());
}

// Writer gets exclusive access — blocks until all readers release
{
    let mut writer = schedule.write().unwrap();
    writer.push("Duronto 09:00");
}
```

When to choose:
| Use case | Choose |
|---|---|
| Mostly reads, rare writes | `RwLock<T>` |
| Balanced reads and writes | `Mutex<T>` |
| Simple, low contention | `Mutex<T>` (simpler, less overhead) |

Wrap in `Arc` the same way: `Arc<RwLock<T>>`.

---

## 6. With Tokio: std::sync::Mutex vs tokio::sync::Mutex

Tokio has its own `Mutex`. The key difference:

| | `std::sync::Mutex` | `tokio::sync::Mutex` |
|---|---|---|
| Lock call | `lock()` — blocks the OS thread | `lock().await` — yields the async task |
| Hold across `.await`? | ❌ Dangerous (blocks the executor thread) | ✅ Designed for this |
| Performance | Faster for quick operations | Slight overhead from async machinery |
| Use when... | Lock is held briefly, no `.await` inside | You need to hold the lock across `.await` points |

**Rule of thumb:** Use `std::sync::Mutex` unless you need to hold the lock across an
`.await` point. It's faster and simpler.

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let board = Arc::new(Mutex::new(vec![]));

    let board_clone = Arc::clone(&board);
    let task1 = tokio::spawn(async move {
        let mut guard = board_clone.lock().await;  // async lock
        guard.push("Rajdhani — Platform 3");
        // Safe to .await here while holding the lock
    });

    let board_clone = Arc::clone(&board);
    let task2 = tokio::spawn(async move {
        let mut guard = board_clone.lock().await;
        guard.push("Shatabdi — Platform 5");
    });

    task1.await.unwrap();
    task2.await.unwrap();

    let guard = board.lock().await;
    println!("Board: {:?}", *guard);
}
```

Tokio also provides `tokio::sync::RwLock` with the same async semantics.

---

## 7. Common Pitfalls

### Deadlocks

If task A locks mutex 1 then tries to lock mutex 2, while task B locks mutex 2 then
tries to lock mutex 1 — both wait forever. Rust prevents data races, but **not** deadlocks.

```rust
// ⚠️ Potential deadlock — don't do this
let guard1 = mutex_a.lock().unwrap();
let guard2 = mutex_b.lock().unwrap();  // might deadlock if another thread locks in reverse
```

Prevention: always lock mutexes in the same order, or use a single mutex for related data.

### Holding Locks Too Long

```rust
// ❌ Bad: lock held during expensive operation
let guard = data.lock().unwrap();
expensive_computation(&guard);  // other threads blocked the entire time

// ✅ Good: clone the data, release the lock, then compute
let snapshot = {
    let guard = data.lock().unwrap();
    guard.clone()
};
expensive_computation(&snapshot);  // lock already released
```

### Poisoned Mutexes

If a thread panics while holding a lock, the mutex becomes "poisoned." `.lock()` returns
`Err(PoisonError)`. Usually `.unwrap()` is fine — if a thread panicked, you probably want
to propagate the panic.

---

## 8. Real-World Patterns

### Shared Application State

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

struct AppState {
    station_name: String,
    departures: Vec<String>,
    config: Config,
}

// Create once, share everywhere
let state = Arc::new(RwLock::new(AppState { /* ... */ }));

// Each handler/task clones the Arc
let state_clone = Arc::clone(&state);
tokio::spawn(async move {
    let reader = state_clone.read().await;
    println!("Station: {}", reader.station_name);
});
```

### In-Memory Cache

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type Cache = Arc<RwLock<HashMap<String, String>>>;

async fn get_schedule(cache: &Cache, train: &str) -> Option<String> {
    // Try read lock first (fast path)
    let reader = cache.read().await;
    if let Some(schedule) = reader.get(train) {
        return Some(schedule.clone());
    }
    drop(reader);  // release read lock before taking write lock

    // Cache miss — compute and store
    let schedule = fetch_from_database(train).await;
    let mut writer = cache.write().await;
    writer.insert(train.to_string(), schedule.clone());
    Some(schedule)
}
```

---

## 9. Comparison with TypeScript

| Concept | TypeScript | Rust |
|---|---|---|
| Threading model | Single-threaded event loop | Multi-threaded (threads + async tasks) |
| Shared state | Just use the variable — it's all one thread | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |
| Data races | Impossible (single thread) | Prevented at compile time |
| Race conditions | Possible (interleaved async ops) | Possible (interleaved tasks, even with locks) |
| Locking | Not needed | Required for shared mutable state |
| Worker threads | `new Worker()` with `postMessage` | `std::thread::spawn` with `Arc<Mutex<T>>` |

The mental shift: In TypeScript, you never think about concurrent access because JavaScript's
event loop serializes everything. In Rust, the compiler makes you think about it — but it
also guarantees you got it right. No "it works on my machine" data races in production.

Note the distinction between **data races** (two threads accessing the same memory without
synchronization — Rust prevents these) and **race conditions** (logic bugs from unexpected
ordering — these are still your responsibility in any language).

---

## 10. Mental Model Summary

```
                        Shared State Toolkit
                        ═══════════════════

  ┌──────────────┐      Arc<T>          ┌──────────────┐
  │   Thread 1   │ ──── clone ─────────▶│              │
  └──────────────┘                      │   Shared     │
  ┌──────────────┐      Arc<T>          │   Data       │
  │   Thread 2   │ ──── clone ─────────▶│   (Heap)     │
  └──────────────┘                      │              │
  ┌──────────────┐      Arc<T>          │              │
  │   Thread 3   │ ──── clone ─────────▶│              │
  └──────────────┘                      └──────┬───────┘
                                               │
                                        Protected by:
                                               │
                        ┌──────────────────────┼─────────────────────┐
                        │                      │                     │
                  Mutex<T>              RwLock<T>            tokio::sync::Mutex
                  One at a time         Many readers OR      Async-aware
                  Simple + fast         one writer           Hold across .await
```

1. **`Arc<T>`** — shared ownership (read-only, reference counted, thread-safe)
2. **`Mutex<T>`** — mutual exclusion (one accessor at a time, auto-unlock via drop)
3. **`Arc<Mutex<T>>`** — the standard combo for shared mutable state across threads
4. **`RwLock<T>`** — optimized for read-heavy workloads (many readers OR one writer)
5. **`tokio::sync::Mutex`** — use when you need to hold a lock across `.await`
6. **Keep locks short** — clone data out, release the lock, then do expensive work
7. The compiler prevents data races. Deadlocks and race conditions are still on you. 🚂
