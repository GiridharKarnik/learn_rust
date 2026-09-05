// =============================================
// EXERCISE 06: Railway Lost & Found System
// =============================================
//
// Build a lost & found system using Option, Result, and pattern matching.
// Complete all the TODOs so the program compiles and produces the expected output.
//
// Run with: cargo run
//
// Expected output:
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
// TODO 1: Define the `ItemCategory` enum
// =============================================
//
// Variants:
//   - Electronics        (no data)
//   - Clothing           (no data)
//   - Document           (no data)
//   - Luggage            (no data)
//   - Other(String)      (holds a description)
//
// Derive: Debug, Clone
#[derive(Debug, Clone)]
enum ItemCategory {
    Electronics,
    Clothing,
    Document,
    Luggage,
    Other(String),
}

// =============================================
// TODO 2: Define the `LostItem` struct
// =============================================
//
// Fields:
//   - id: u32
//   - description: String
//   - train_number: u32
//   - coach: String
//   - category: ItemCategory
//   - claimed: bool
//
// Derive: Debug
struct LostItem {
    id: u32,
    description: String,
    train_number: u32,
    coach: String,
    category: ItemCategory,
    claimed: bool,
}

// =============================================
// TODO 3: Implement `LostItem` methods
// =============================================
//
// Associated function:
//   - fn new(id: u32, description: &str, train_number: u32, coach: &str,
//            category: ItemCategory) -> Self
//     `claimed` defaults to false
//
// Methods:
//   - fn display(&self)
//     Prints: "Item #{id}: {description} (Train #{train_number}, Coach {coach}) [{category}] — {status}"
//     Where category is: "Electronics", "Clothing", "Document", "Luggage", or "Other: {s}"
//     Where status is: "Claimed" or "Unclaimed"
//
//   - fn claim(&mut self)
//     Sets claimed to true
impl LostItem {
    fn new(
        id: u32,
        description: &str,
        train_number: u32,
        coach: &str,
        category: ItemCategory,
    ) -> Self {
        return LostItem {
            id,
            description: String::from(description),
            train_number,
            coach: String::from(coach),
            category,
            claimed: false,
        };
    }

    fn display(&self) {
        println!(
            "Item #{}: {} (Train #{}, Coach {}) [{}] \u{2014} {}",
            self.id,
            self.description,
            self.train_number,
            self.coach,
            match &self.category {
                ItemCategory::Electronics => String::from("Electronics"),
                ItemCategory::Clothing => String::from("Clothing"),
                ItemCategory::Document => String::from("Document"),
                ItemCategory::Luggage => String::from("Luggage"),
                ItemCategory::Other(s) => format!("Other: {s}"),
            },
            if self.claimed { "Claimed" } else { "Unclaimed" }
        )
    }

    fn claim(&mut self) {
        self.claimed = true;
    }
}

// =============================================
// TODO 4: Implement `find_item_by_id`
// =============================================
//
// fn find_item_by_id(items: &[LostItem], id: u32) -> Option<&LostItem>
//
// Search the slice for an item with matching id.
// Return Some(&item) if found, None otherwise.
//
// You can use a for loop:
//   for item in items {
//       if item.id == id { return Some(item); }
//   }
//   None
//
// Or: items.iter().find(|item| item.id == id)

fn find_item_by_id(items: &[LostItem], id: u32) -> Option<&LostItem> {
    return items.iter().find(|item| item.id == id);
}

// =============================================
// TODO 5: Implement `find_items_by_train`
// =============================================
//
// fn find_items_by_train(items: &[LostItem], train_number: u32) -> Vec<&LostItem>
//
// Return ALL items from a specific train.
//
// You can use a for loop and push to a Vec:
//   let mut result = Vec::new();
//   for item in items {
//       if item.train_number == train_number { result.push(item); }
//   }
//   result
//
// Or: items.iter().filter(|item| item.train_number == train_number).collect()
fn find_items_by_train(items: &[LostItem], train_number: u32) -> Vec<&LostItem> {
    return items
        .iter()
        .filter(|item| item.train_number == train_number)
        .collect();
}

// =============================================
// TODO 6: Implement `parse_item_category`
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
fn parse_item_category(input: &str) -> Result<ItemCategory, String> {
    if input.is_empty() {
        return Err("Category cannot be empty".to_string());
    }

    match input.to_lowercase().as_str() {
        "electronics" => Ok(ItemCategory::Electronics),
        "clothing" => Ok(ItemCategory::Clothing),
        "document" => Ok(ItemCategory::Document),
        "luggage" => Ok(ItemCategory::Luggage),
        _ => Ok(ItemCategory::Other(input.to_string())),
    }
}

// =============================================
// TODO 7: Implement `process_claim`
// =============================================
//
// fn process_claim(items: &mut [LostItem], item_id: u32, owner_name: &str) -> Result<String, String>
//
// Try to claim an item:
//   1. Find the item by id. If not found → Err("Item #{item_id} not found in the system")
//   2. If already claimed → Err("Item #{item_id} has already been claimed")
//   3. Otherwise, call .claim() on it and return Ok("Item #{item_id} claimed by {owner_name}")
//
// Note: You need items.iter_mut().find() to get a mutable reference.
// Or use a for loop with `for item in items.iter_mut()`
//
// Advanced: You can use .ok_or()? to convert Option to Result:
//   let item = items.iter_mut().find(|i| i.id == item_id)
//       .ok_or(format!("Item #{item_id} not found in the system"))?;
fn process_claim(items: &mut [LostItem], item_id: u32, owner_name: &str) -> Result<String, String> {
    let item = items
        .iter_mut()
        .find(|item| item.id == item_id)
        .ok_or(format!("Item #{item_id} not found in the system"))?;

    // by the point the control reaches here, the combination of ok_or and ? would have ensured that the
    // variable item will have a valid value in it.
    if item.claimed {
        return Err(format!("Item #{item_id} has already been claimed"));
    }

    item.claim();
    return Ok(format!("Item #{item_id} claimed by {owner_name}"));
}

// =============================================
// TODO 8: Implement `get_item_description`
// =============================================
//
// fn get_item_description(items: &[LostItem], id: u32) -> String
//
// Return the description of an item, or a default message if not found.
//
// Challenge: Do this in ONE line using Option methods:
//   find_item_by_id(items, id)
//       .map(|item| item.description.clone())
//       .unwrap_or_else(|| format!("No item with id #{id}"))
//
// Or use match if you prefer.
fn get_item_description(items: &[LostItem], id: u32) -> String {
    return find_item_by_id(items, id)
        .map(|item| item.description.clone())
        .unwrap_or_else(|| format!("No item with id #{id}"));
}

// =============================================
// TODO 9: Implement `daily_report`
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
fn daily_report(items: &[LostItem]) {
    let mut claimed = 0;
    let mut unclaimed = 0;
    let mut electronics = 0;
    let mut clothing = 0;
    let mut document = 0;
    let mut luggage = 0;
    let mut other = 0;

    for item in items {
        if item.claimed {
            claimed += 1;
        } else {
            unclaimed += 1;
        }

        match &item.category {
            ItemCategory::Electronics => electronics += 1,
            ItemCategory::Clothing => clothing += 1,
            ItemCategory::Document => document += 1,
            ItemCategory::Luggage => luggage += 1,
            ItemCategory::Other(_) => other += 1,
        }
    }

    println!("=== Daily Lost & Found Report ===");
    println!("Total items: {}", items.len());
    println!("Claimed: {claimed}");
    println!("Unclaimed: {unclaimed}");
    println!("Items by category:");
    println!("  Electronics: {electronics}");
    println!("  Clothing: {clothing}");
    println!("  Document: {document}");
    println!("  Luggage: {luggage}");
    println!("  Other: {other}");
}

// =============================================
// main() — DO NOT EDIT BELOW THIS LINE
// =============================================

fn main() {
    println!("=== RAILWAY LOST & FOUND SYSTEM ===");

    // --- Create lost items ---
    println!("\n--- All Lost Items ---");
    let mut items = vec![
        LostItem::new(
            1,
            "Black laptop bag",
            12049,
            "A1",
            ItemCategory::Electronics,
        ),
        LostItem::new(2, "Blue denim jacket", 12049, "S3", ItemCategory::Clothing),
        LostItem::new(3, "Indian passport", 12007, "B2", ItemCategory::Document),
        LostItem::new(
            4,
            "Red suitcase with wheels",
            12213,
            "S1",
            ItemCategory::Luggage,
        ),
        LostItem::new(
            5,
            "Reading glasses in brown case",
            12007,
            "A1",
            ItemCategory::Other("eyewear".to_string()),
        ),
        LostItem::new(
            6,
            "Silver wristwatch",
            12578,
            "S5",
            ItemCategory::Electronics,
        ),
        LostItem::new(
            7,
            "Umbrella \u{2014} black folding",
            12049,
            "S3",
            ItemCategory::Other("accessory".to_string()),
        ),
    ];

    for item in &items {
        item.display();
    }

    // --- Search by ID using Option ---
    println!("\n--- Search by ID ---");
    match find_item_by_id(&items, 3) {
        Some(item) => println!("Looking for item #3: Found! {}", item.description),
        None => println!("Looking for item #3: Not found"),
    }
    match find_item_by_id(&items, 99) {
        Some(item) => println!("Looking for item #99: Found! {}", item.description),
        None => println!("Looking for item #99: Not found"),
    }

    // --- Search by train ---
    println!("\n--- Search by Train ---");
    let train_items = find_items_by_train(&items, 12049);
    println!("Items from train #12049:");
    if train_items.is_empty() {
        println!("  (none)");
    } else {
        for item in &train_items {
            println!("  - Item #{}: {}", item.id, item.description);
        }
    }

    let train_items = find_items_by_train(&items, 99999);
    println!("Items from train #99999:");
    if train_items.is_empty() {
        println!("  (none)");
    } else {
        for item in &train_items {
            println!("  - Item #{}: {}", item.id, item.description);
        }
    }

    // --- Parse categories using Result ---
    println!("\n--- Parse Categories ---");
    let inputs = [
        "electronics",
        "clothing",
        "document",
        "luggage",
        "",
        "eyewear",
    ];
    for input in inputs {
        match parse_item_category(input) {
            Ok(cat) => println!("  \"{input}\" \u{2192} Ok: {:?}", cat),
            Err(e) => println!("  \"{input}\" \u{2192} Error: {e}"),
        }
    }

    // --- Process claims using Result ---
    println!("\n--- Process Claims ---");
    match process_claim(&mut items, 3, "Giridhar") {
        Ok(msg) => println!("Claim item #3 for Giridhar: Ok \u{2014} {msg}"),
        Err(e) => println!("Claim item #3 for Giridhar: Error \u{2014} {e}"),
    }
    match process_claim(&mut items, 3, "Priya") {
        Ok(msg) => println!("Claim item #3 for Priya: Ok \u{2014} {msg}"),
        Err(e) => println!("Claim item #3 for Priya: Error \u{2014} {e}"),
    }
    match process_claim(&mut items, 99, "Arjun") {
        Ok(msg) => println!("Claim item #99 for Arjun: Ok \u{2014} {msg}"),
        Err(e) => println!("Claim item #99 for Arjun: Error \u{2014} {e}"),
    }

    // --- Quick description lookup using Option methods ---
    println!("\n--- Quick Description Lookup ---");
    println!("Item #1: {}", get_item_description(&items, 1));
    println!("Item #99: {}", get_item_description(&items, 99));

    // --- if let for quick lookups ---
    println!("\n--- if let Lookup ---");
    if let Some(item) = find_item_by_id(&items, 6) {
        println!(
            "Found item #{}: {} \u{2014} claiming it now",
            item.id, item.description
        );
    }

    // --- Daily report ---
    println!();
    daily_report(&items);
}
