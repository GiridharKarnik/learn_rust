# Lesson 14: Async Execution with Tokio

You already know `async`/`await` from TypeScript. Good news — the syntax in Rust
looks almost identical. The *mental model* underneath, however, is fundamentally
different. This lesson bridges that gap and gets you productive with **Tokio**,
Rust's most popular async runtime.

---

## 1 — Async in Rust vs TypeScript

### TypeScript
```ts
// One thread, one event loop. The runtime is baked into Node / the browser.
const train = await fetchTrain(42);
```

- The runtime (V8 + libuv) is **always there**.
- Every `Promise` is immediately scheduled on the event loop.
- Execution is **single-threaded** (ignoring workers).

### Rust
```rust
// No runtime is included by default. You choose one.
let train = fetch_train(42).await;
```

- `async fn` returns a **`Future`** — a value that *describes* work but does
  **nothing** until you `.await` it or hand it to a runtime.
- Futures are a **zero-cost abstraction**: the compiler turns them into state
  machines with no heap allocation for the future itself.
- Rust has no built-in event loop. You bring your own runtime — and the most
  widely-used one is **Tokio**.

> **Railway analogy:** In TypeScript, every train departs the moment you buy a
> ticket. In Rust, buying a ticket (calling an async fn) just reserves a seat.
> The train doesn't move until the dispatcher (the runtime) says go.

---

## 2 — What is Tokio?

Tokio is an **async runtime** for Rust — think of it as Node.js's event loop,
but **multi-threaded** by default.

It provides:

| Feature | What it does |
|---|---|
| **Task scheduler** | Runs your futures across a thread pool |
| **`tokio::time`** | Sleep, timeout, interval — like `setTimeout` / `setInterval` |
| **`tokio::sync`** | Channels, mutexes, semaphores for async code |
| **`tokio::spawn`** | Launch concurrent tasks (like lightweight threads) |
| **I/O drivers** | Async TCP, UDP, filesystem (not covered this lesson) |

Tokio is to Rust what libuv is to Node — except it can use **all your CPU
cores** out of the box.

---

## 3 — Setup

### Cargo.toml
```toml
[package]
name = "railway-async"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
```

The `"full"` feature flag enables everything (scheduler, time, sync, I/O).
In production you'd cherry-pick only the features you need.

### The `#[tokio::main]` macro

```rust
#[tokio::main]
async fn main() {
    println!("🚂 Dispatcher online");
}
```

This macro rewrites your `main` into something like:

```rust
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            println!("🚂 Dispatcher online");
        });
}
```

You don't have to think about this — just slap `#[tokio::main]` on `main` and
write async code.

---

## 4 — `async fn` and `.await`

### Declaring an async function

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn fetch_train_status(number: u32) -> String {
    // Simulate a network call
    sleep(Duration::from_millis(200)).await;
    format!("Train {} is on time", number)
}
```

Key points:

- `async fn` automatically wraps the return type in a `Future`.
  `async fn foo() -> String` actually returns `impl Future<Output = String>`.
- The function body **does not execute** until you `.await` the future.
- `.await` can only be used inside an `async` context.

### Calling it

```rust
#[tokio::main]
async fn main() {
    let status = fetch_train_status(42).await;
    println!("{status}");
}
```

> **TypeScript parallel:**
> ```ts
> async function fetchTrainStatus(n: number): Promise<string> { ... }
> const status = await fetchTrainStatus(42);
> ```
> Almost identical syntax. The difference: Rust's future is **lazy** — it does
> zero work until `.await`ed.

---

## 5 — Sequential vs Concurrent Execution

This is where Rust async gets interesting — and where the TypeScript comparison
is most illuminating.

### Sequential (one after another)

```rust
async fn sequential() {
    let a = fetch_train_status(1).await;   // waits 200ms
    let b = fetch_train_status(2).await;   // then waits another 200ms
    println!("{a}, {b}");
    // Total: ~400ms
}
```

This is exactly like:
```ts
const a = await fetchTrainStatus(1);  // 200ms
const b = await fetchTrainStatus(2);  // 200ms — total 400ms
```

### Concurrent with `tokio::join!`

```rust
async fn concurrent() {
    let (a, b) = tokio::join!(
        fetch_train_status(1),
        fetch_train_status(2),
    );
    println!("{a}, {b}");
    // Total: ~200ms — both ran at the same time!
}
```

TypeScript equivalent:
```ts
const [a, b] = await Promise.all([
    fetchTrainStatus(1),
    fetchTrainStatus(2),
]);
// Total: ~200ms
```

`tokio::join!` is like `Promise.all` — it polls all futures concurrently and
returns when **all** complete.

### Dynamic number of tasks with `JoinSet`

When you don't know the number of tasks at compile time:

```rust
use tokio::task::JoinSet;

async fn fetch_many(numbers: &[u32]) -> Vec<String> {
    let mut set = JoinSet::new();

    for &n in numbers {
        set.spawn(async move {
            fetch_train_status(n).await
        });
    }

    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        results.push(res.unwrap());
    }
    results
}
```

---

## 6 — `tokio::spawn`: Fire-and-Forget Tasks

`tokio::spawn` launches a future as an independent task on the runtime — like
starting a background job.

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("🚂 Background train departed");
        42
    });

    println!("⏳ Main task continues immediately");

    // .await the handle to get the result
    let result = handle.await.unwrap();
    println!("Background task returned: {result}");
}
```

Output:
```
⏳ Main task continues immediately
🚂 Background train departed
Background task returned: 42
```

> **Important:** `tokio::spawn` requires the future to be `'static` — it can't
> borrow local variables. Use `move` closures or clone/own the data.

---

## 7 — `tokio::time`: Sleep, Timeout, Interval

### `sleep` — like `setTimeout` (but you await it)

```rust
use tokio::time::{sleep, Duration};

async fn delayed_departure(secs: u64) {
    println!("🕐 Train departing in {secs}s...");
    sleep(Duration::from_secs(secs)).await;
    println!("🚂 Departed!");
}
```

### `timeout` — add a deadline to any future

```rust
use tokio::time::timeout;

async fn fetch_with_deadline() {
    let result = timeout(
        Duration::from_secs(2),
        fetch_train_status(42),
    ).await;

    match result {
        Ok(status) => println!("Got status: {status}"),
        Err(_) => println!("⏰ Timed out!"),
    }
}
```

`timeout` returns `Result<T, Elapsed>` — `Ok` if the inner future completed in
time, `Err` if the deadline expired.

### `interval` — like `setInterval`

```rust
use tokio::time::{interval, Duration};

async fn announce_departures() {
    let mut ticker = interval(Duration::from_secs(5));

    for i in 0..3 {
        ticker.tick().await;
        println!("📢 Departure announcement #{}", i + 1);
    }
}
```

---

## 8 — Async and `Result`

Async functions compose beautifully with `Result` and the `?` operator:

```rust
use std::fmt;

#[derive(Debug)]
struct TrainError(String);

impl fmt::Display for TrainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrainError: {}", self.0)
    }
}

impl std::error::Error for TrainError {}

async fn fetch_departure(number: u32) -> Result<String, TrainError> {
    if number == 0 {
        return Err(TrainError("Invalid train number".into()));
    }

    sleep(Duration::from_millis(100)).await;
    Ok(format!("Train {number} departs Platform 3"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let info = fetch_departure(42).await?;
    println!("{info}");
    Ok(())
}
```

The `?` operator works inside async functions exactly as it does in sync code.
You can also use `#[tokio::main]` with a `Result` return type on `main`.

---

## 9 — Channels: `tokio::sync::mpsc`

Channels let async tasks communicate — a **producer** sends messages, a
**consumer** receives them. This is the async equivalent of Node.js event
emitters or streams.

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Create a channel with buffer size 32
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // Producer task
    tokio::spawn(async move {
        let trains = ["Eurostar 9001", "TGV 7200", "ICE 501"];
        for name in trains {
            tx.send(format!("🚂 {name} departed")).await.unwrap();
            sleep(Duration::from_millis(300)).await;
        }
        // tx is dropped here → channel closes
    });

    // Consumer: receive until channel closes
    while let Some(msg) = rx.recv().await {
        println!("📡 Received: {msg}");
    }

    println!("✅ All trains dispatched");
}
```

Output:
```
📡 Received: 🚂 Eurostar 9001 departed
📡 Received: 🚂 TGV 7200 departed
📡 Received: 🚂 ICE 501 departed
✅ All trains dispatched
```

### Channel types in Tokio

| Channel | Use case |
|---|---|
| `mpsc` | Multiple producers, single consumer (most common) |
| `oneshot` | Single value, one sender, one receiver (like a Promise) |
| `broadcast` | Multiple consumers all see every message |
| `watch` | Single value, notifies consumers when it changes |

---

## 10 — Comparison with TypeScript

| Concept | TypeScript | Rust + Tokio |
|---|---|---|
| Async function | `async function f(): Promise<T>` | `async fn f() -> T` (returns `impl Future<Output=T>`) |
| Await | `await promise` | `future.await` |
| Run concurrently | `Promise.all([a, b])` | `tokio::join!(a, b)` |
| Fire-and-forget | `f()` (no await) | `tokio::spawn(f())` |
| Sleep | `setTimeout` + Promise wrapper | `tokio::time::sleep(duration).await` |
| Timeout | `Promise.race([p, timeout])` | `tokio::time::timeout(duration, future).await` |
| Interval | `setInterval(cb, ms)` | `tokio::time::interval(duration)` |
| Channel / events | `EventEmitter`, streams | `tokio::sync::mpsc` |
| Thread model | Single-threaded | Multi-threaded by default |
| Lazy? | No — Promise starts immediately | Yes — Future does nothing until polled |
| Runtime | Built into Node/browser | You choose (Tokio, async-std, smol…) |

The **laziness** difference is the biggest gotcha. In TypeScript, calling an
async function starts the work immediately. In Rust, it just creates a Future.
Nothing happens until you `.await` it or `spawn` it.

---

## 11 — Mental Model

Think of Tokio as a **railway dispatch center**:

```
┌─────────────────────────────────────────────┐
│              Tokio Runtime                  │
│         (dispatch control room)             │
│                                             │
│   ┌─────────┐  ┌─────────┐  ┌─────────┐   │
│   │ Worker  │  │ Worker  │  │ Worker  │   │
│   │ Thread 1│  │ Thread 2│  │ Thread 3│   │
│   └────┬────┘  └────┬────┘  └────┬────┘   │
│        │            │            │         │
│   ┌────┴────┐  ┌────┴────┐  ┌────┴────┐   │
│   │ Task A  │  │ Task B  │  │ Task D  │   │
│   │ Task C  │  │         │  │ Task E  │   │
│   └─────────┘  └─────────┘  └─────────┘   │
└─────────────────────────────────────────────┘
```

- **Tasks** are like trains waiting at the platform.
- **Worker threads** are like tracks — multiple trains can move simultaneously.
- **`.await`** is a train stopping at a signal — it yields the track so another
  train can proceed.
- **`tokio::join!`** dispatches multiple trains at once and waits for all to
  arrive.
- **`tokio::spawn`** sends a train off on its own — you can check on it later
  (or not).
- **Channels (`mpsc`)** are the radio system — tasks communicate without
  blocking each other.

### Key takeaways

1. **Async in Rust is lazy** — futures do nothing until polled.
2. **Tokio is the runtime** — it drives your futures like a dispatcher drives
   trains.
3. **`tokio::join!`** = run concurrently (like `Promise.all`).
4. **`tokio::spawn`** = fire-and-forget task (like calling an async function
   without `await` in TS, but safe).
5. **Channels** are the idiomatic way to pass data between async tasks.
6. **`?` works in async** — combine `Result` and `async` freely.

---

## Next Steps

In the exercise you'll build a **Railway Dispatch Simulator** — fetching train
statuses sequentially and concurrently, adding timeouts, and wiring up channels.
No real network calls — just `tokio::time::sleep` to simulate delays. You'll see
the timing differences with your own eyes.
