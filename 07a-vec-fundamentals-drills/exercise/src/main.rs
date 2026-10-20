// =============================================
// EXERCISE 07a: Vec Fundamentals Drills
// =============================================
//
// There is NO code provided - you write everything, including main().
// This exercises: push/pop/insert/remove, get vs [], capacity, iter/iter_mut/into_iter, Vec<Vec<T>>, windows/chunks, borrow trap.
//
// Run with: cargo run
//
// Expected output (match it exactly, including the two-space indents):
//
//   === VEC FUNDAMENTALS DRILLS ===
//
//   --- Stations ---
//     len=5 empty=false
//     station 2: Katpadi
//     station 9: out of range
//     route: Chennai -> Coimbatore
//     route: none
//
//   --- Boarding Queue ---
//     len=0 empty=true
//     queue: ["Dr. Devi", "Anand", "Bala", "Chitra"]
//     departed: Some("Chitra")
//     cancelled: Some("Anand")
//     cancelled: None
//     queue: ["Dr. Devi", "Bala"]
//     departed: None
//
//   --- Capacity ---
//     len=3 reserved=true still_reserved=true
//
//   --- Three Ways to Iterate ---
//     sum: 370
//     boosted: [110, 132, 165]
//     sum: 407
//     consumed: Chennai+Arakkonam+Katpadi+Salem+Coimbatore
//     still have 5 stations
//
//   --- Searching ---
//     Salem is at index 3
//     Madurai not on this route
//
//   --- Seat Grid ---
//     booked (0,0)
//     booked (1,2)
//     booked (2,3)
//     error: seat (1,2) already booked
//     error: seat (3,0) does not exist
//     error: seat (0,9) does not exist
//     X...
//     ..X.
//     ...X
//
//   --- Slices ---
//     max gap: Some(60)
//     max gap (one stop): None
//     chunk sums: [6, 15, 7]
//
//   --- Borrow Trap ---
//     first was Chennai, route now has 6 stops
//     last: Some("Madurai")

// =============================================
// STEP 1: `build_stations() -> Vec<String>`
// =============================================
// Return a Vec<String> of: Chennai, Arakkonam, Katpadi, Salem, Coimbatore
// (in that order). Use vec![] and .to_string().

// =============================================
// STEP 2: `describe_len(items: &[String]) -> String`
// =============================================
// Returns "len={len} empty={is_empty}"   e.g. "len=5 empty=false"
// Take a SLICE, not &Vec - then &Vec<String> still works via deref coercion.

// =============================================
// STEP 3: `safe_get(items: &[String], index: usize) -> String`
// =============================================
// Use .get(index) and match. NEVER index with [] here.
//   Some -> "station {index}: {name}"
//   None -> "station {index}: out of range"

// =============================================
// STEP 4: `first_and_last(items: &[String]) -> Option<(&str, &str)>`
// =============================================
// Return None for an empty slice, else the first and last names as &str.
// Hint: .first()? and .last()? with the ? operator on Option,
//       then .as_str() to turn &String into &str.
// Note: the returned &str values borrow from `items` (lifetime elision does it).
// In main, test the empty case with `first_and_last(&[])`.

// =============================================
// STEP 5: `board` and `depart` (queue = Vec<String>)
// =============================================
// fn board(queue: &mut Vec<String>, name: &str)        -> push to the back
// fn depart(queue: &mut Vec<String>) -> Option<String> -> pop from the back
// (&mut Vec because we push, which may reallocate. A &mut [T] could not push.)

// =============================================
// STEP 6: `priority_board` and `cancel`
// =============================================
// fn priority_board(queue: &mut Vec<String>, name: &str) -> insert at index 0
// fn cancel(queue: &mut Vec<String>, index: usize) -> Option<String>
//   remove(index) PANICS when out of bounds, so check `index < queue.len()`
//   first and return None otherwise.

// =============================================
// STEP 7: `capacity_demo() -> String`
// =============================================
// Create Vec<u32> with_capacity(10). Record `before = capacity() >= 10`.
// Push 1, 2, 3. Record `after = capacity() >= 10`.
// Return "len={len} reserved={before} still_reserved={after}"
// (Never print exact capacity() values - they are an implementation detail.
//  Comparing with >= is the portable way.)

// =============================================
// STEP 8: the three iteration modes
// =============================================
// fn sum_speeds(speeds: &[u32]) -> u32
//     for s in speeds  (s is &u32; `total += s` works)
// fn boost(speeds: &mut [u32], percent: u32)
//     iter_mut(); each speed grows by `speed * percent / 100` (integer math,
//     use *s to deref)
// fn consume_names(names: Vec<String>) -> String
//     takes OWNERSHIP, uses into_iter().enumerate(); joins names with '+'
//     (no leading '+'). Build with String::new(), push(), push_str().

// =============================================
// STEP 9: `find_index(items: &[String], name: &str) -> Option<usize>`
// =============================================
// Hint: items.iter().position(|s| s == name)   (&String == &str? Try `s == name`
// where s: &String and name: &str - if the compiler complains, think about
// which side needs a deref or as_str().)

// =============================================
// STEP 10: Vec<Vec<bool>> seat grid
// =============================================
// fn make_grid(rows: usize, cols: usize) -> Vec<Vec<bool>>
//     vec![vec![false; cols]; rows]   (the inner Vec is cloned per row)
// fn book_seat(grid: &mut [Vec<bool>], row: usize, col: usize) -> Result<(), String>
//     - nonexistent seat -> Err("seat ({row},{col}) does not exist")
//     - already true     -> Err("seat ({row},{col}) already booked")
//     - otherwise set true, Ok(())
//     Hint: grid.get_mut(row).and_then(|r| r.get_mut(col)) gives Option<&mut bool>
//     with no panics. Turn None into Err with .ok_or(...)? .
// fn render_grid(grid: &[Vec<bool>]) -> Vec<String>
//     one String per row, 'X' for booked, '.' for free.

// =============================================
// STEP 11: windows and chunks
// =============================================
// fn max_gap(times: &[u32]) -> Option<u32>
//     largest difference between consecutive entries. windows(2), then max().
//     Fewer than 2 entries -> None (windows gives nothing, max gives None).
// fn chunk_sums(values: &[u32], size: usize) -> Vec<u32>
//     sum of each chunks(size) piece; the last chunk may be shorter.

// =============================================
// STEP 12: the borrow trap
// =============================================
// fn first_then_push(items: &mut Vec<String>, extra: &str) -> String
//     Return a CLONE of items[0], and push `extra` onto items.
//     Try first writing `let first = &items[0]; items.push(...); first...`
//     and read the error. Then fix it by cloning BEFORE pushing.
//     (push may reallocate, which would leave `first` dangling.)

// =============================================
// STEP 13: Write `fn main()`
// =============================================
// Print each section exactly as in the expected output. Section headers are
// "--- Name ---", with a blank line (println!("\n--- ...")) before all but the
// first. Details:
//
// Stations:   build_stations(); describe_len; safe_get(2) and safe_get(9);
//             first_and_last via match -> "  route: A -> B" or "  route: none"
//             (do the empty-slice case too).
// Boarding Queue: start with Vec::new(); print describe_len; board Anand, Bala,
//             Chitra; priority_board "Dr. Devi"; print "  queue: {queue:?}";
//             print depart/cancel results with {:?}: depart, cancel(1),
//             cancel(7); print the queue; depart twice silently, then print
//             the third depart result (None).
// Capacity:   print capacity_demo().
// Three Ways to Iterate: speeds = [100,120,150]; sum; boost by 10; print
//             "  boosted: {speeds:?}"; sum again; consume_names(stations.clone())
//             then show the original is still usable (.len()).
// Searching:  loop over ["Salem", "Madurai"].
// Seat Grid:  3 rows x 4 cols; try booking (0,0) (1,2) (2,3) (1,2) (3,0) (0,9)
//             in a loop over an array of tuples; then print render_grid lines.
// Slices:     times = [0,25,70,90,150,160] as an array; max_gap(&times) and
//             max_gap(&times[..1]); chunk_sums(&[1..=7], 3) with {:?}.
// Borrow Trap: fresh build_stations(), call first_then_push(.., "Madurai"),
//             print first, the new length, and route.last() with {:?}.

fn main() {
    // Your code here
}
