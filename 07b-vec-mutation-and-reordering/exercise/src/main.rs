// =============================================
// EXERCISE 07b: Vec Mutation and Reordering
// =============================================
//
// There is NO code provided - you write everything, including main().
// This exercises: remove/swap_remove, truncate, split_off, append, extend, retain,
// drain, dedup, sort/sort_by/sort_by_key, binary_search, rotate/reverse/swap,
// and the "remove while iterating" trap.
//
// Run with: cargo run
//
// Expected output (match it exactly, including the two-space indents):
//
//   === VEC MUTATION AND REORDERING ===
//
//   --- Remove: ordered vs fast ---
//     remove_ordered(1) -> Some("B"), left ["A", "C", "D", "E"]
//     remove_fast(1)    -> Some("B"), left ["A", "E", "C", "D"]
//     remove_ordered(9) -> None
//     remove_fast(9)    -> None
//
//   --- Truncate and split_off ---
//     kept: [10, 20, 30]
//     kept (n too big): [10, 20, 30]
//     head: ["Chennai", "Katpadi", "Salem"]
//     tail: ["Erode", "Coimbatore"]
//     split past end: [], head len 3
//
//   --- Append and extend ---
//     merged: ["Chennai", "Katpadi", "Salem", "Erode"]
//     donor empty: true (len 0)
//     extended: ["Chennai", "Katpadi", "Salem", "Erode", "Tiruppur", "Coimbatore"]
//
//   --- Retain ---
//     dropped 3, kept [120, 150, 110]
//     dropped again: 0
//
//   --- Drain ---
//     drained: ["S2", "S3", "S4"]
//     left: ["S1", "S5", "S6"]
//     take_all: ["S1", "S5", "S6"], left 0 items
//
//   --- Dedup ---
//     consecutive only: ["Salem", "Erode", "Salem"]
//     unique_sorted: [1, 2, 3]
//
//   --- Sorting ---
//     desc: [180, 150, 130, 110, 90]
//     len then alpha: ["Goa", "Jan", "Mail", "Kovai", "Duronto", "Rajdhani"]
//     by last char (stable): ["Goa", "Kovai", "Rajdhani", "Mail", "Jan", "Duronto"]
//     floats desc: [1200.25, 250.5, 99.0, 99.0, 15.75]
//
//   --- Binary search ---
//     30 found at 2
//     10 found at 0
//     50 found at 4
//     35 missing, would insert at 3
//     5 missing, would insert at 0
//     99 missing, would insert at 5
//     after inserts: [5, 10, 20, 30, 30, 35, 40, 50, 99]
//
//   --- Rotate, reverse, swap ---
//     rotate_left(2) then reverse: [2, 1, 6, 5, 4, 3]
//     swap ends: [3, 1, 6, 5, 4, 2]
//
//   --- Removing while iterating ---
//     removed: [2, 4, 6, 8, 8]
//     remaining: [5, 7, 9]
//
//   --- In-place edits ---
//     doubled: [2, 4, 6, 8, 10]
//     pairs summing to 12: [(0, 4), (1, 3)]
//     pairs summing to 1: []

// =============================================
// STEP 1: `remove_ordered` and `remove_fast`
// =============================================
// fn remove_ordered(v: &mut Vec<String>, index: usize) -> Option<String>
// fn remove_fast(v: &mut Vec<String>, index: usize) -> Option<String>
//
// Both return None when index >= v.len() (the std methods would PANIC).
//   remove_ordered: v.remove(index)       - O(n), shifts the tail left, order kept
//   remove_fast:    v.swap_remove(index)  - O(1), moves the LAST element into the hole
// Compare the "left" lists in the expected output to see the difference.

// =============================================
// STEP 2: `keep_first` and `split_route`
// =============================================
// fn keep_first(v: &mut Vec<u32>, n: usize)
//     v.truncate(n). Truncating to a length larger than len is a harmless no-op.
// fn split_route(v: &mut Vec<String>, at: usize) -> Vec<String>
//     v.split_off(at) returns the tail [at..] and leaves [..at] in v.
//     split_off PANICS if at > len, so clamp first: at.min(v.len()).

// =============================================
// STEP 3: `merge` and `extend_from`
// =============================================
// fn merge(a: &mut Vec<String>, b: &mut Vec<String>)
//     a.append(b): moves ALL of b into a, leaving b empty (but still alive).
// fn extend_from(a: &mut Vec<String>, extra: &[&str])
//     a.extend(iterator of String). Hint: extra.iter().map(|s| s.to_string()).

// =============================================
// STEP 4: `drop_slow`
// =============================================
// fn drop_slow(speeds: &mut Vec<u32>, min: u32) -> usize
//     Keep only speeds >= min with retain(|&s| ...). Return how many were removed
//     (len before minus len after).

// =============================================
// STEP 5: `take_range` and `take_all`
// =============================================
// fn take_range(v: &mut Vec<String>, start: usize, end: usize) -> Vec<String>
//     v.drain(start..end).collect() - removes AND yields those items.
// fn take_all(v: &mut Vec<String>) -> Vec<String>
//     v.drain(..).collect() - empties the Vec but keeps its allocation.
// (drain panics on a bad range; we don't guard it here, just use valid ranges.)

// =============================================
// STEP 6: `dedup_consecutive` and `unique_sorted`
// =============================================
// fn dedup_consecutive(v: &mut Vec<String>)
//     v.dedup() removes only ADJACENT duplicates.
// fn unique_sorted(v: &mut Vec<u32>)
//     sort() first, then dedup() - the classic way to get unique values.

// =============================================
// STEP 7: sorting with closures
// =============================================
// fn sort_desc(v: &mut [u32])
//     sort_by(|a, b| b.cmp(a))   (swap a and b to reverse)
// fn sort_by_len_then_alpha(v: &mut [String])
//     sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)))
//     Ordering::then only looks at the second comparison on a tie.
// fn sort_by_last_char(v: &mut [String])
//     sort_by_key(|s| s.chars().last())   (key is Option<char>; Option is Ord)
//     sort_by_key is STABLE: equal keys keep their previous relative order.
// Note: all take slices - sorting never changes the length.

// =============================================
// STEP 8: `sort_floats_desc`
// =============================================
// fn sort_floats_desc(v: &mut [f64])
//     f64 is not Ord (NaN!), so v.sort() will not compile. Use
//     sort_by(|a, b| b.partial_cmp(a).unwrap()).  Try plain sort() once and
//     read the error.

// =============================================
// STEP 9: `locate` and `insert_sorted`
// =============================================
// fn locate(sorted: &[u32], target: u32) -> String
//     match sorted.binary_search(&target):
//       Ok(i)  -> "{target} found at {i}"
//       Err(i) -> "{target} missing, would insert at {i}"
// fn insert_sorted(sorted: &mut Vec<u32>, value: u32)
//     Position = binary_search(&value).unwrap_or_else(|i| i)  (both Ok and Err
//     carry a valid insertion index), then sorted.insert(pos, value).
// binary_search requires sorted input; on unsorted data the result is garbage.

// =============================================
// STEP 10: `rotate_and_reverse` and `swap_ends`
// =============================================
// fn rotate_and_reverse(v: &mut [u32])   rotate_left(2) then reverse()
// fn swap_ends(v: &mut [u32])            v.swap(0, last) where last = len - 1
//     (assume at least one element; len - 1 would underflow on empty!)

// =============================================
// STEP 11: `remove_while_iterating`
// =============================================
// fn remove_while_iterating(v: &mut Vec<u32>) -> Vec<u32>
//     Remove every EVEN number from v and return the removed ones in order.
//     `for x in v.iter() { v.remove(..) }` cannot compile (shared borrow +
//     mutation). Use a while loop with an index i: if v[i] is even, push
//     v.remove(i) to the result and do NOT advance i (the next item slid into
//     slot i); otherwise i += 1.
//     (retain would be the idiomatic fix when you don't need the removed items.)

// =============================================
// STEP 12: in-place edits and a helper
// =============================================
// fn double_in_place(v: &mut [u32])           for x in v.iter_mut() { *x *= 2 }
// fn pairs_sum_to(v: &[u32], target: u32) -> Vec<(usize, usize)>
//     All index pairs (i, j) with i < j and v[i] + v[j] == target, in loop order.
//     Nested loops: i in 0..len, j in (i+1)..len.
// fn names(v: &[&str]) -> Vec<String>         convenience: &str list -> Vec<String>
//     Use this helper in main to build test data.

// =============================================
// STEP 13: Write `fn main()`
// =============================================
// Print the title, then each section exactly as shown (blank line before every
// header except via the "\n" after the title). Test data per section:
//
// Remove:    a = names A..E, b = a.clone(). Print remove_ordered(&mut a,1) with
//            {:?} and the remaining list; same for remove_fast on b (note the
//            aligned arrows); then remove_ordered(9) and remove_fast(9).
// Truncate:  times = [10,20,30,40,50]; keep_first 3, then 10. route =
//            Chennai, Katpadi, Salem, Erode, Coimbatore; split_route at 3, then
//            at 99 (print the empty result and the head's len).
// Append:    first_half = Chennai, Katpadi; second_half = Salem, Erode; merge;
//            print merged, then second_half.is_empty() and .len(); extend_from
//            Tiruppur, Coimbatore.
// Retain:    speeds = [40,120,60,150,30,110]; drop_slow(100) twice.
// Drain:     stops S1..S6; take_range(1,4); print drained and left; take_all.
// Dedup:     log = Salem,Salem,Erode,Salem,Salem,Salem; platforms =
//            [3,1,3,2,1,3,2].
// Sorting:   nums = [130,90,150,110,180]; words = Kovai, Duronto, Rajdhani,
//            Mail, Jan, Goa (sort by len+alpha, then by last char on the SAME
//            vec); fares = [250.5, 99.0, 1200.25, 99.0, 15.75].
// Binary search: sorted = [10,20,30,40,50]; locate for 30,10,50,35,5,99; then
//            insert_sorted 35, 5, 99, 30 and print.
// Rotate:    r = [1,2,3,4,5,6]; rotate_and_reverse, then swap_ends.
// Removing while iterating: mixed = [2,4,5,6,7,8,8,9].
// In-place:  loads = [1,2,3,4,5]; double_in_place; pairs_sum_to 12 and 1.

fn main() {
    // Your code here
}
