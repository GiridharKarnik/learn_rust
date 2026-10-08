# Guard API — Specification v0.1

> Rate limiting and idempotency as a service, for serverless and vibe-coded apps.
> Working name: **Guard**. Built in Rust (axum + tokio + sqlx/SQLite). Distributed via RapidAPI.

**Status:** Draft — build target
**Audience:** You, building it. Each section is meant to be implementable on its own.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Goals and Non-Goals](#2-goals-and-non-goals)
3. [Glossary](#3-glossary)
4. [API Conventions](#4-api-conventions)
5. [Authentication and Tenancy](#5-authentication-and-tenancy)
6. [Rate Limiting API](#6-rate-limiting-api)
7. [Rate Limiting Algorithms](#7-rate-limiting-algorithms)
8. [Idempotency API](#8-idempotency-api)
9. [Idempotency State Machine](#9-idempotency-state-machine)
10. [Validation Rules and Hard Limits](#10-validation-rules-and-hard-limits)
11. [Error Catalogue](#11-error-catalogue)
12. [Architecture](#12-architecture)
13. [Persistence](#13-persistence)
14. [Configuration](#14-configuration)
15. [Observability](#15-observability)
16. [Failure Modes and Guarantees](#16-failure-modes-and-guarantees)
17. [Security and Data Handling](#17-security-and-data-handling)
18. [Project Structure](#18-project-structure)
19. [Testing Strategy](#19-testing-strategy)
20. [Milestones](#20-milestones)
21. [Client SDK (TypeScript)](#21-client-sdk-typescript)
22. [RapidAPI Listing and Pricing](#22-rapidapi-listing-and-pricing)
23. [Future Work](#23-future-work)
24. [Open Questions](#24-open-questions)

---

## 1. Overview

Serverless apps (Vercel, Netlify, Cloudflare Workers, Supabase Edge Functions) run on short-lived,
stateless instances. Two common safety mechanisms need **shared state** across instances and are
easy to get subtly wrong:

| Capability | Question it answers | Typical use |
|---|---|---|
| **Rate limiting** | "Is this caller allowed to do this right now?" | AI spend caps, free-tier quotas, login brute-force protection |
| **Idempotency** | "Has this operation already happened?" | Payments, webhook handling, emails, job retries |

Guard provides both behind a small JSON-over-HTTPS API. Customers send **opaque keys**; Guard stores
**counters, timestamps and small results**, never business data.

---

## 2. Goals and Non-Goals

### Goals (v1)

- G1. Correct, atomic rate limiting with per-call `cost` (token budgets).
- G2. Correct, atomic idempotency with lock expiry, fencing tokens and stored results.
- G3. p99 server-side processing time **< 1 ms** for rate-limit checks, **< 5 ms** for idempotency
  writes (excludes network).
- G4. Single small VPS handles **≥ 5,000 req/s** of mixed traffic.
- G5. Strict tenant isolation: one customer can never read, modify or exhaust another's keys.
- G6. Error messages actionable enough that pasting them into an AI assistant fixes the bug.
- G7. Minimal data footprint: no personal data by design; everything has a TTL.

### Non-Goals (v1)

- Multi-region / globally distributed limits.
- Direct (non-RapidAPI) API keys and billing.
- Dashboards / web UI.
- Arbitrary data storage (results are capped and short-lived — this is not a KV store).
- WebSockets or streaming.

---

## 3. Glossary

| Term | Meaning |
|---|---|
| **Tenant** | A paying customer (one RapidAPI user). All state is namespaced by tenant. |
| **Key** | Caller-chosen string identifying what is limited or deduplicated, e.g. `user_42:ai`. |
| **Limit** | Maximum units allowed per window. |
| **Window** | Duration over which `limit` applies, e.g. `1m`, `1d`. |
| **Cost** | Units consumed by one call (default 1). Use token counts for AI budgets. |
| **TAT** | *Theoretical Arrival Time* — the single timestamp GCRA stores per key. |
| **Claim** | Request to become the sole executor of an idempotent operation. |
| **Lock TTL** | How long a claim stays exclusive before it's considered abandoned. |
| **Result TTL** | How long a completed result is remembered. |
| **Fence (fencing token)** | Monotonically increasing number issued per claim; proves the holder is still current. |

---

## 4. API Conventions

- **Base URL:** `https://<rapidapi-host>/v1`
- **Transport:** HTTPS only. JSON request and response bodies, `Content-Type: application/json`.
- **Methods:** All operations are `POST` with keys in the body (avoids URL-encoding problems with
  arbitrary key strings). Health endpoints are `GET`.
- **Durations:** Strings with a unit suffix: `ms`, `s`, `m`, `h`, `d`. Examples: `"500ms"`,
  `"30s"`, `"15m"`, `"1d"`. Integers are **not** accepted (avoids seconds-vs-milliseconds bugs).
- **Timestamps:** RFC 3339 UTC strings, e.g. `"2026-10-03T00:00:00Z"`, millisecond precision.
- **Relative times:** Always integer milliseconds with an `_ms` suffix, e.g. `retry_after_ms`.
- **Unknown fields:** Rejected with `400 unknown_field` (catches typos like `"limt"`).
  In serde: `#[serde(deny_unknown_fields)]`.
- **Business outcomes are not HTTP errors.** A denied rate limit is `200 OK` with
  `"allowed": false`. A completed idempotency key is `200 OK` with `"status": "completed"`.
  HTTP 4xx/5xx are reserved for invalid requests and server faults.
- **Request IDs:** Every response includes `X-Request-Id` (UUID v7). Echo a client-supplied
  `X-Request-Id` if present and valid.

### Error envelope

Every non-2xx response has this shape:

```json
{
  "error": {
    "code": "invalid_duration",
    "message": "Field `window` has invalid duration \"10 minutes\".",
    "hint": "Use a number followed by a unit: ms, s, m, h or d. Example: \"10m\".",
    "field": "window",
    "docs": "https://<docs-host>/errors#invalid_duration"
  }
}
```

- `code` — stable, machine-readable, snake_case. Never changes once published.
- `message` — what went wrong, mentioning the field and the offending value.
- `hint` — how to fix it, with a corrected example where possible.
- `field` — optional, the JSON path of the offending field.

---

## 5. Authentication and Tenancy

### 5.1 RapidAPI mode (production)

RapidAPI sits in front of Guard and forwards requests with headers:

| Header | Use |
|---|---|
| `X-RapidAPI-Proxy-Secret` | Shared secret proving the request came through RapidAPI. **Must match config, else `401`.** Compare in constant time. |
| `X-RapidAPI-User` | The subscriber's username. Becomes the **tenant ID**. Missing → `401`. |
| `X-RapidAPI-Subscription` | Plan name (e.g. `BASIC`, `PRO`). Used for per-plan hard limits (§10). |

### 5.2 Dev mode

When `GUARD_MODE=dev`, the proxy secret check is skipped and the tenant is read from
`X-Guard-Tenant` (default `dev`). Dev mode **must refuse to start** if bound to a non-loopback
address unless `GUARD_ALLOW_PUBLIC_DEV=true` is also set.

### 5.3 Isolation

- Internal storage key = `tenant_id` + `0x1F` (unit separator) + `namespace` + `0x1F` + `key`,
  where namespace is `rl` or `idem`. Tenant IDs never appear in responses.
- A tenant cannot enumerate keys. There is no list endpoint.
- Implement tenancy as an axum **extractor** (`Tenant`) so a handler cannot forget it.

---

## 6. Rate Limiting API

### 6.1 `POST /v1/limit` — check and consume

Atomically checks whether `cost` units are available and, if so, consumes them.

**Request**

```json
{
  "key": "user_8f2a:ai",
  "limit": 50000,
  "window": "1d",
  "cost": 1250,
  "algorithm": "gcra"
}
```

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `key` | string | yes | — | See §10 for format. |
| `limit` | integer | yes | — | 1 ..= 1,000,000,000 |
| `window` | duration | yes | — | 1s ..= 30d |
| `cost` | integer | no | `1` | 0 ..= `limit`. `0` behaves like a peek. |
| `algorithm` | `"gcra"` \| `"fixed_window"` | no | `"gcra"` | See §7. |

**Response `200`**

```json
{
  "allowed": true,
  "limit": 50000,
  "remaining": 48750,
  "reset_at": "2026-10-03T09:14:22.120Z",
  "retry_after_ms": null
}
```

| Field | Meaning |
|---|---|
| `allowed` | Whether `cost` units were consumed. |
| `remaining` | Units available **after** this call (whole units, rounded down). |
| `reset_at` | When the full `limit` will be available again if no further calls are made. |
| `retry_after_ms` | If denied: ms until `cost` units will be available. `null` if allowed. |

**Denied example**

```json
{
  "allowed": false,
  "limit": 50000,
  "remaining": 310,
  "reset_at": "2026-10-03T09:14:22.120Z",
  "retry_after_ms": 1622000
}
```

A denied call **consumes nothing**.

### 6.2 `POST /v1/limit/peek` — check without consuming

Same request as §6.1 minus `cost`. Same response, with `allowed` meaning "would a call with
`cost: 1` be allowed".

### 6.3 `POST /v1/limit/reset` — clear a key

```json
{ "key": "user_8f2a:ai" }
```

Response `200`: `{ "reset": true, "existed": true }`. Use case: user upgraded their plan.

### 6.4 `POST /v1/limit/batch` — multiple limits, all-or-nothing

Common need: "10 per minute **and** 500 per day".

```json
{
  "checks": [
    { "key": "user_42:ai:min", "limit": 10,  "window": "1m" },
    { "key": "user_42:ai:day", "limit": 500, "window": "1d" }
  ]
}
```

- Max 10 checks per batch. Keys must be distinct (else `400 duplicate_key`).
- **All-or-nothing:** units are consumed only if **every** check is allowed.
- Response: `{ "allowed": bool, "results": [ <§6.1 response>, ... ] }` in request order.
  When the batch is denied, each result shows what *would* have happened; nothing is consumed.
- Implementation: lock all involved shards in a **consistent order** (sorted by shard index) to
  avoid deadlocks.

### 6.5 Parameter changes on an existing key

`limit`, `window` and `algorithm` are supplied on every call, not stored as configuration.

- **GCRA:** the stored TAT is reinterpreted using the new parameters. This is well-defined
  (see §7.1) and generally does what users expect.
- **Fixed window:** if `window` changes, the counter resets.
- **Algorithm change** on an existing key: state is discarded and the key starts fresh.
  Document this; don't error.

---

## 7. Rate Limiting Algorithms

### 7.1 GCRA (default)

Generic Cell Rate Algorithm. Smooth, rolling refill. Stores **one `u64`** (TAT, in ms since epoch)
per key.

**Definitions** (all times in milliseconds, integer arithmetic):

```
T   = window_ms / limit              // emission interval: time to refill one unit
                                     //   use u128 or rational math to avoid precision loss
                                     //   when window_ms is not divisible by limit
now = current time
tat = max(stored_tat or now, now)    // a key never seen behaves like a full bucket
```

**Check and consume `cost` units:**

```
new_tat  = tat + T * cost
allow_at = new_tat - window_ms

if now >= allow_at:
    store new_tat
    allowed   = true
    remaining = floor((window_ms - (new_tat - now)) / T)
    retry_after_ms = null
else:
    allowed   = false             // state unchanged
    remaining = floor((window_ms - (tat - now)) / T)
    retry_after_ms = allow_at - now

reset_at = (allowed ? new_tat : tat)   // when TAT catches up with real time = full bucket
```

**Sanity checks to put in tests:**

- Fresh key, `limit=10`, `window=10s`, `cost=1` → allowed, `remaining=9`.
- 10 rapid calls → all allowed; 11th denied with `retry_after_ms ≈ 1000`.
- Wait 1s → exactly one more call allowed.
- `cost = limit` on a fresh key → allowed, `remaining = 0`.
- `cost = 0` → always allowed, state unchanged.

**Precision:** with `window = 30d` and `limit = 1e9`, `T` is ~2.6 ms. With `window = 1s` and
`limit = 1e9`, `T` is 0.000001 ms. Store TAT in **microseconds or nanoseconds** internally
(u64 nanoseconds covers ~584 years), and compute `T * cost` in `u128` to avoid overflow.

**Semantics to document for users:** GCRA is a *rolling* limit. `"1d"` means "refills at
`limit/day`, continuously", **not** "resets at midnight". Users wanting calendar resets use
`fixed_window`.

Because the bucket starts full and refills continuously, a caller can spend the whole `limit`
at once and then keep spending the refill — up to almost `2 × limit` within any single span of
`window`. Long-run average is exactly `limit / window`. Document this clearly. Users who need a
strict "never more than N in any 24h span" guarantee need a sliding-log algorithm (see §23).

### 7.2 Fixed window

For calendar-style quotas ("5 exports per day, reset at midnight UTC").

**State:** `(window_start_ms: u64, count: u64)`.

```
window_start = floor(now / window_ms) * window_ms   // aligned to Unix epoch, so "1d" = UTC midnight
if stored.window_start != window_start: count = 0

if count + cost <= limit:
    count += cost; allowed = true
else:
    allowed = false

remaining      = limit - count
reset_at       = window_start + window_ms
retry_after_ms = allowed ? null : reset_at - now
```

**Known trade-off (document it):** up to `2 × limit` can pass across a window boundary.

### 7.3 Memory eviction

- **GCRA:** a key whose `TAT <= now` is indistinguishable from a key that doesn't exist. Delete it.
- **Fixed window:** a key whose `window_start + window_ms <= now` can be deleted.
- A background **sweeper** task (§12.3) removes such keys. Memory is therefore bounded by the
  number of *active* keys, not historical ones.

---

## 8. Idempotency API

### 8.1 `POST /v1/idempotency/claim`

**Request**

```json
{
  "key": "stripe_evt_1Q2w3E",
  "lock_ttl": "30s",
  "result_ttl": "24h",
  "request_hash": "sha256:9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
}
```

| Field | Type | Required | Default | Notes |
|---|---|---|---|---|
| `key` | string | yes | — | §10. |
| `lock_ttl` | duration | no | `"30s"` | 1s ..= 15m |
| `result_ttl` | duration | no | `"24h"` | 1m ..= 7d |
| `request_hash` | string | no | — | Opaque, ≤ 128 chars. If supplied, retries with a different hash are rejected. |

**Responses (all `200`)**

Acquired — you are the executor:

```json
{
  "status": "acquired",
  "fence": 18273645,
  "lock_expires_at": "2026-10-02T14:00:30.000Z"
}
```

In progress — someone else holds a live lock:

```json
{
  "status": "in_progress",
  "retry_after_ms": 4200
}
```

Completed — the operation already happened:

```json
{
  "status": "completed",
  "result": { "order_id": "ord_991" },
  "completed_at": "2026-10-02T13:59:58.512Z"
}
```

**Error `409 request_mismatch`:** a record exists for this key with a different `request_hash`.

### 8.2 `POST /v1/idempotency/complete`

```json
{
  "key": "stripe_evt_1Q2w3E",
  "fence": 18273645,
  "result": { "order_id": "ord_991" }
}
```

- `result` is optional (any JSON value, ≤ plan limit — §10). Absent → stored as `null`.
- Succeeds only if the record is `in_progress` **and** `fence` matches the current fence.
  Lock expiry alone does **not** invalidate the fence — if no one else has claimed, a slow
  worker may still complete.
- Response `200`: `{ "status": "completed", "result_expires_at": "..." }`
- Errors: `409 lock_lost` (fence mismatch, already completed, or no record).

### 8.3 `POST /v1/idempotency/release`

The work failed; allow a retry to try again.

```json
{ "key": "stripe_evt_1Q2w3E", "fence": 18273645 }
```

- Deletes the record if `in_progress` and fence matches.
- Response `200`: `{ "released": true }`
- Errors: `409 lock_lost`.

### 8.4 `POST /v1/idempotency/extend`

Heartbeat for long-running work.

```json
{ "key": "stripe_evt_1Q2w3E", "fence": 18273645, "lock_ttl": "30s" }
```

- Sets `lock_expires_at = now + lock_ttl` if `in_progress`, fence matches.
- Response `200`: `{ "lock_expires_at": "..." }`
- Errors: `409 lock_lost`.

### 8.5 `POST /v1/idempotency/get`

Read-only inspection (debugging, dashboards).

```json
{ "key": "stripe_evt_1Q2w3E" }
```

Response `200`:

```json
{ "status": "in_progress" | "completed" | "absent", "lock_expires_at": "...", "completed_at": "...", "result": ... }
```

Fields present only where relevant. The `fence` is **never** returned here (it's a capability).

---

## 9. Idempotency State Machine

```
                     claim (absent or lock expired)
        ┌─────────────────────────────────────────────┐
        │                                             ▼
   ┌─────────┐                                 ┌─────────────┐
   │ ABSENT  │◄──────── release (fence ok) ────│ IN_PROGRESS │──── extend (fence ok) ──┐
   └─────────┘                                 │ fence=F     │◄────────────────────────┘
        ▲                                      │ lock_exp=t  │
        │                                      └─────────────┘
        │                                             │ complete (fence ok)
        │                                             ▼
        │                                      ┌─────────────┐
        └────────── result_ttl expires ────────│  COMPLETED  │
                                               │ result=R    │
                                               └─────────────┘
```

### 9.1 Transition table

| Current state | Operation | Condition | Outcome |
|---|---|---|---|
| ABSENT | claim | — | → IN_PROGRESS, new fence; `acquired` |
| IN_PROGRESS | claim | `now < lock_exp` | unchanged; `in_progress` |
| IN_PROGRESS | claim | `now >= lock_exp` | → IN_PROGRESS with **new fence**; `acquired` (lock takeover) |
| COMPLETED | claim | `now < result_exp` | unchanged; `completed` + result |
| COMPLETED | claim | `now >= result_exp` | treat as ABSENT |
| IN_PROGRESS | complete | fence matches | → COMPLETED |
| IN_PROGRESS | release | fence matches | → ABSENT |
| IN_PROGRESS | extend | fence matches | `lock_exp` updated |
| any other | complete / release / extend | — | `409 lock_lost` |
| any non-ABSENT | claim with `request_hash` ≠ stored | — | `409 request_mismatch` |

### 9.2 Fencing tokens

- Fences come from a **single global monotonic counter** (persisted, see §13). Never per-key —
  a per-key counter would reset on release and let a stale holder with fence `1` complete a newer
  claim that also got fence `1`.
- The fence is returned only from `claim`. Treat it as a secret capability.

### 9.3 Atomicity requirement

For any key, the operations above must be **linearizable**: under N concurrent `claim`s on an
ABSENT key, **exactly one** receives `acquired`. This is the single most important property to
test (§19).

---

## 10. Validation Rules and Hard Limits

### 10.1 Keys

- Length 1 ..= 256 bytes.
- Allowed characters: printable ASCII `0x21`–`0x7E` (no spaces, no control characters).
- Error hint should suggest hashing if the input looks like an email (contains `@`):
  *"Keys are stored and logged. Avoid personal data — hash it first, e.g. `sha256(email)`."*

### 10.2 Per-plan limits

Enforced from `X-RapidAPI-Subscription`. Values are starting points.

| Limit | Free | Hobby | Pro |
|---|---|---|---|
| Max `result` size (serialized) | 1 KB | 4 KB | 8 KB |
| Max `result_ttl` | 1d | 3d | 7d |
| Max active idempotency records | 1,000 | 50,000 | 500,000 |
| Max active rate-limit keys | 10,000 | 250,000 | 2,500,000 |

Exceeding an active-key cap → `429 tenant_capacity_exceeded` (this is the only use of 429).
Request volume quotas are enforced by RapidAPI, not Guard.

### 10.3 Global

- Request body ≤ 16 KB (`413 payload_too_large`).
- Batch ≤ 10 checks.

---

## 11. Error Catalogue

| HTTP | `code` | When |
|---|---|---|
| 400 | `invalid_json` | Body isn't valid JSON. |
| 400 | `unknown_field` | Body contains an unrecognised field. |
| 400 | `missing_field` | Required field missing. |
| 400 | `invalid_key` | Key empty, too long, or bad characters. |
| 400 | `invalid_duration` | Duration unparseable or missing unit. |
| 400 | `out_of_range` | Number or duration outside allowed range (message states the range). |
| 400 | `cost_exceeds_limit` | `cost > limit` — can never be allowed. |
| 400 | `duplicate_key` | Same key twice in a batch. |
| 400 | `invalid_fence` | Fence isn't a positive integer. |
| 401 | `unauthorized` | Missing/invalid proxy secret or tenant. |
| 404 | `not_found` | Unknown route. |
| 409 | `lock_lost` | Fence doesn't match current claim. |
| 409 | `request_mismatch` | `request_hash` differs from stored one. |
| 413 | `payload_too_large` | Body or `result` exceeds limits. |
| 415 | `unsupported_media_type` | Content-Type isn't JSON. |
| 429 | `tenant_capacity_exceeded` | Active key cap reached for plan. |
| 500 | `internal` | Bug. Message is generic; details go to logs with the request ID. |
| 503 | `unavailable` | Shutting down or storage unavailable. Includes `Retry-After`. |

Implement as a single `thiserror` enum `ApiError` with an `IntoResponse` impl that maps each
variant to status + envelope. Handlers return `Result<Json<T>, ApiError>`.

**Rule:** axum's built-in JSON rejection messages must be converted into this envelope (use a
custom `Json` extractor wrapper) — never leak default rejection text.

---

## 12. Architecture

### 12.1 Overview

```
                       ┌──────────────────────────── guard (single binary) ─────────────────────────┐
                       │                                                                            │
 RapidAPI ──HTTPS──▶   │  axum Router                                                               │
                       │   ├─ middleware: request id, tracing, body limit, timeout(2s)              │
                       │   ├─ extractor: Tenant (auth + plan)                                       │
                       │   ├─ /v1/limit*        ──▶  RateLimiter  ── sharded in-memory map ──┐      │
                       │   ├─ /v1/idempotency/* ──▶  IdemStore    ── in-memory cache         │      │
                       │   │                                       └─ SQLite (source of truth)│      │
                       │   └─ /healthz, /readyz, /metrics                                     │      │
                       │                                                                      │      │
                       │  background tasks (tokio::spawn):                                    │      │
                       │   ├─ sweeper     — evict expired rate-limit + idempotency entries    │      │
                       │   └─ snapshotter — persist dirty rate-limit state every N seconds ◀──┘      │
                       └────────────────────────────────────────────────────────────────────────────┘
```

### 12.2 Shared state

```rust
// Shape only — not a required API.
struct AppState {
    config: Arc<Config>,
    clock: Arc<dyn Clock>,          // trait so tests can control time
    rate_limiter: Arc<RateLimiter>,
    idem: Arc<IdemStore>,
}
```

- **`Clock` trait** with `now()` — the real impl reads system time; the test impl is settable.
  Every time-dependent function takes time from here. This is non-negotiable for testability.
- **Rate-limit map:** N shards (N = 64, power of two), each a `std::sync::Mutex<HashMap<StorageKey, RlState>>`.
  Shard = `hash(key) & (N - 1)`. Critical sections are tiny and never `.await`, so a std mutex is
  correct (lesson 17). Alternative: `DashMap` — fine to start with.
- **Idempotency:** SQLite is the source of truth. Optional in-memory cache of COMPLETED records
  for fast repeat claims.

### 12.3 Background tasks

| Task | Interval | Job |
|---|---|---|
| Sweeper | 10s | Remove expired rate-limit entries (§7.3); `DELETE` expired idempotency rows in batches of 1,000. |
| Snapshotter | 5s (configurable) | Write rate-limit entries modified since last snapshot to SQLite. |

Both use `tokio::time::interval` and stop on the shutdown signal.

### 12.4 Graceful shutdown

On SIGTERM/SIGINT:

1. `/readyz` starts returning `503`.
2. Stop accepting new connections; finish in-flight requests (deadline 10s).
3. Run a final snapshot.
4. Close the SQLite pool. Exit 0.

---

## 13. Persistence

SQLite via `sqlx`, WAL mode. One database file.

### 13.1 Pragmas

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous = FULL;   -- idempotency needs durability; revisit with benchmarks
PRAGMA busy_timeout = 5000;
PRAGMA foreign_keys = ON;
```

### 13.2 Schema (migration `0001_init.sql`)

```sql
CREATE TABLE idempotency (
    tenant           TEXT    NOT NULL,
    key              TEXT    NOT NULL,
    status           INTEGER NOT NULL,        -- 1 = in_progress, 2 = completed
    fence            INTEGER NOT NULL,
    request_hash     TEXT,
    lock_expires_at  INTEGER,                 -- unix ms, in_progress only
    result           BLOB,                    -- JSON bytes, completed only
    completed_at     INTEGER,
    expires_at       INTEGER NOT NULL,        -- row may be deleted after this
    PRIMARY KEY (tenant, key)
) WITHOUT ROWID;

CREATE INDEX idx_idempotency_expires ON idempotency (expires_at);

CREATE TABLE fence_counter (
    id    INTEGER PRIMARY KEY CHECK (id = 1),
    value INTEGER NOT NULL
);
INSERT INTO fence_counter (id, value) VALUES (1, 0);

CREATE TABLE rate_limit_snapshot (
    tenant     TEXT    NOT NULL,
    key        TEXT    NOT NULL,
    algorithm  INTEGER NOT NULL,              -- 1 = gcra, 2 = fixed_window
    a          INTEGER NOT NULL,              -- gcra: tat_ns   | fixed: window_start_ms
    b          INTEGER NOT NULL,              -- gcra: unused 0 | fixed: count
    window_ms  INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    PRIMARY KEY (tenant, key)
) WITHOUT ROWID;
```

`expires_at` for idempotency rows:
- IN_PROGRESS → `lock_expires_at + result_ttl` (generous; the row is cleaned up eventually even if abandoned).
- COMPLETED → `completed_at + result_ttl`.

### 13.3 Claim transaction

Run inside `BEGIN IMMEDIATE` (takes the write lock up front, preventing two claimers from both
reading ABSENT):

1. `SELECT` the row for `(tenant, key)`.
2. Apply §9.1 transition table in Rust.
3. If a new fence is needed: `UPDATE fence_counter SET value = value + 1 RETURNING value`.
4. `INSERT ... ON CONFLICT (tenant, key) DO UPDATE` the new state.
5. `COMMIT`. Only then respond.

SQLite allows one writer at a time, which is what makes this linearizable. Expect a few thousand
claims/sec on an SSD with `synchronous = FULL`; measure it (§19.5).

### 13.4 Startup

1. Run migrations (`sqlx::migrate!`).
2. Load `rate_limit_snapshot` rows with `expires_at > now` into memory.
3. Start background tasks, then start serving.

Rate-limit state lost between the last snapshot and a crash is **acceptable** (worst case: a few
seconds of extra allowance). Idempotency state is never lost once a response is sent.

---

## 14. Configuration

Environment variables, loaded once at startup into a typed `Config` struct. Invalid config →
exit with a clear message. Support a `.env` file in dev (`dotenvy`).

| Variable | Default | Notes |
|---|---|---|
| `GUARD_MODE` | `prod` | `prod` or `dev` (§5.2) |
| `GUARD_BIND_ADDR` | `127.0.0.1:8080` | |
| `GUARD_DB_PATH` | `./guard.db` | |
| `GUARD_RAPIDAPI_PROXY_SECRET` | — | **Required** in prod. Wrap in `secrecy::SecretString`. |
| `GUARD_SNAPSHOT_INTERVAL` | `5s` | |
| `GUARD_SWEEP_INTERVAL` | `10s` | |
| `GUARD_SHARDS` | `64` | Power of two. |
| `GUARD_REQUEST_TIMEOUT` | `2s` | |
| `GUARD_ALLOW_PUBLIC_DEV` | `false` | |
| `RUST_LOG` | `info` | `tracing-subscriber` filter. |

---

## 15. Observability

### 15.1 Logging

- `tracing` with JSON output in prod, pretty output in dev.
- One span per request: `request_id`, `tenant`, `route`, `status`, `latency_us`.
- **Never log raw keys.** Log `key_hash` = first 12 hex chars of SHA-256(tenant + key).
- **Never log `result` payloads or the proxy secret.**

### 15.2 Metrics (`GET /metrics`, Prometheus format)

Protect with a separate bearer token or bind to a private interface.

| Metric | Type | Labels |
|---|---|---|
| `guard_requests_total` | counter | `route`, `status` |
| `guard_request_duration_seconds` | histogram | `route` |
| `guard_limit_decisions_total` | counter | `algorithm`, `allowed` |
| `guard_idem_claims_total` | counter | `outcome` (`acquired`/`in_progress`/`completed`/`takeover`) |
| `guard_rl_active_keys` | gauge | — |
| `guard_idem_active_records` | gauge | — |
| `guard_db_tx_duration_seconds` | histogram | `op` |
| `guard_snapshot_duration_seconds` | histogram | — |

No per-tenant labels (cardinality explosion). Per-tenant usage, if ever needed, goes to logs.

### 15.3 Health

- `GET /healthz` → `200 ok` if the process is up.
- `GET /readyz` → `200` if DB reachable and not shutting down, else `503`.

---

## 16. Failure Modes and Guarantees

### 16.1 Server-side guarantees

| Property | Guarantee |
|---|---|
| Idempotency claim atomicity | Exactly one `acquired` per key per lock period. |
| Idempotency durability | A `200` response means the state survives a crash. |
| Rate-limit accuracy | Exact within a process lifetime; may over-allow by up to one snapshot interval after a crash. |
| Rate-limit availability | Prefer serving over persisting — snapshot failure is logged, not fatal. |

### 16.2 Client-side guidance (documented, implemented in the SDK)

| Situation | Rate limit default | Idempotency default |
|---|---|---|
| Guard timeout / network error / 5xx | **Fail open** — allow | **Fail closed** — throw |
| Recommended client timeout | 150 ms | 1 s |
| Retry | None (latency matters) | Up to 2 retries, jittered backoff |

Both defaults are configurable in the SDK.

### 16.3 Things that can go wrong (and the answer)

| Scenario | Outcome |
|---|---|
| Worker crashes after `claim`, before `complete` | Lock expires after `lock_ttl`; next claim takes over with a new fence. |
| Worker is slow; lock expires; another worker claims | Slow worker's `complete` fails with `lock_lost`. It must not assume success. |
| Worker completes the work but `complete` call fails (network) | Retry `complete` with the same fence. Still valid unless someone took over. |
| Clock jumps backwards on server | Use a monotonic-safe clock: `max(system_now, last_seen_now)`. |
| Disk full | Idempotency → `503 unavailable`. Rate limiting continues in memory. |

---

## 17. Security and Data Handling

- **No personal data by design.** Keys and results are customer-controlled; docs, error hints and
  SDK helpers steer customers to IDs and hashes.
- **Everything expires.** Max result TTL 7d; rate-limit state evicted when idle.
- **Proxy secret** compared with constant-time equality (`subtle` crate).
- **Bind** behind RapidAPI only; firewall the port to RapidAPI egress IPs if feasible.
- **Dependencies:** `cargo audit` and `cargo deny` in CI.
- **Unsafe code:** `#![forbid(unsafe_code)]` in the crate root.
- **Data deletion:** `reset` and `release` delete immediately; a tenant-wide purge is a manual
  admin operation in v1.
- Publish a short **Data Handling** page: what's stored, for how long, where.

---

## 18. Project Structure

```
guard-api/
├── Cargo.toml
├── .env.example
├── migrations/
│   └── 0001_init.sql
├── src/
│   ├── main.rs              # parse config, init tracing, build state, serve, shutdown
│   ├── lib.rs               # pub fn app(state) -> Router  (so tests can build the router)
│   ├── config.rs
│   ├── error.rs             # ApiError + IntoResponse + error envelope
│   ├── clock.rs             # Clock trait, SystemClock, ManualClock (tests)
│   ├── duration.rs          # "30s" <-> Duration parsing + serde impls
│   ├── extract/
│   │   ├── mod.rs
│   │   ├── tenant.rs        # Tenant extractor (auth + plan)
│   │   └── json.rs          # Json wrapper that maps rejections to ApiError
│   ├── routes/
│   │   ├── mod.rs           # Router assembly
│   │   ├── limit.rs
│   │   ├── idempotency.rs
│   │   └── health.rs
│   ├── ratelimit/
│   │   ├── mod.rs           # RateLimiter (sharded map), batch locking
│   │   ├── gcra.rs          # pure functions — no I/O, no clock reads
│   │   ├── fixed_window.rs  # pure functions
│   │   └── snapshot.rs
│   ├── idempotency/
│   │   ├── mod.rs           # IdemStore
│   │   ├── state.rs         # state machine as pure functions over (state, op, now)
│   │   └── sql.rs           # sqlx queries + transaction
│   └── tasks.rs             # sweeper, snapshotter
└── tests/
    ├── limit_api.rs
    ├── idempotency_api.rs
    ├── concurrency.rs
    └── common/mod.rs        # test app builder with ManualClock + temp DB
```

**Design rule:** algorithm and state-machine code (`gcra.rs`, `fixed_window.rs`, `state.rs`) are
**pure functions** taking `now` as a parameter. All I/O lives at the edges. This makes the hard
parts trivially unit-testable.

### Suggested crates

| Purpose | Crate |
|---|---|
| HTTP | `axum`, `tower`, `tower-http` (trace, request-id, limit, timeout) |
| Runtime | `tokio` (full) |
| Serialization | `serde`, `serde_json` |
| Errors | `thiserror` |
| DB | `sqlx` (features: `sqlite`, `runtime-tokio`, `migrate`) |
| Logging | `tracing`, `tracing-subscriber` (json, env-filter) |
| Metrics | `metrics`, `metrics-exporter-prometheus` |
| Config | `dotenvy` |
| Secrets | `secrecy`, `subtle` |
| IDs | `uuid` (v7) |
| Hashing | `sha2`, `ahash` (shard selection) |
| Time | `time` or `chrono` (RFC 3339 formatting only) |
| Testing | `proptest`, `tempfile`, `tower` (`ServiceExt::oneshot`), `http-body-util` |

---

## 19. Testing Strategy

### 19.1 Unit tests (pure functions)

- **GCRA:** every sanity check in §7.1; `cost = 0`; `cost = limit`; very large `window`/`limit`
  combos don't overflow; parameter change on existing key.
- **Fixed window:** boundary alignment; reset on new window; `2 × limit` across boundary
  (assert it, so the behaviour is intentional).
- **Idempotency state machine:** one test per row of the §9.1 table.
- **Duration parsing:** valid/invalid inputs; error messages.
- **Key validation.**

### 19.2 Property tests (`proptest`)

- **GCRA never over-allows:** for any sequence of `(time_advance, cost)` events, the total cost
  allowed in any interval `[t, t + d]` is ≤ `limit + floor(d / T)` (full bucket plus refill
  during the interval). Write it as: replay events, record `(t, cost)` of allowed calls, check
  every interval between pairs of allowed calls.
- **GCRA eventually allows:** after waiting `retry_after_ms`, the same call is allowed.

### 19.3 Integration tests (in-process)

Build the router with `ManualClock` + temp SQLite and call it with `tower::ServiceExt::oneshot`.
No network needed. Cover every endpoint and every error code in §11.

### 19.4 Concurrency tests

- **Claim race:** 200 concurrent `claim`s on one key → exactly one `acquired`, 199 `in_progress`.
- **Limit race:** 1,000 concurrent `cost=1` calls with `limit=100` → exactly 100 allowed.
- **Batch deadlock:** concurrent batches with overlapping keys in different orders complete
  within a timeout.
- **Tenant isolation:** same key under two tenants is independent.

### 19.5 Load and durability (manual / CI optional)

- Load test with `oha` or `k6`: confirm G3 and G4 on target VPS size. Record results in the README.
- Crash test: `kill -9` during a claim storm; after restart, every `acquired` that received a
  response is still present.

---

## 20. Milestones

Each milestone ends with tests passing and something runnable.

| # | Milestone | Done when | Concepts exercised |
|---|---|---|---|
| M0 | **Skeleton** | `cargo run` serves `/healthz`; config loads; tracing logs requests; `ApiError` envelope returned for unknown routes. | modules, axum basics, thiserror, `IntoResponse` |
| M1 | **GCRA + fixed window (pure)** | All §19.1 rate-limit unit tests and §19.2 property tests pass. No HTTP. | generics, traits (`Clock`), integer math, testing |
| M2 | **Rate-limit endpoints, in memory** | §6.1–6.3 work in dev mode; integration tests pass; limit race test passes. | `Arc<Mutex<_>>`, sharding, extractors, serde |
| M3 | **Tenancy + RapidAPI auth** | `Tenant` extractor; dev/prod modes; isolation test passes. | custom extractors, `secrecy`, constant-time compare |
| M4 | **Idempotency on SQLite** | §8 endpoints; §9.1 table tests; claim race test passes. | sqlx, transactions, migrations |
| M5 | **Background tasks** | Sweeper + snapshotter; restart restores rate-limit state; graceful shutdown. | `tokio::spawn`, `interval`, shutdown signals |
| M6 | **Batch + hard limits** | §6.4 and §10.2; deadlock test passes. | lock ordering |
| M7 | **Observability** | `/metrics`, `/readyz`, JSON logs with key hashing. | tracing, metrics |
| M8 | **Deploy** | Running on a VPS behind TLS (Caddy); load test results recorded; listed on RapidAPI (private). | ops |
| M9 | **TypeScript SDK** | `limit()`, `once()`, published to npm with README examples. | — |
| M10 | **Public launch** | Docs, OpenAPI spec, `llms.txt`, status page, RapidAPI listing public. | — |

**Course gaps to fill before/during:** modules & project structure (M0), axum basics (M0),
errors as HTTP responses (M0), testing (M1), closures in depth (M2), config & tracing (M0/M7).

---

## 21. Client SDK (TypeScript)

Thin wrapper; the product's selling point is these two functions.

```ts
import { Guard } from "@guard/sdk";

const guard = new Guard({ rapidApiKey: process.env.RAPIDAPI_KEY! });

// Rate limiting — returns the §6.1 response
const r = await guard.limit(`user:${userId}:ai`, { limit: 50_000, window: "1d", cost: tokens });
if (!r.allowed) return new Response("Daily AI limit reached", { status: 429 });

// Idempotency — runs fn at most once per key
const result = await guard.once(`stripe:${event.id}`, async () => {
  await fulfillOrder(event);
  return { ok: true };
}, { lockTtl: "30s", resultTtl: "24h" });
```

### `once(key, fn, opts)` algorithm

1. `claim` →
   - `completed` → return stored `result`.
   - `in_progress` → wait `retry_after_ms` (capped by `opts.maxWait`, default 10s), then claim
     again; if still in progress, throw `GuardInProgressError`.
   - `acquired` → continue.
2. If `fn` may exceed `lockTtl`, start a heartbeat calling `extend` every `lockTtl / 3`.
3. Run `fn`.
   - Success → `complete(key, fence, result)`; return `result`.
     If `complete` returns `lock_lost`, throw `GuardLockLostError` (the caller's work ran but
     another executor may also have run — surface it loudly).
   - Throws → `release(key, fence)`; rethrow the original error.
4. Stop heartbeat.

### Helpers

- `guard.hashKey(value)` → `sha256` hex, for keys derived from personal data.
- `guard.requestHash(obj)` → stable hash of a JSON object (sorted keys), for `request_hash`.

---

## 22. RapidAPI Listing and Pricing

| Plan | Price | Requests / month | Notes |
|---|---|---|---|
| Basic (Free) | $0 | 10,000 | Hard limit |
| Hobby | $5 | 500,000 | Overage ~$1 / extra 1M |
| Pro | $15 | 5,000,000 | Overage ~$1 / extra 1M |

Listing must include: one-paragraph pitch, the two SDK snippets above, curl examples for every
endpoint, error catalogue link, Data Handling page link, and the OpenAPI spec.

---

## 23. Future Work

- Direct API keys (bypass RapidAPI latency), Stripe billing.
- MCP server exposing `limit` and `once`-style tools to AI agents.
- Multi-region with per-region limit shares.
- Python SDK.
- Webhook signature verification helpers (Stripe, GitHub) bundled with `once`.
- `sliding_log` algorithm for strict "at most N in any window-length span" guarantees
  (stores one timestamp per allowed unit — cap `limit` low, e.g. ≤ 1,000).
- Concurrency limiter ("max 3 simultaneous jobs per user") — semaphore semantics with leases.
- Read-only dashboard per tenant.

---

## 24. Open Questions

1. **Latency through RapidAPI:** measure the added round-trip before committing to RapidAPI-only.
   If it's > 50 ms, prioritise direct API keys.
2. **`synchronous = FULL` vs `NORMAL`:** benchmark claim throughput; decide based on numbers.
3. **Hosting region:** `us-east-1`-adjacent (many Vercel functions) vs EU (closer to you). Measure.
4. **Fence exposure:** is a 64-bit sequential fence guessable enough to matter, given it's
   tenant-scoped and key-scoped? Consider returning an opaque token (fence + HMAC) instead.
5. **Name:** "Guard" is a placeholder — check npm/RapidAPI availability.
