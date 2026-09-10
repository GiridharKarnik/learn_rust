# Lesson 12 — JSON Handling with Serde

In TypeScript, JSON is effortless — `JSON.parse()` and `JSON.stringify()` just work. But
they give you `any`, and you're on your own for validation. Rust flips this: you tell the
compiler *exactly* what shape your data has, and Serde handles the rest — with full type
safety, at zero runtime cost.

Think of it like a railway ticketing system: every ticket (JSON document) must match the
exact format the gate (your struct) expects. No sneaking through with the wrong fields.

---

## 1. What Is Serde?

**Serde** = **SER**ialize + **DE**serialize.

It's not one crate — it's an ecosystem:

| Crate | Purpose |
|---|---|
| `serde` | The framework — defines the `Serialize` and `Deserialize` traits |
| `serde_json` | The JSON format adapter |
| `serde_yaml`, `toml`, `bincode`, ... | Other formats (same traits, different wire format) |

The key insight: you derive `Serialize`/`Deserialize` **once** on your types, and they
work with *any* format that has a Serde adapter. Define your train schedule struct once,
and you can output it as JSON, YAML, TOML, or binary — no extra work.

---

## 2. Setup

Add these to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

The `"derive"` feature gives you `#[derive(Serialize, Deserialize)]` — the magic that
auto-generates all the conversion code at compile time.

Then import in your Rust file:

```rust
use serde::{Serialize, Deserialize};
```

---

## 3. Serialization — Rust to JSON

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: f64,
}

let train = Train {
    name: "Rajdhani Express".to_string(),
    number: 12001,
    speed_kmh: 130.0,
};

let json = serde_json::to_string(&train)?;        // compact one-liner
let json = serde_json::to_string_pretty(&train)?;  // formatted with indentation
```

`#[derive(Serialize)]` auto-generates the `Serialize` trait implementation. The derive
macro inspects your struct's fields at compile time and writes the conversion code for you.

**Type mappings** (Rust → JSON):

| Rust Type | JSON Type |
|---|---|
| `String`, `&str` | string |
| `u32`, `i64`, `f64`, ... | number |
| `bool` | boolean |
| `Vec<T>` | array |
| `HashMap<String, T>` | object |
| `Option<T>` → `Some(v)` | value |
| `Option<T>` → `None` | null |

---

## 4. Deserialization — JSON to Rust

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: f64,
}

let json = r#"{"name":"Rajdhani Express","number":12001,"speed_kmh":130.0}"#;
let train: Train = serde_json::from_str(json)?;

println!("{}", train.name);  // Rajdhani Express
```

Key behaviors:
- Returns `Result` — invalid JSON gives you an `Err`, not a panic
- **Missing required fields** → error
- **Extra fields** → silently ignored by default
- **Wrong types** → error (e.g., string where number expected)

This is where Rust blows TypeScript out of the water. In TS, `JSON.parse()` returns `any`
and you have to use Zod or io-ts for validation. In Rust, the struct definition *is* the
validation. If the JSON doesn't match, you get a clear error at the exact point of failure.

---

## 5. Optional Fields

```rust
#[derive(Serialize, Deserialize)]
struct Train {
    name: String,
    number: u32,
    delay_minutes: Option<u32>,  // null or missing in JSON → None
}
```

When deserializing:
- JSON `null` → `None`
- Field missing entirely → `None`
- Field present with value → `Some(value)`

When serializing:
- `None` → `null` in JSON
- `Some(value)` → the value

This maps naturally to how you'd handle nullable fields in a TypeScript interface:
`delay_minutes?: number | null`.

---

## 6. Renaming and Customizing

Rust conventions use `snake_case`, but JSON APIs typically use `camelCase`. Serde makes
bridging this trivial:

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // all fields → camelCase in JSON
struct StationBoard {
    station_name: String,      // → "stationName" in JSON
    platform_count: u8,        // → "platformCount" in JSON

    #[serde(rename = "id")]
    station_code: String,      // → "id" in JSON (custom rename)

    #[serde(skip)]
    internal_only: bool,       // not included in JSON at all

    #[serde(default)]
    is_active: bool,           // uses Default::default() (false) if missing from JSON
}
```

Common `rename_all` values: `"camelCase"`, `"SCREAMING_SNAKE_CASE"`, `"kebab-case"`,
`"PascalCase"`.

This is one of the most-used Serde features — almost every real-world API uses camelCase,
and `rename_all` saves you from writing individual renames on every field.

---

## 7. Enums in JSON

Rust enums map to JSON in several ways. The most useful for API work is **internally
tagged** — which works exactly like TypeScript's discriminated unions:

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]  // ← adds a "type" discriminator field
enum TrainEvent {
    Departure { train: String, platform: u8 },
    Arrival { train: String, platform: u8 },
    Delay { train: String, minutes: u32 },
}
```

**Without** `#[serde(tag = "type")]` (externally tagged — the default):
```json
{"Departure": {"train": "Rajdhani", "platform": 3}}
```

**With** `#[serde(tag = "type")]` (internally tagged):
```json
{"type": "Departure", "train": "Rajdhani", "platform": 3}
```

The internally tagged version is the same pattern as TypeScript discriminated unions:

```typescript
// TypeScript equivalent
type TrainEvent =
  | { type: "Departure"; train: string; platform: number }
  | { type: "Arrival";   train: string; platform: number }
  | { type: "Delay";     train: string; minutes: number };
```

> **Note:** `#[serde(tag = "...")]` requires struct variants (named fields) or unit
> variants. Tuple variants like `Delayed(u32)` are not supported with internal tagging —
> use `Delayed { minutes: u32 }` instead.

---

## 8. Working with Untyped JSON

Sometimes you don't know the JSON structure ahead of time. Serde has you covered:

```rust
let json = r#"{"train":"Rajdhani","number":12001,"stops":["Chennai","Delhi"]}"#;
let v: serde_json::Value = serde_json::from_str(json)?;

// Dynamic access — like JavaScript
println!("{}", v["train"]);      // "Rajdhani"
println!("{}", v["stops"][0]);   // "Chennai"
println!("{}", v["missing"]);    // null (no panic!)
```

You can also *build* JSON dynamically with the `json!` macro:

```rust
let announcement = serde_json::json!({
    "message": "Train arriving",
    "platform": 3,
    "coaches": ["S1", "S2", "A1"],
});
```

Use `Value` when you're exploring an API or handling truly dynamic data. Prefer typed
structs for everything else — they're faster, safer, and self-documenting.

---

## 9. Vec and HashMap Serialization

Collections serialize naturally:

```rust
// Vec<Train> → JSON array
let trains = vec![train1, train2, train3];
let json = serde_json::to_string(&trains)?;
// [{"name":"Rajdhani",...}, {"name":"Shatabdi",...}, ...]

// HashMap<String, T> → JSON object
let mut speeds: HashMap<String, f64> = HashMap::new();
speeds.insert("Rajdhani".to_string(), 130.0);
speeds.insert("Shatabdi".to_string(), 150.0);
let json = serde_json::to_string(&speeds)?;
// {"Rajdhani":130.0,"Shatabdi":150.0}
```

Any combination works: `Vec<HashMap<...>>`, `HashMap<String, Vec<...>>`, structs
containing Vecs and HashMaps — Serde handles the nesting automatically.

> **Tip:** If you need deterministic key order in JSON output (e.g., for tests), use
> `BTreeMap` instead of `HashMap`. BTreeMap sorts keys alphabetically.

---

## 10. Comparison with TypeScript

| TypeScript | Rust + Serde |
|---|---|
| `JSON.parse(str)` | `serde_json::from_str(str)?` |
| `JSON.stringify(obj)` | `serde_json::to_string(&obj)?` |
| Returns `any` — no type safety | Returns typed `Result<T, Error>` |
| Runtime type checking (none by default) | Compile-time struct validation |
| Zod / io-ts / yup for validation | Serde validates by default |
| `obj.field` on `any` — might be undefined | `value["field"]` on `Value` — returns `Null` |
| camelCase is native | `#[serde(rename_all = "camelCase")]` |
| Discriminated unions (`type` field) | `#[serde(tag = "type")]` on enums |

The mental shift: in TypeScript, you parse first and validate later (if at all). In Rust,
parsing *is* validation — if `from_str` succeeds, your data is guaranteed to match the
struct. No more `if (data && data.name && typeof data.name === 'string')` chains.

---

## 11. Mental Model Summary

```
    Rust Struct                    Serde                        JSON String
  ┌─────────────┐          ┌─────────────────┐            ┌──────────────────┐
  │ Train {     │  ──────▶ │  Serialize      │  ────────▶ │ {"name":"...",   │
  │   name,     │          │  (to_string)    │            │  "number":12001} │
  │   number,   │          └─────────────────┘            └──────────────────┘
  │   speed,    │                                                  │
  │ }           │          ┌─────────────────┐                     │
  │             │  ◀────── │  Deserialize    │  ◀──────────────────┘
  └─────────────┘          │  (from_str)     │
                           └─────────────────┘
```

1. **Derive** `Serialize` and `Deserialize` on your types
2. **Customize** with `#[serde(...)]` attributes (rename, skip, default, tag)
3. **Serialize** with `serde_json::to_string()` / `to_string_pretty()`
4. **Deserialize** with `serde_json::from_str()` — returns `Result`, never panics
5. **Option fields** handle null/missing gracefully
6. **Enums** with `#[serde(tag = "type")]` give you TypeScript-style discriminated unions

Serde is one of Rust's greatest strengths — once you've used it, going back to
`JSON.parse()` returning `any` feels terrifying. 🚂
