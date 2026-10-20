// =============================================
// EXERCISE 03a: Ownership: Moves and Copies
// =============================================
//
// There is NO code provided - you write everything, including main().
// This exercises: moves, Copy vs Clone, move in/out, Vec moves, partial moves, Drop order, move closures.
//
// Run with: cargo run
//
// Expected output (match it exactly, including the two-space indents):
//
//   === OWNERSHIP: MOVES AND COPIES ===
//
//   --- Move In, Move Out ---
//     Rajdhani!
//     consumed, len was 9
//
//   --- Copy Types ---
//     x=3 y=9 -> p=9 q=3
//     total=460, array still usable: [100, 120, 150, 90]
//     label_len=7
//
//   --- Clone ---
//     Chennai Chennai Chennai
//
//   --- Moving Into and Out of Vec ---
//     ["Anand", "Bala", "Chitra"]
//     second: Some("Bala")
//     left: ["Anand", "Chitra"]
//     longest: Some("Chitra")
//     longest of none: None
//
//   --- Accumulator ---
//     Chennai+Katpadi+Salem
//
//   --- Copy vs Clone Structs ---
//     original platform Platform(3), copied Platform(3)
//     Duronto @ Platform(5)
//     Duronto @ Platform(3)
//
//   --- Partial Moves ---
//     Devi / MAS-CBE / seat 42
//     Esha -> (transferred) / MAS-MDU / seat 7
//
//   --- Drop Order ---
//     inspecting B
//     dropped coach B
//     end of drop_demo
//     dropped coach C
//     dropped coach A
//     explicit drop next
//     dropped coach Z
//     after drop
//
//   --- Closures That Own ---
//     Now arriving: Shatabdi
//     Now arriving: Vande Bharat

// =============================================
// STEP 1: `add_bang(s: String) -> String`
// =============================================
// Takes OWNERSHIP of s, pushes '!' on the end, hands it back.
// Hint: the parameter must be declared `mut s: String`.
// In main: `let name = add_bang(name);` (shadowing re-binds the returned owner).

// =============================================
// STEP 2: `consume_len(s: String) -> usize`
// =============================================
// Takes ownership and returns s.len(). The String is dropped at the end of the
// function, so the caller can never use it again. After calling it in main,
// try printing the moved variable and read E0382, then remove that line.

// =============================================
// STEP 3: `swap_pair(a: i32, b: i32) -> (i32, i32)`
// =============================================
// Returns (b, a). i32 is Copy, so the caller's x and y stay usable.

// =============================================
// STEP 4: arrays and tuples of Copy data are Copy too
// =============================================
// fn sum_speeds(speeds: [u32; 4]) -> u32     (loop with `for s in speeds`)
// fn label_len(pair: (u32, String)) -> usize (returns pair.1.len())
// [u32; 4] is Copy, so `speeds` is still usable after the call.
// (u32, String) is NOT Copy (String is not), so the tuple moves.

// =============================================
// STEP 5: `twins(s: &String) -> (String, String)`
// =============================================
// Borrow in, two independent owned copies out. Use .clone() twice.

// =============================================
// STEP 6: `collect_names(a: String, b: String, c: String) -> Vec<String>`
// =============================================
// Moves all three into a Vec with vec![a, b, c].

// =============================================
// STEP 7: taking from a Vec
// =============================================
// fn longest_owned(names: Vec<String>) -> Option<String>
//     Consume the Vec; keep the first name with the greatest length.
//     Hint: `let mut best: Option<String> = None;` loop `for name in names`;
//     compare against `&best` with match, replace with `best = Some(name)`
//     only when the new one is strictly longer (or best is None).
// fn take_second(names: &mut Vec<String>) -> Option<String>
//     If len >= 2, Some(names.remove(1)), else None. (remove moves it out.)

// =============================================
// STEP 8: `join_all(parts: Vec<String>) -> String`
// =============================================
// Consume the Vec and join with '+' (no leading '+'). Build an accumulator
// String::new(); push('+') when it is not empty; push_str(&p).

// =============================================
// STEP 9: Copy vs Clone on structs
// =============================================
// #[derive(Debug, Clone, Copy, PartialEq)] struct Platform(u8);
// #[derive(Debug, Clone)] struct Train { name: String, platform: Platform }
// fn reassign(train: Train, platform: Platform) -> Train
//     struct update syntax: Train { platform, ..train }
// fn describe_train(t: Train) -> String      -> "{name} @ {platform:?}"
// Train cannot be Copy because it contains a String.

// =============================================
// STEP 10: partial moves
// =============================================
// struct Ticket { passenger: String, route: String, seat: u32 }
// fn split_ticket(t: Ticket) -> (String, String, u32)
//     move the fields out one by one: passenger, route, then the Copy seat.
// fn passenger_only(t: Ticket) -> (String, Ticket)
//     move passenger out, then rebuild a Ticket with passenger set to
//     "(transferred)" and the remaining route/seat.

// =============================================
// STEP 11: Drop order
// =============================================
// struct Coach(&'static str);
// impl Drop for Coach: fn drop(&mut self) prints "  dropped coach {name}"
// fn inspect(coach: Coach): prints "  inspecting {name}" then drops at the end.
// fn drop_demo(): create _a = Coach("A"), b = Coach("B"), _c = Coach("C");
//     inspect(b); print "  end of drop_demo".
// Locals drop in REVERSE declaration order; a moved value drops where it was moved to.
// In main, also use drop(late) to drop early.

// =============================================
// STEP 12: closures that own
// =============================================
// fn make_announcer(prefix: String) -> impl Fn(&str) -> String
//     returns `move |train| format!("{prefix}: {train}")`.
// `move` transfers prefix into the closure so it outlives the function.

// =============================================
// STEP 13: Write `fn main()`
// =============================================
// Print each section exactly as in the expected output (blank line before each
// header after the first, two-space indents). Reproduce the demos in the order
// of the steps above; the expected output shows every value to use.

fn main() {
    // Your code here
}
