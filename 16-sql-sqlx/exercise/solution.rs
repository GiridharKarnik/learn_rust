// Lesson 16: Compile-Time SQL with sqlx — Railway Reservation Database (SOLUTION)

use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use sqlx::Row;

// ---------------------------------------------------------------------------
// STEP 1: Define structs
// ---------------------------------------------------------------------------

#[derive(Debug, FromRow)]
struct Train {
    id: i64,
    name: String,
    from_station: String,
    to_station: String,
    total_seats: i64,
    available_seats: i64,
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
struct Reservation {
    id: i64,
    passenger_name: String,
    train_id: i64,
    status: String,
}

#[derive(Debug, FromRow)]
struct ReservationDetail {
    id: i64,
    passenger_name: String,
    train_name: String,
    status: String,
}

// ---------------------------------------------------------------------------
// STEP 2: create_tables
// ---------------------------------------------------------------------------

async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS trains (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            from_station TEXT NOT NULL,
            to_station TEXT NOT NULL,
            total_seats INTEGER NOT NULL,
            available_seats INTEGER NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS reservations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            passenger_name TEXT NOT NULL,
            train_id INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'confirmed',
            FOREIGN KEY (train_id) REFERENCES trains(id)
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// STEP 3: seed_trains
// ---------------------------------------------------------------------------

async fn seed_trains(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let trains = vec![
        ("Rajdhani Express", "Chennai", "New Delhi", 500),
        ("Shatabdi Express", "Chennai", "Bangalore", 300),
        ("Duronto Express", "Chennai", "Mumbai", 400),
    ];

    for (name, from, to, seats) in &trains {
        let result = sqlx::query(
            "INSERT INTO trains (name, from_station, to_station, total_seats, available_seats)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(name)
        .bind(from)
        .bind(to)
        .bind(seats)
        .bind(seats)
        .execute(pool)
        .await?;

        println!(
            "  #{}: {} ({} → {}) — {} seats",
            result.last_insert_rowid(),
            name,
            from,
            to,
            seats
        );
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// STEP 4: list_trains
// ---------------------------------------------------------------------------

async fn list_trains(pool: &SqlitePool) -> Result<Vec<Train>, sqlx::Error> {
    let trains = sqlx::query_as::<_, Train>(
        "SELECT id, name, from_station, to_station, total_seats, available_seats
         FROM trains ORDER BY id",
    )
    .fetch_all(pool)
    .await?;

    Ok(trains)
}

// ---------------------------------------------------------------------------
// STEP 5: make_reservation
// ---------------------------------------------------------------------------

async fn make_reservation(
    pool: &SqlitePool,
    passenger: &str,
    train_id: i64,
) -> Result<i64, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    // Check if train exists and has seats
    let row = sqlx::query("SELECT available_seats FROM trains WHERE id = ?")
        .bind(train_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    let row = match row {
        Some(r) => r,
        None => return Err("Train not found".to_string()),
    };

    let available: i64 = row.get("available_seats");
    if available <= 0 {
        return Err("No seats available".to_string());
    }

    // Insert reservation
    let result = sqlx::query(
        "INSERT INTO reservations (passenger_name, train_id, status) VALUES (?, ?, 'confirmed')",
    )
    .bind(passenger)
    .bind(train_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let reservation_id = result.last_insert_rowid();

    // Decrement available seats
    sqlx::query("UPDATE trains SET available_seats = available_seats - 1 WHERE id = ?")
        .bind(train_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(reservation_id)
}

// ---------------------------------------------------------------------------
// STEP 6: cancel_reservation
// ---------------------------------------------------------------------------

async fn cancel_reservation(pool: &SqlitePool, id: i64) -> Result<(), String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    // Find the reservation
    let row = sqlx::query("SELECT id, train_id, status FROM reservations WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    let row = match row {
        Some(r) => r,
        None => return Err("Reservation not found".to_string()),
    };

    let status: String = row.get("status");
    if status == "cancelled" {
        return Err("Already cancelled".to_string());
    }

    let train_id: i64 = row.get("train_id");

    // Update reservation status
    sqlx::query("UPDATE reservations SET status = 'cancelled' WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // Restore the seat
    sqlx::query("UPDATE trains SET available_seats = available_seats + 1 WHERE id = ?")
        .bind(train_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(())
}

// ---------------------------------------------------------------------------
// STEP 7: list_reservations
// ---------------------------------------------------------------------------

async fn list_reservations(pool: &SqlitePool) -> Result<Vec<ReservationDetail>, sqlx::Error> {
    let reservations = sqlx::query_as::<_, ReservationDetail>(
        "SELECT r.id, r.passenger_name, t.name as train_name, r.status
         FROM reservations r
         JOIN trains t ON r.train_id = t.id
         ORDER BY r.id",
    )
    .fetch_all(pool)
    .await?;

    Ok(reservations)
}

// ---------------------------------------------------------------------------
// STEP 8: main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RAILWAY RESERVATION DATABASE ===");

    // Connect to in-memory SQLite
    let pool = SqlitePool::connect(":memory:").await?;

    // Create tables
    create_tables(&pool).await?;
    println!("\n--- Tables created ---");

    // Seed trains
    println!("\n--- Trains seeded ---");
    seed_trains(&pool).await?;

    // List trains
    println!("\n--- Available trains ---");
    let trains = list_trains(&pool).await?;
    for train in &trains {
        println!(
            "  #{}: {} | {} → {} | {}/{} seats",
            train.id, train.name, train.from_station, train.to_station,
            train.available_seats, train.total_seats
        );
    }

    // Make reservations
    println!("\n--- Making reservations ---");
    let bookings = vec![("Amit", 1_i64), ("Priya", 1), ("Raj", 2), ("Sita", 3)];
    for (passenger, train_id) in &bookings {
        match make_reservation(&pool, passenger, *train_id).await {
            Ok(id) => println!(
                "  ✅ Reservation #{}: {} on train #{}",
                id, passenger, train_id
            ),
            Err(e) => println!("  ❌ Failed for {}: {}", passenger, e),
        }
    }

    // List reservations
    println!("\n--- All reservations ---");
    let reservations = list_reservations(&pool).await?;
    for r in &reservations {
        println!(
            "  [#{}] {} on {} — {}",
            r.id, r.passenger_name, r.train_name, r.status
        );
    }

    // Cancel reservation #2
    println!("\n--- Cancelling reservation #2 ---");
    match cancel_reservation(&pool, 2).await {
        Ok(()) => println!("  ✅ Reservation #2 cancelled"),
        Err(e) => println!("  ❌ Failed: {}", e),
    }

    // List reservations after cancellation
    println!("\n--- Reservations after cancellation ---");
    let reservations = list_reservations(&pool).await?;
    for r in &reservations {
        println!(
            "  [#{}] {} on {} — {}",
            r.id, r.passenger_name, r.train_name, r.status
        );
    }

    // Show seat availability
    println!("\n--- Seat availability after operations ---");
    let trains = list_trains(&pool).await?;
    for train in &trains {
        let row = sqlx::query(
            "SELECT COUNT(*) as count FROM reservations WHERE train_id = ? AND status = 'confirmed'",
        )
        .bind(train.id)
        .fetch_one(&pool)
        .await?;
        let active: i64 = row.get("count");

        println!(
            "  {}: {}/{} seats ({} active reservation(s))",
            train.name, train.available_seats, train.total_seats, active
        );
    }

    Ok(())
}
