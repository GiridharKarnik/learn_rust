// =============================================
// EXERCISE 06: Railway Lost & Found System
// =============================================
//
// Build a lost & found system using Option, Result, and pattern matching.
// You write EVERYTHING from scratch — enums, structs, functions, and main().
//
// Run with: cargo run
//
// =============================================
// EXPECTED OUTPUT:
// =============================================
//
//   === RAILWAY LOST & FOUND SYSTEM ===
//
//   --- All Lost Items ---
//   Item #1: Black laptop bag (Train #12049, Coach A1) [Electronics] — Unclaimed
//   Item #2: Blue denim jacket (Train #12049, Coach S3) [Clothing] — Unclaimed
//   Item #3: Indian passport (Train #12007, Coach B2) [Document] — Unclaimed
//   Item #4: Red suitcase with wheels (Train #12213, Coach S1) [Luggage] — Unclaimed
//   Item #5: Reading glasses in brown case (Train #12007, Coach A1) [Other: eyewear] — Unclaimed
//   Item #6: Silver wristwatch (Train #12578, Coach S5) [Electronics] — Unclaimed
//   Item #7: Umbrella — black folding (Train #12049, Coach S3) [Other: accessory] — Unclaimed
//
//   --- Search by ID ---
//   Looking for item #3: Found! Indian passport
//   Looking for item #99: Not found
//
//   --- Search by Train ---
//   Items from train #12049:
//     - Item #1: Black laptop bag
//     - Item #2: Blue denim jacket
//     - Item #7: Umbrella — black folding
//   Items from train #99999:
//     (none)
//
//   --- Parse Categories ---
//     "electronics" → Ok: Electronics
//     "clothing" → Ok: Clothing
//     "document" → Ok: Document
//     "luggage" → Ok: Luggage
//     "" → Error: Category cannot be empty
//     "eyewear" → Ok: Other("eyewear")
//
//   --- Process Claims ---
//   Claim item #3 for Giridhar: Ok — Item #3 claimed by Giridhar
//   Claim item #3 for Priya: Error — Item #3 has already been claimed
//   Claim item #99 for Arjun: Error — Item #99 not found in the system
//
//   --- Quick Description Lookup ---
//   Item #1: Black laptop bag
//   Item #99: No item with id #99
//
//   --- if let Lookup ---
//   Found item #6: Silver wristwatch — claiming it now
//
//   === Daily Lost & Found Report ===
//   Total items: 7
//   Claimed: 1
//   Unclaimed: 6
//   Items by category:
//     Electronics: 2
//     Clothing: 1
//     Document: 1
//     Luggage: 1
//     Other: 2

// =============================================
// STEP 1: Define the `ItemCategory` enum
// =============================================
//
// Variants:
//   Electronics        (no data)
//   Clothing           (no data)
//   Document           (no data)
//   Luggage            (no data)
//   Other(String)      (holds a description)
//
// Derive: Debug, Clone

// =============================================
// STEP 2: Define the `LostItem` struct
// =============================================
//
// Fields:
//   id: u32
//   description: String
//   train_number: u32
//   coach: String
//   category: ItemCategory
//   claimed: bool
//
// Derive: Debug

// =============================================
// STEP 3: Implement `LostItem` methods
// =============================================
//
//   fn new(id: u32, description: &str, train_number: u32, coach: &str,
//          category: ItemCategory) -> Self
//     `claimed` defaults to false.
//
//   fn display(&self)
//     Prints: "Item #{id}: {description} (Train #{train_number}, Coach {coach}) [{category}] — {status}"
//     Where category is: "Electronics", "Clothing", "Document", "Luggage", or "Other: {s}"
//     Where status is: "Claimed" or "Unclaimed"
//     (— is an em dash, Unicode \u{2014})
//
//   fn claim(&mut self)
//     Sets claimed to true.

// =============================================
// STEP 4: Implement `find_item_by_id`
// =============================================
//
// fn find_item_by_id(items: &[LostItem], id: u32) -> Option<&LostItem>
//
// Search the slice for an item with matching id.
// Return Some(&item) if found, None otherwise.
//
// Hint: items.iter().find(|item| item.id == id)
//   Or use a for loop.

// =============================================
// STEP 5: Implement `find_items_by_train`
// =============================================
//
// fn find_items_by_train(items: &[LostItem], train_number: u32) -> Vec<&LostItem>
//
// Return ALL items from a specific train.
//
// Hint: items.iter().filter(|item| item.train_number == train_number).collect()
//   Or use a for loop and push to a Vec.

// =============================================
// STEP 6: Implement `parse_item_category`
// =============================================
//
// fn parse_item_category(input: &str) -> Result<ItemCategory, String>
//
// Parse a string into an ItemCategory. Case-insensitive.
//   "" (empty)      → Err("Category cannot be empty")
//   "electronics"   → Ok(ItemCategory::Electronics)
//   "clothing"      → Ok(ItemCategory::Clothing)
//   "document"      → Ok(ItemCategory::Document)
//   "luggage"       → Ok(ItemCategory::Luggage)
//   anything else   → Ok(ItemCategory::Other(input.to_string()))
//
// Hint: Check for empty first with input.is_empty(),
// then match on input.to_lowercase().as_str()

// =============================================
// STEP 7: Implement `process_claim`
// =============================================
//
// fn process_claim(items: &mut [LostItem], item_id: u32, owner_name: &str) -> Result<String, String>
//
// Try to claim an item:
//   1. Find the item by id (use items.iter_mut().find()).
//      If not found → Err("Item #{item_id} not found in the system")
//   2. If already claimed → Err("Item #{item_id} has already been claimed")
//   3. Otherwise, call .claim() and return Ok("Item #{item_id} claimed by {owner_name}")
//
// Advanced: Use .ok_or(...)? to convert Option to Result:
//   let item = items.iter_mut().find(|i| i.id == item_id)
//       .ok_or(format!("Item #{item_id} not found in the system"))?;

// =============================================
// STEP 8: Implement `get_item_description`
// =============================================
//
// fn get_item_description(items: &[LostItem], id: u32) -> String
//
// Return the description of an item, or a default message if not found.
//
// Challenge — do it in ONE expression using Option methods:
//   find_item_by_id(items, id)
//       .map(|item| item.description.clone())
//       .unwrap_or_else(|| format!("No item with id #{id}"))
//
// Or use match if you prefer.

// =============================================
// STEP 9: Implement `daily_report`
// =============================================
//
// fn daily_report(items: &[LostItem])
//
// Loop through items and count:
//   - Total items (items.len())
//   - Claimed vs Unclaimed (check item.claimed)
//   - Count by category (match on &item.category)
//
// Print:
//   === Daily Lost & Found Report ===
//   Total items: 7
//   Claimed: 1
//   Unclaimed: 6
//   Items by category:
//     Electronics: 2
//     Clothing: 1
//     Document: 1
//     Luggage: 1
//     Other: 2

// =============================================
// STEP 10: Write main()
// =============================================
//
// Your main() should do the following:
//
// 1. Print "=== RAILWAY LOST & FOUND SYSTEM ==="
//
// 2. Print "\n--- All Lost Items ---"
//    Create a mutable Vec of 7 lost items:
//      #1: "Black laptop bag",              train 12049, coach "A1", Electronics
//      #2: "Blue denim jacket",             train 12049, coach "S3", Clothing
//      #3: "Indian passport",               train 12007, coach "B2", Document
//      #4: "Red suitcase with wheels",      train 12213, coach "S1", Luggage
//      #5: "Reading glasses in brown case", train 12007, coach "A1", Other("eyewear")
//      #6: "Silver wristwatch",             train 12578, coach "S5", Electronics
//      #7: "Umbrella — black folding",      train 12049, coach "S3", Other("accessory")
//    Display all items with a for loop.
//
// 3. Print "\n--- Search by ID ---"
//    Search for item #3 — match on the Option, print "Found! {description}" or "Not found".
//    Search for item #99 — same.
//
// 4. Print "\n--- Search by Train ---"
//    Find items from train #12049. Print "Items from train #12049:".
//      If empty, print "  (none)". Otherwise print each as "  - Item #{id}: {description}".
//    Find items from train #99999. Same format.
//
// 5. Print "\n--- Parse Categories ---"
//    Parse these inputs: "electronics", "clothing", "document", "luggage", "", "eyewear"
//    For each, match on the Result and print:
//      Ok  → "  \"{input}\" → Ok: {category:?}"
//      Err → "  \"{input}\" → Error: {error}"
//    (→ is Unicode \u{2192})
//
// 6. Print "\n--- Process Claims ---"
//    process_claim for item #3, owner "Giridhar" — print "Claim item #3 for Giridhar: Ok — {msg}" or "Error — {e}"
//    process_claim for item #3, owner "Priya"    — same format (will fail: already claimed)
//    process_claim for item #99, owner "Arjun"   — same format (will fail: not found)
//    (— is Unicode \u{2014})
//
// 7. Print "\n--- Quick Description Lookup ---"
//    Print "Item #1: {}" with get_item_description for id 1.
//    Print "Item #99: {}" with get_item_description for id 99.
//
// 8. Print "\n--- if let Lookup ---"
//    Use `if let Some(item) = find_item_by_id(&items, 6)` to print:
//      "Found item #{id}: {description} — claiming it now"
//
// 9. Print a blank line, then call daily_report(&items).

fn main() {
    // Your code here
}
