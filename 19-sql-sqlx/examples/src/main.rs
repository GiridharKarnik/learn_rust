use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use sqlx::Row;

// =========================================================================
// 1. 🚂 Connecting to SQLite (In-Memory)
// =========================================================================

async fn connect_demo() -> Result<SqlitePool, sqlx::Error> {
    println!("=== 1. 🚂 Connecting to SQLite ===\n");

    // In-memory database — no file created, perfect for demos
    let pool = SqlitePool::connect(":memory:").await?;
    println!("Connected to in-memory SQLite database!");
    println!("Pool size: {} connections\n", pool.size());

    Ok(pool)
}

// =========================================================================
// 2. 📦 Creating Tables
// =========================================================================

async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("=== 2. 📦 Creating Tables ===\n");

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
            seat_number INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'confirmed',
            FOREIGN KEY (train_id) REFERENCES trains(id)
        )",
    )
    .execute(pool)
    .await?;

    println!("Tables 'trains' and 'reservations' created!");
    Ok(())
}

// =========================================================================
// 3. ➕ Inserting Data with .bind()
// =========================================================================

async fn insert_demo(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("\n=== 3. ➕ Inserting Data ===\n");

    // Insert individual trains
    let trains = vec![
        ("Rajdhani Express", "Chennai", "New Delhi", 500),
        ("Shatabdi Express", "Chennai", "Bangalore", 300),
        ("Duronto Express", "Chennai", "Mumbai", 400),
        ("Tejas Express", "Chennai", "Madurai", 250),
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
        .bind(seats) // initially all seats available
        .execute(pool)
        .await?;

        println!(
            "  Inserted '{}' (id: {})",
            name,
            result.last_insert_rowid()
        );
    }

    println!("\nInserted {} trains", trains.len());
    Ok(())
}

// =========================================================================
// 4. 🔍 Querying — Dynamic (row.get)
// =========================================================================

async fn query_dynamic(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("\n=== 4. 🔍 Dynamic Queries (row.get) ===\n");

    let rows = sqlx::query("SELECT id, name, total_seats FROM trains ORDER BY id")
        .fetch_all(pool)
        .await?;

    println!("All trains ({} rows):", rows.len());
    for row in &rows {
        let id: i64 = row.get("id");
        let name: String = row.get("name");
        let seats: i64 = row.get("total_seats");
        println!("  #{}: {} ({} seats)", id, name, seats);
    }

    Ok(())
}

// =========================================================================
// 5. 🎯 Querying — With Structs (query_as + FromRow)
// =========================================================================

#[derive(Debug, FromRow)]
#[allow(dead_code)]
struct Train {
    id: i64,
    name: String,
    from_station: String,
    to_station: String,
    total_seats: i64,
    available_seats: i64,
}

async fn query_with_structs(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("\n=== 5. 🎯 Typed Queries (query_as + FromRow) ===\n");

    // Fetch all trains as Train structs
    let trains: Vec<Train> = sqlx::query_as::<_, Train>(
        "SELECT id, name, from_station, to_station, total_seats, available_seats
         FROM trains ORDER BY id",
    )
    .fetch_all(pool)
    .await?;

    for train in &trains {
        println!(
            "  {} → {} to {} ({}/{} seats available)",
            train.name, train.from_station, train.to_station,
            train.available_seats, train.total_seats
        );
    }

    // Fetch one specific train
    println!("\nLooking up train #2:");
    let train = sqlx::query_as::<_, Train>(
        "SELECT id, name, from_station, to_station, total_seats, available_seats
         FROM trains WHERE id = ?",
    )
    .bind(2)
    .fetch_one(pool)
    .await?;
    println!("  Found: {:?}", train);

    // Fetch optional — might not exist
    println!("\nLooking up train #999:");
    let maybe_train = sqlx::query_as::<_, Train>(
        "SELECT id, name, from_station, to_station, total_seats, available_seats
         FROM trains WHERE id = ?",
    )
    .bind(999)
    .fetch_optional(pool)
    .await?;

    match maybe_train {
        Some(t) => println!("  Found: {}", t.name),
        None => println!("  Not found (returned None)"),
    }

    Ok(())
}

// =========================================================================
// 6. ✏️ Updates and Deletes
// =========================================================================

async fn update_delete_demo(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("\n=== 6. ✏️ Updates and Deletes ===\n");

    // Update available seats
    let result = sqlx::query("UPDATE trains SET available_seats = ? WHERE id = ?")
        .bind(495)
        .bind(1)
        .execute(pool)
        .await?;
    println!("Updated {} row(s) — Rajdhani now has 495 available seats", result.rows_affected());

    // Update non-existent row — no error, just 0 rows affected
    let result = sqlx::query("UPDATE trains SET available_seats = ? WHERE id = ?")
        .bind(0)
        .bind(999)
        .execute(pool)
        .await?;
    println!("Updated {} row(s) for non-existent train (no error!)", result.rows_affected());

    // Delete — we'll add a dummy train and delete it
    sqlx::query(
        "INSERT INTO trains (name, from_station, to_station, total_seats, available_seats)
         VALUES ('Temp Express', 'A', 'B', 100, 100)",
    )
    .execute(pool)
    .await?;

    let result = sqlx::query("DELETE FROM trains WHERE name = 'Temp Express'")
        .execute(pool)
        .await?;
    println!("Deleted {} row(s)", result.rows_affected());

    Ok(())
}

// =========================================================================
// 7. 💳 Transactions
// =========================================================================

async fn transaction_demo(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("\n=== 7. 💳 Transactions ===\n");

    // Book a ticket: insert reservation + decrement available seats (atomically)
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO reservations (passenger_name, train_id, seat_number, status)
         VALUES (?, ?, ?, 'confirmed')",
    )
    .bind("Amit Kumar")
    .bind(1_i64)
    .bind(1_i64)
    .execute(&mut *tx)
    .await?;

    sqlx::query("UPDATE trains SET available_seats = available_seats - 1 WHERE id = ?")
        .bind(1_i64)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    println!("Transaction committed: Amit booked on Rajdhani Express");

    // Verify the changes
    let train = sqlx::query_as::<_, Train>(
        "SELECT id, name, from_station, to_station, total_seats, available_seats
         FROM trains WHERE id = ?",
    )
    .bind(1)
    .fetch_one(pool)
    .await?;
    println!(
        "  {} now has {}/{} seats available",
        train.name, train.available_seats, train.total_seats
    );

    // Demonstrate rollback
    println!("\nAttempting a transaction that we'll rollback...");
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO reservations (passenger_name, train_id, seat_number, status)
         VALUES (?, ?, ?, 'confirmed')",
    )
    .bind("Ghost Passenger")
    .bind(1_i64)
    .bind(999_i64)
    .execute(&mut *tx)
    .await?;

    // Rollback — Ghost Passenger reservation is discarded
    tx.rollback().await?;
    println!("Transaction rolled back: Ghost Passenger reservation discarded");

    // Verify ghost passenger doesn't exist
    let reservations = sqlx::query("SELECT * FROM reservations WHERE passenger_name = 'Ghost Passenger'")
        .fetch_all(pool)
        .await?;
    println!("  Ghost Passenger reservations: {} (should be 0)", reservations.len());

    Ok(())
}

// =========================================================================
// 8. 📊 Aggregations and Joins
// =========================================================================

async fn aggregation_demo(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("\n=== 8. 📊 Aggregations and Joins ===\n");

    // Add a few more reservations for variety
    let bookings = vec![
        ("Priya Sharma", 2_i64, 1_i64),
        ("Raj Patel", 1_i64, 2_i64),
        ("Sita Devi", 3_i64, 1_i64),
    ];

    for (name, train_id, seat) in &bookings {
        sqlx::query(
            "INSERT INTO reservations (passenger_name, train_id, seat_number, status)
             VALUES (?, ?, ?, 'confirmed')",
        )
        .bind(name)
        .bind(train_id)
        .bind(seat)
        .execute(pool)
        .await?;
    }

    // Count reservations per train using JOIN
    let rows = sqlx::query(
        "SELECT t.name, COUNT(r.id) as booking_count
         FROM trains t
         LEFT JOIN reservations r ON t.id = r.train_id
         GROUP BY t.id
         ORDER BY booking_count DESC",
    )
    .fetch_all(pool)
    .await?;

    println!("Reservations per train:");
    for row in &rows {
        let name: String = row.get("name");
        let count: i64 = row.get("booking_count");
        println!("  {} — {} reservation(s)", name, count);
    }

    // Total stats
    let row = sqlx::query(
        "SELECT COUNT(*) as total, COUNT(DISTINCT train_id) as trains_booked
         FROM reservations",
    )
    .fetch_one(pool)
    .await?;

    let total: i64 = row.get("total");
    let trains_booked: i64 = row.get("trains_booked");
    println!(
        "\nTotal: {} reservations across {} trains",
        total, trains_booked
    );

    Ok(())
}

// =========================================================================
// Main
// =========================================================================

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = connect_demo().await?;
    create_tables(&pool).await?;
    insert_demo(&pool).await?;
    query_dynamic(&pool).await?;
    query_with_structs(&pool).await?;
    update_delete_demo(&pool).await?;
    transaction_demo(&pool).await?;
    aggregation_demo(&pool).await?;

    println!("\n🚂 All sqlx examples complete!");
    Ok(())
}
