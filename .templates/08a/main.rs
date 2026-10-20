// =============================================
// EXERCISE 08a: Range Type Drills
// =============================================
//
// Twelve small drills on the whole Range family: Range, RangeInclusive,
// RangeFrom, RangeTo, RangeToInclusive and RangeFull.
// There is NO code provided - you write everything, including main().
//
// This exercises: .start/.end, .start()/.end(), len, is_empty, contains,
// .rev(), .step_by(), slicing with ranges, char ranges, float ranges,
// ranges as struct fields, saturating/checked arithmetic at boundaries.
//
// Put this at the top of your file:
//     use std::ops::{Range, RangeInclusive};
//
// Run with: cargo run
//
// Expected output:
//
//   === RANGE TYPE DRILLS ===
//
//   --- 1. Range<i32> anatomy ---
//     1..5 -> start=1 end=5 len=4 empty=false
//     3..3 -> start=3 end=3 len=0 empty=true
//     5..1 -> start=5 end=1 len=0 empty=true
//     -2..2 -> start=-2 end=2 len=4 empty=false
//
//   --- 2. RangeInclusive<u32> anatomy ---
//     1..=5 -> start=1 end=5 count=5 empty=false
//     3..=3 -> start=3 end=3 count=1 empty=false
//     5..=1 -> start=5 end=1 count=0 empty=true
//
//   --- 3. The range family ---
//     1. 2..6
//     2. 2..=6
//     3. 2..
//     4. ..6
//     5. ..=6
//     6. ..
//
//   --- 4. Countdown and platform numbers ---
//     countdown(5) = [5, 4, 3, 2, 1]
//     countdown(0) = []
//     platforms(1, 12, 3) = [1, 4, 7, 10]
//     platforms(250, 255, 2) = [250, 252, 254]
//
//   --- 5. step_by vs rev order ---
//     step_by(3).rev(): [9, 6, 3, 0]
//     rev().step_by(3): [10, 7, 4, 1]
//
//   --- 6. Sums and overflow ---
//     triangular(10) = 55
//     triangular(0) = 0
//     checked_total(1..=22) = Some(253)
//     checked_total(1..=23) = None
//
//   --- 7. Slicing with ranges ---
//     window(1..3) = Some(["Arakkonam", "Katpadi"])
//     window(0..0) = Some([])
//     window(3..9) = None
//     window(4..2) = None
//     last_n(2) = ["Jolarpettai", "Salem"]
//     last_n(99) = 5
//
//   --- 8. Char ranges ---
//     letters(a, f) = abcdef
//     letters(x, c) = ""
//     Platform A
//     Platform B
//     Platform C
//
//   --- 9. Float ranges ---
//         0 -> low
//      4.99 -> low
//         5 -> normal
//         8 -> normal
//      8.01 -> high
//        -1 -> invalid
//       NaN -> invalid
//
//   --- 10. Ranges in structs ---
//     Morning: 06:00-14:00 (8h) | covers 14h? false
//     Evening: 14:00-22:00 (8h) | covers 14h? true
//     Night: empty shift | covers 14h? false
//     Shift { label: "Morning", hours: 6..14 }
//     morning == clone? true
//
//   --- 11. Chunking and spans ---
//     chunk_ranges(10, 4) = [0..4, 4..8, 8..10]
//     chunk_ranges(8, 4) = [0..4, 4..8]
//     chunk_ranges(0, 4) = []
//     span_of([40, 15, 90, 62]) = Some(15..=90)
//     span_of([]) = None
//
//   --- 12. Clamped platform bumps ---
//     bump(3, 2) -> platform 5, clamped=false
//     bump(9, 5) -> platform 12, clamped=true
//     bump(250, 10) -> platform 12, clamped=true

// =============================================
// STEP 1: `range_info`
// =============================================
// fn range_info(r: Range<i32>) -> String
//
// Returns: "{r:?} -> start={start} end={end} len={len} empty={empty}"
// Hints:
//   - Range has PUBLIC FIELDS: r.start and r.end
//   - Range<i32> implements ExactSizeIterator, so r.len() works
//   - r.is_empty() is true when start >= end (5..1 is empty, NOT an error)
//   - Format the debug string FIRST or use r.clone(): don't move r early

// =============================================
// STEP 2: `inclusive_info`
// =============================================
// fn inclusive_info(r: RangeInclusive<u32>) -> String
//
// Returns: "{r:?} -> start={} end={} count={} empty={}"
// Hints:
//   - RangeInclusive hides its fields: use r.start() and r.end()
//   - There is no .len() for RangeInclusive<u32>; use r.clone().count()
//     (count() consumes the iterator, clone() keeps r usable)
//   - 3..=3 is NOT empty (one item); 5..=1 is empty

// =============================================
// STEP 3: `range_family`
// =============================================
// fn range_family() -> Vec<String>
//
// Return the Debug text of these six literals, in order:
//   2..6   2..=6   2..   ..6   ..=6   ..
// Hint: format!("{:?}", 2..6). Each literal has a different type
// (Range, RangeInclusive, RangeFrom, RangeTo, RangeToInclusive, RangeFull).

// =============================================
// STEP 4: `countdown` and `platform_numbers`
// =============================================
// fn countdown(from: u32) -> Vec<u32>
//     from, from-1, ... 1 (empty for 0). Hint: (1..=from).rev()
//
// fn platform_numbers(first: u8, last: u8, step: usize) -> Vec<u8>
//     every `step`-th platform from first to last INCLUSIVE.
//     Hint: (first..=last).step_by(step). Watch 250..=255 on a u8:
//     the inclusive range must not overflow while iterating.

// =============================================
// STEP 5: `step_then_rev`
// =============================================
// fn step_then_rev(n: u32, k: usize) -> (Vec<u32>, Vec<u32>)
//
// Over 0..n return a tuple of two vectors:
//   .0 = (0..n).step_by(k).rev()   -> steps from the front, then reversed
//   .1 = (0..n).rev().step_by(k)   -> reversed first, then stepped
// For n=11,k=3 the two orders give DIFFERENT answers. Observe why.

// =============================================
// STEP 6: `triangular` and `checked_total`
// =============================================
// fn triangular(n: u32) -> u32
//     1 + 2 + ... + n (0 for n = 0). Hint: (1..=n).sum()
//
// fn checked_total(r: RangeInclusive<u8>) -> Option<u8>
//     Sum the range but return None if the u8 total would overflow.
//     Hint: r.try_fold(0u8, |acc, x| acc.checked_add(x))
//     try_fold needs `mut r`. Note 1..=22 sums to 253, 1..=23 to 276.

// =============================================
// STEP 7: Slicing with ranges
// =============================================
// fn window<T>(items: &[T], r: Range<usize>) -> Option<&[T]>
//     Non-panicking slice: items.get(r). Out-of-bounds or start > end
//     gives None; 0..0 gives Some(empty slice).
//
// fn last_n<T>(items: &[T], n: usize) -> &[T]
//     The last n items (all of them if n > len).
//     Hint: &items[items.len().saturating_sub(n)..]   (RangeFrom!)

// =============================================
// STEP 8: Char ranges
// =============================================
// fn letters(from: char, to: char) -> String
//     All chars from..=to collected into a String (empty if from > to).
//     Hint: (from..=to).collect()  - char ranges ARE iterable.
//
// fn platform_labels(last: char) -> Vec<String>
//     "Platform A", "Platform B", ... up to and including `last`.

// =============================================
// STEP 9: Float ranges are not iterable
// =============================================
// fn gauge_status(pressure: f64) -> &'static str
//
//   outside 0.0.. (negative or NaN) => "invalid"
//   0.0 <= p < 5.0                  => "low"
//   5.0 <= p <= 8.0                 => "normal"
//   otherwise                       => "high"
//
// Hint: `.contains(&p)` works on float ranges, but you cannot loop over
// them (f64 has no "next value"). (0.0..).contains(&f64::NAN) is false.
// Match patterns on floats are discouraged, so use if / else if.

// =============================================
// STEP 10: Ranges as struct fields
// =============================================
// #[derive(Debug, Clone, PartialEq)]
// struct Shift { label: &'static str, hours: Range<u32> }
//
// impl Shift {
//     fn covers(&self, hour: u32) -> bool        // hours.contains(&hour)
//     fn duration(&self) -> u32                  // end - start, never underflows
//                                                // (use saturating_sub)
//     fn describe(&self) -> String
// }
// describe: if hours.is_empty() => "{label}: empty shift"
//           else                => "{label}: {start:02}:00-{end:02}:00 ({duration}h)"
// (A night shift written as 22..6 is an EMPTY range - a known trap.)

// =============================================
// STEP 11: `chunk_ranges` and `span_of`
// =============================================
// fn chunk_ranges(total: usize, size: usize) -> Vec<Range<usize>>
//     Split 0..total into consecutive ranges of at most `size`.
//     Hint: (0..total).step_by(size).map(|s| s..(s + size).min(total))
//
// fn span_of(values: &[i32]) -> Option<RangeInclusive<i32>>
//     min..=max of the values; None for an empty slice.
//     Hint: use `?` on iter().min() inside a function returning Option.

// =============================================
// STEP 12: `bump_platform`
// =============================================
// fn bump_platform(current: u8, by: u8, last: u8) -> (u8, bool)
//
// Move `by` platforms up, never beyond `last`. Return (new_platform, clamped).
// u8 addition can overflow (250 + 10), so use checked_add:
//   Some(n) with n <= last => (n, false)
//   overflow or n > last   => (last, true)

// =============================================
// STEP 13: Write `fn main()`
// =============================================
// Print exactly the EXPECTED OUTPUT above. Structure:
//   - "=== RANGE TYPE DRILLS ===", then each section as "\n--- N. Title ---"
//   - 1: loop over [1..5, 3..3, 5..1, -2..2]  (an array of Range<i32>)
//   - 2: loop over [1..=5, 3..=3, 5..=1]
//   - 3: enumerate range_family(), print "  {i+1}. {text}"
//   - 7: stops = ["Chennai","Arakkonam","Katpadi","Jolarpettai","Salem"];
//        print window(1..3), (0..0), (3..9), (4..2) with {:?}; then last_n(2)
//        and last_n(99).len()
//   - 8: letters('a','f') with {}, letters('x','c') with {:?}, then labels
//   - 9: for p in [0.0, 4.99, 5.0, 8.0, 8.01, -1.0, f64::NAN] print "  {:>5} -> {}"
//   - 10: three shifts: Morning 6..14, Evening 14..22, Night 22..6;
//         "  {describe} | covers 14h? {bool}", then {:?} of the first and
//         whether it equals its own clone
//   - 12: for (cur, by) in [(3u8,2u8),(9,5),(250,10)] with last = 12
// Everything is indented two spaces. Compare with `cargo run` carefully.

fn main() {
    // Your code here
}
