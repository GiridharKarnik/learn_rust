# Lesson 16 — Compile-Time SQL with sqlx

In TypeScript, you've probably used Prisma, Drizzle, or raw SQL strings to talk to
databases. The problem with raw SQL is obvious: a typo in a column name blows up at
runtime. Prisma and Drizzle solve this with code generation. Rust's `sqlx` takes a
different approach: it checks your SQL queries against your actual database **at compile
time**. If the query is wrong, your code won't even build.

Think of it like a railway reservation system: every booking (query) must match the exact
format the station database expects. sqlx is the automated inspector that catches invalid
bookings before they ever reach the counter.

---

## 1. What Is sqlx?

**sqlx** is an async, pure-Rust SQL crate with a killer feature: **compile-time query
verification**. It connects to your real database during `cargo build` and validates that
your SQL is correct — column names, types, table existence, everything.

Key traits:
- **Async-first** — built for Tokio (or async-std)
- **No DSL** — you write real SQL, not a query builder abstraction
- **Type-safe** — query results map to Rust types automatically
- **Multi-database** — PostgreSQL, MySQL, SQLite
- **Pure Rust** — no C dependencies for SQLite (uses bundled)

---

## 2. Setup

### Cargo.toml

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
tokio = { version = "1", features = ["full"] }
```

Feature flags control which database and runtime you use:

| Feature | What it enables |
|---|---|
| `runtime-tokio` | Tokio async runtime |
| `sqlite` | SQLite support |
| `postgres` | PostgreSQL support |
| `mysql` | MySQL support |

### The sqlx CLI (optional but useful)

```bash
cargo install sqlx-cli
```

This gives you `sqlx migrate`, `sqlx database create`, and `sqlx prepare` commands.

---

## 3. Connecting to a Database

For SQLite, you can use a file path or an in-memory database:

```rust
use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // In-memory database (great for testing, exercises)
    let pool = SqlitePool::connect(":memory:").await?;

    // Or a file-based database
    // let pool = SqlitePool::connect("sqlite:railway.db").await?;

    println!("Connected!");
    Ok(())
}
```

`SqlitePool` is a **connection pool** — it manages multiple connections and hands them
out as needed. This is the standard way to use sqlx: create the pool once, share it
everywhere (via function parameters, or `Arc` in complex apps).

For PostgreSQL, you'd use `PgPool::connect("postgres://user:pass@host/db")`.

---

## 4. Creating Tables

Execute raw SQL with `sqlx::query()`:

```rust
sqlx::query(
    "CREATE TABLE IF NOT EXISTS trains (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        from_station TEXT NOT NULL,
        to_station TEXT NOT NULL,
        total_seats INTEGER NOT NULL
    )"
)
.execute(&pool)
.await?;
```

`.execute()` runs the query and returns `SqliteQueryResult` with info like rows affected.
No rows are returned — it's a DDL statement.

---

## 5. Inserting Data

```rust
let result = sqlx::query(
    "INSERT INTO trains (name, from_station, to_station, total_seats)
     VALUES (?, ?, ?, ?)"
)
.bind("Rajdhani Express")
.bind("Chennai")
.bind("New Delhi")
.bind(500)
.execute(&pool)
.await?;

println!("Inserted train with id: {}", result.last_insert_rowid());
```

Key details:
- Use `?` for parameter placeholders (SQLite and MySQL). PostgreSQL uses `$1`, `$2`, etc.
- `.bind()` is called once per parameter, in order
- Bind parameters prevent SQL injection — never interpolate user input into SQL strings

---

## 6. Querying Data — The Basics

### Fetching rows dynamically

```rust
let rows = sqlx::query("SELECT id, name, total_seats FROM trains")
    .fetch_all(&pool)
    .await?;

for row in rows {
    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let seats: i64 = row.get("total_seats");
    println!("Train #{}: {} ({} seats)", id, name, seats);
}
```

`row.get("column_name")` returns the value, but you need to specify the Rust type.
This is the dynamic approach — it works but isn't very ergonomic.

### Fetching into structs with `query_as`

The idiomatic approach — map rows directly to Rust structs:

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Train {
    id: i64,
    name: String,
    from_station: String,
    to_station: String,
    total_seats: i64,
}

let trains: Vec<Train> = sqlx::query_as::<_, Train>(
    "SELECT id, name, from_station, to_station, total_seats FROM trains"
)
.fetch_all(&pool)
.await?;

for train in &trains {
    println!("{:?}", train);
}
```

`#[derive(FromRow)]` auto-generates the mapping from SQL columns to struct fields. Column
names must match field names (or you can use `#[sqlx(rename = "...")]`).

---

## 7. Compile-Time Checked Queries (The Killer Feature)

This is what makes sqlx special. With the `query!` macro, sqlx connects to your database
**during compilation** and verifies the query:

```rust
// This is checked at COMPILE TIME against your actual database schema
let train = sqlx::query!("SELECT id, name FROM trains WHERE id = ?", 1)
    .fetch_one(&pool)
    .await?;

// train.id and train.name are strongly typed — no .get() needed
println!("Train: {} (id: {})", train.name, train.id);
```

For this to work, you need:
1. `DATABASE_URL` environment variable pointing to your database
2. Run `cargo sqlx prepare` to cache query metadata (for CI builds without a DB)

**In this course**, we use the runtime-checked `query()` and `query_as()` functions
instead of the `query!()` macro — they don't require a database at compile time, which
makes the exercises simpler to run. In production, you'd typically use `query!()` for
maximum safety.

---

## 8. Fetch Variants

sqlx provides several fetch methods:

| Method | Returns | Use when |
|---|---|---|
| `.fetch_one()` | Single row | You expect exactly one result (errors if 0 or 2+) |
| `.fetch_optional()` | `Option<Row>` | Zero or one result |
| `.fetch_all()` | `Vec<Row>` | Any number of results |
| `.fetch()` | `Stream<Row>` | Large result sets (streaming) |

```rust
// Fetch one — panics if not found
let train = sqlx::query_as::<_, Train>("SELECT * FROM trains WHERE id = ?")
    .bind(1)
    .fetch_one(&pool)
    .await?;

// Fetch optional — returns None if not found
let maybe_train = sqlx::query_as::<_, Train>("SELECT * FROM trains WHERE id = ?")
    .bind(999)
    .fetch_optional(&pool)
    .await?;

match maybe_train {
    Some(t) => println!("Found: {}", t.name),
    None => println!("Train not found"),
}
```

---

## 9. Updates and Deletes

```rust
// Update
let result = sqlx::query("UPDATE trains SET total_seats = ? WHERE id = ?")
    .bind(550)
    .bind(1)
    .execute(&pool)
    .await?;
println!("Rows updated: {}", result.rows_affected());

// Delete
let result = sqlx::query("DELETE FROM trains WHERE id = ?")
    .bind(1)
    .execute(&pool)
    .await?;
println!("Rows deleted: {}", result.rows_affected());
```

Always check `rows_affected()` to confirm the operation actually did something — a
`DELETE WHERE id = 999` on a non-existent row returns 0, not an error.

---

## 10. Transactions

Transactions ensure a group of operations either all succeed or all roll back:

```rust
let mut tx = pool.begin().await?;

sqlx::query("INSERT INTO reservations (passenger, train_id) VALUES (?, ?)")
    .bind("Amit")
    .bind(1)
    .execute(&mut *tx)
    .await?;

sqlx::query("UPDATE trains SET available_seats = available_seats - 1 WHERE id = ?")
    .bind(1)
    .execute(&mut *tx)
    .await?;

tx.commit().await?;  // Both operations committed atomically
// If any query failed, tx.rollback().await? (or just drop tx — auto-rollbacks)
```

Note: you pass `&mut *tx` (a mutable reference to the transaction) instead of `&pool`.

Transactions are essential for operations like booking a ticket: you need to insert the
reservation AND decrement available seats atomically. If the decrement fails, the
reservation should not exist.

---

## 11. Migrations

sqlx has built-in migration support:

```bash
# Create a new migration
sqlx migrate add create_trains_table

# This creates: migrations/20240101120000_create_trains_table.sql
# Write your SQL in that file, then:

sqlx migrate run
```

Migration files are plain SQL. They run in order and are tracked in a `_sqlx_migrations`
table so they only execute once.

For production apps, migrations are the proper way to manage schema. For exercises and
prototyping, creating tables in code (like we do below) is perfectly fine.

---

## 12. Type Mappings

SQLite types map to Rust types:

| SQLite Type | Rust Type |
|---|---|
| `INTEGER` | `i64` (or `i32`) |
| `TEXT` | `String` |
| `REAL` | `f64` |
| `BLOB` | `Vec<u8>` |
| `NULL` | `Option<T>` |
| `BOOLEAN` (INTEGER 0/1) | `bool` |

For PostgreSQL, the mappings are richer (UUID, JSONB, arrays, timestamps, etc.).

---

## 13. Comparison with TypeScript

| Feature | TypeScript (Prisma/Drizzle) | Rust (sqlx) |
|---|---|---|
| Query language | DSL / query builder | Raw SQL |
| Type safety | Generated types from schema | Compile-time checked against DB |
| Schema management | Schema file → migrations | SQL migrations |
| ORM features | Relations, includes, nested queries | Manual JOINs (it's not an ORM) |
| Async | Promise-based | async/await with Tokio |
| Connection pool | Built-in | Built-in (`PgPool`, `SqlitePool`) |
| SQL injection prevention | Parameterized by default | `.bind()` parameters |

The key difference: Prisma and Drizzle generate a type-safe client from your schema
definition. sqlx checks your actual SQL against the real database at compile time. Both
achieve type safety, but sqlx lets you write real SQL — no query builder abstractions,
no impedance mismatch.

If you loved writing raw SQL but hated the lack of type safety, sqlx is your crate.

---

## 14. Mental Model Summary

```
                            sqlx Flow
                            ═════════

  ┌──────────────┐     SqlitePool::connect()     ┌──────────────┐
  │  Rust Code   │ ────────────────────────────▶  │   Database   │
  │              │                                │   (SQLite)   │
  │  query_as()  │ ◀── rows mapped to structs ──  │              │
  │  query()     │ ◀── dynamic rows ────────────  │              │
  │  execute()   │ ◀── rows_affected ───────────  │              │
  └──────────────┘                                └──────────────┘

  Compile-time mode (query! macro):
    cargo build → sqlx connects to DB → validates SQL → build succeeds or fails

  Runtime mode (query() function):
    SQL checked when executed → errors returned as Result
```

1. **Connect** with `SqlitePool::connect()` — creates a connection pool
2. **Execute** DDL with `sqlx::query("CREATE TABLE ...").execute(&pool)`
3. **Insert** with `sqlx::query("INSERT ...").bind(value).execute(&pool)`
4. **Select** with `sqlx::query_as::<_, MyStruct>("SELECT ...").fetch_all(&pool)`
5. **Transactions** with `pool.begin()` → execute → `tx.commit()`
6. **FromRow** derive maps SQL columns to struct fields automatically
7. Use `query!()` for compile-time checking in production, `query()` for simplicity 🚂
