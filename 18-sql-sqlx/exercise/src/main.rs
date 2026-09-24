// =============================================
// EXERCISE 16: Railway Reservation Database
// =============================================
//
// Build a reservation system backed by SQLite (in-memory).
// There is NO code provided — you write everything, including main().
//
// This exercises: sqlx, SqlitePool, FromRow, query, query_as, execute,
// bind, fetch_all, fetch_optional, transactions, async/await.
//
// Run with: cargo run
//
// Expected output:
//
//   === RAILWAY RESERVATION DATABASE ===
//
//   --- Tables created ---
//
//   --- Trains seeded ---
//     #1: Rajdhani Express (Chennai → New Delhi) — 500 seats
//     #2: Shatabdi Express (Chennai → Bangalore) — 300 seats
//     #3: Duronto Express (Chennai → Mumbai) — 400 seats
//
//   --- Available trains ---
//     #1: Rajdhani Express | Chennai → New Delhi | 500/500 seats
//     #2: Shatabdi Express | Chennai → Bangalore | 300/300 seats
//     #3: Duronto Express | Chennai → Mumbai | 400/400 seats
//
//   --- Making reservations ---
//     ✅ Reservation #1: Amit on train #1
//     ✅ Reservation #2: Priya on train #1
//     ✅ Reservation #3: Raj on train #2
//     ✅ Reservation #4: Sita on train #3
//
//   --- All reservations ---
//     [#1] Amit on Rajdhani Express — confirmed
//     [#2] Priya on Rajdhani Express — confirmed
//     [#3] Raj on Shatabdi Express — confirmed
//     [#4] Sita on Duronto Express — confirmed
//
//   --- Cancelling reservation #2 ---
//     ✅ Reservation #2 cancelled
//
//   --- Reservations after cancellation ---
//     [#1] Amit on Rajdhani Express — confirmed
//     [#2] Priya on Rajdhani Express — cancelled
//     [#3] Raj on Shatabdi Express — confirmed
//     [#4] Sita on Duronto Express — confirmed
//
//   --- Seat availability after operations ---
//     Rajdhani Express: 499/500 seats (1 active reservation)
//     Shatabdi Express: 299/300 seats (1 active reservation)
//     Duronto Express: 399/400 seats (1 active reservation)

// =============================================
// STEP 1: Define `Train` and `Reservation` structs
// =============================================
//
// #[derive(Debug, FromRow)]
// struct Train {
//     id: i64,
//     name: String,
//     from_station: String,
//     to_station: String,
//     total_seats: i64,
//     available_seats: i64,
// }
//
// #[derive(Debug, FromRow)]
// struct Reservation {
//     id: i64,
//     passenger_name: String,
//     train_id: i64,
//     status: String,
// }
//
// You'll also need a struct for the joined reservation data:
//
// #[derive(Debug, FromRow)]
// struct ReservationDetail {
//     id: i64,
//     passenger_name: String,
//     train_name: String,
//     status: String,
// }
//
// Don't forget: use sqlx::FromRow;

// =============================================
// STEP 2: Implement `create_tables`
// =============================================
//
// async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error>
//
// Create two tables:
//
// trains:
//   id INTEGER PRIMARY KEY AUTOINCREMENT
//   name TEXT NOT NULL
//   from_station TEXT NOT NULL
//   to_station TEXT NOT NULL
//   total_seats INTEGER NOT NULL
//   available_seats INTEGER NOT NULL
//
// reservations:
//   id INTEGER PRIMARY KEY AUTOINCREMENT
//   passenger_name TEXT NOT NULL
//   train_id INTEGER NOT NULL
//   status TEXT NOT NULL DEFAULT 'confirmed'
//   FOREIGN KEY (train_id) REFERENCES trains(id)
//
// Use sqlx::query("CREATE TABLE IF NOT EXISTS ...").execute(pool).await?;

// =============================================
// STEP 3: Implement `seed_trains`
// =============================================
//
// async fn seed_trains(pool: &SqlitePool) -> Result<(), sqlx::Error>
//
// Insert 3 trains:
//   ("Rajdhani Express", "Chennai", "New Delhi", 500)
//   ("Shatabdi Express", "Chennai", "Bangalore", 300)
//   ("Duronto Express",  "Chennai", "Mumbai",    400)
//
// For each, INSERT INTO trains (name, from_station, to_station, total_seats, available_seats)
// Set available_seats = total_seats initially.
//
// After inserting, print each train:
//   "  #{id}: {name} ({from} → {to}) — {seats} seats"

// =============================================
// STEP 4: Implement `list_trains`
// =============================================
//
// async fn list_trains(pool: &SqlitePool) -> Result<Vec<Train>, sqlx::Error>
//
// SELECT all columns FROM trains ORDER BY id.
// Use sqlx::query_as::<_, Train>(...).fetch_all(pool).await

// =============================================
// STEP 5: Implement `make_reservation`
// =============================================
//
// async fn make_reservation(
//     pool: &SqlitePool,
//     passenger: &str,
//     train_id: i64,
// ) -> Result<i64, String>
//
// Use a TRANSACTION:
// 1. pool.begin().await
// 2. Check if the train exists and has available seats:
//    SELECT available_seats FROM trains WHERE id = ?
//    Use fetch_optional. If None → return Err("Train not found")
//    If available_seats <= 0 → return Err("No seats available")
// 3. INSERT INTO reservations (passenger_name, train_id, status) VALUES (?, ?, 'confirmed')
// 4. UPDATE trains SET available_seats = available_seats - 1 WHERE id = ?
// 5. tx.commit().await
// 6. Return Ok(reservation_id)
//
// Map sqlx errors to String with .map_err(|e| e.to_string())

// =============================================
// STEP 6: Implement `cancel_reservation`
// =============================================
//
// async fn cancel_reservation(pool: &SqlitePool, id: i64) -> Result<(), String>
//
// Use a TRANSACTION:
// 1. Find the reservation: SELECT id, train_id, status FROM reservations WHERE id = ?
//    If not found → return Err("Reservation not found")
//    If status is already "cancelled" → return Err("Already cancelled")
// 2. UPDATE reservations SET status = 'cancelled' WHERE id = ?
// 3. UPDATE trains SET available_seats = available_seats + 1 WHERE id = ?
// 4. tx.commit().await
// 5. Return Ok(())
//
// Note: when fetching the reservation in the transaction, you'll need to use
// sqlx::query("SELECT ...").bind(id).fetch_optional(&mut *tx) and row.get()
// to extract train_id and status, since we're inside a transaction.

// =============================================
// STEP 7: Implement `list_reservations`
// =============================================
//
// async fn list_reservations(pool: &SqlitePool) -> Result<Vec<ReservationDetail>, sqlx::Error>
//
// Use a JOIN to get the train name:
//   SELECT r.id, r.passenger_name, t.name as train_name, r.status
//   FROM reservations r
//   JOIN trains t ON r.train_id = t.id
//   ORDER BY r.id
//
// Use sqlx::query_as::<_, ReservationDetail>(...).fetch_all(pool).await

// =============================================
// STEP 8: Write main()
// =============================================
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//
// 1. Print "=== RAILWAY RESERVATION DATABASE ==="
//
// 2. Connect: SqlitePool::connect(":memory:").await?
//
// 3. Create tables, print "\n--- Tables created ---"
//
// 4. Seed trains, print "\n--- Trains seeded ---"
//
// 5. List trains, print "\n--- Available trains ---"
//    For each: "  #{id}: {name} | {from} → {to} | {available}/{total} seats"
//
// 6. Make reservations, print "\n--- Making reservations ---"
//    Book: ("Amit", 1), ("Priya", 1), ("Raj", 2), ("Sita", 3)
//    For each success: "  ✅ Reservation #{id}: {passenger} on train #{train_id}"
//
// 7. List reservations, print "\n--- All reservations ---"
//    For each: "  [#{id}] {passenger} on {train_name} — {status}"
//
// 8. Cancel reservation #2, print "\n--- Cancelling reservation #2 ---"
//    On success: "  ✅ Reservation #2 cancelled"
//
// 9. List reservations again, print "\n--- Reservations after cancellation ---"
//
// 10. Show final seat availability, print "\n--- Seat availability after operations ---"
//     List trains and for each:
//     "  {name}: {available}/{total} seats ({active} active reservation(s))"
//     Count active reservations with a query:
//       SELECT COUNT(*) as count FROM reservations WHERE train_id = ? AND status = 'confirmed'
//
// }

#[tokio::main]
async fn main() {
    // Your code here
}
