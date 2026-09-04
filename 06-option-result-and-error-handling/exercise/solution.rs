// ============================================================================
// LESSON 06 EXERCISE — SOLUTION: Railway Lost & Found System
// ============================================================================

// TODO 1: Define the `ItemCategory` enum
#[derive(Debug, Clone)]
enum ItemCategory {
    Electronics,
    Clothing,
    Document,
    Luggage,
    Other(String),
}

// TODO 2: Define the `LostItem` struct
#[derive(Debug)]
struct LostItem {
    id: u32,
    description: String,
    train_number: u32,
    coach: String,
    category: ItemCategory,
    claimed: bool,
}

// TODO 3: Implement `LostItem` methods
impl LostItem {
    fn new(
        id: u32,
        description: &str,
        train_number: u32,
        coach: &str,
        category: ItemCategory,
    ) -> Self {
        LostItem {
            id,
            description: description.to_string(),
            train_number,
            coach: coach.to_string(),
            category,
            claimed: false,
        }
    }

    fn display(&self) {
        let category_str = match &self.category {
            ItemCategory::Electronics => String::from("Electronics"),
            ItemCategory::Clothing => String::from("Clothing"),
            ItemCategory::Document => String::from("Document"),
            ItemCategory::Luggage => String::from("Luggage"),
            ItemCategory::Other(s) => format!("Other: {s}"),
        };
        let status = if self.claimed { "Claimed" } else { "Unclaimed" };
        println!(
            "Item #{}: {} (Train #{}, Coach {}) [{}] \u{2014} {}",
            self.id, self.description, self.train_number, self.coach, category_str, status
        );
    }

    fn claim(&mut self) {
        self.claimed = true;
    }
}

// TODO 4: Implement `find_item_by_id`
fn find_item_by_id(items: &[LostItem], id: u32) -> Option<&LostItem> {
    items.iter().find(|item| item.id == id)
}

// TODO 5: Implement `find_items_by_train`
fn find_items_by_train(items: &[LostItem], train_number: u32) -> Vec<&LostItem> {
    items
        .iter()
        .filter(|item| item.train_number == train_number)
        .collect()
}

// TODO 6: Implement `parse_item_category`
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

// TODO 7: Implement `process_claim`
fn process_claim(items: &mut [LostItem], item_id: u32, owner_name: &str) -> Result<String, String> {
    let item = items
        .iter_mut()
        .find(|item| item.id == item_id)
        .ok_or(format!("Item #{item_id} not found in the system"))?;

    if item.claimed {
        return Err(format!("Item #{item_id} has already been claimed"));
    }

    item.claim();
    Ok(format!("Item #{item_id} claimed by {owner_name}"))
}

// TODO 8: Implement `get_item_description`
fn get_item_description(items: &[LostItem], id: u32) -> String {
    find_item_by_id(items, id)
        .map(|item| item.description.clone())
        .unwrap_or_else(|| format!("No item with id #{id}"))
}

// TODO 9: Implement `daily_report`
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

// ============================================================================
// main() — identical to the exercise file
// ============================================================================

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
