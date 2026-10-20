// =============================================
// SOLUTION 03b - Borrowing Drills
// =============================================

// STEP 1
fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

// STEP 2
fn shout(s: &mut String) {
    *s = s.to_uppercase();
    s.push('!');
}

// STEP 3
fn total_len(a: &str, b: &str, c: &str) -> usize {
    a.len() + b.len() + c.len()
}

// STEP 4
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// STEP 5
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        b
    } else {
        a
    }
}

// STEP 6
fn find_station<'a>(stations: &'a [String], prefix: &str) -> Option<&'a String> {
    stations.iter().find(|s| s.starts_with(prefix))
}

fn last_station(stations: &[String]) -> Option<&str> {
    stations.last().map(|s| s.as_str())
}

// STEP 7
fn bump_all(speeds: &mut [u32], by: u32) {
    for s in speeds.iter_mut() {
        *s += by;
    }
}

fn fastest_mut(speeds: &mut [u32]) -> Option<&mut u32> {
    let mut best: Option<&mut u32> = None;
    for s in speeds.iter_mut() {
        match &best {
            Some(b) if **b >= *s => {}
            _ => best = Some(s),
        }
    }
    best
}

// STEP 8
fn describe(label: &str, values: &[u32]) -> String {
    format!("{label}: {values:?}")
}

// STEP 9
fn rename_in_place(names: &mut Vec<String>, index: usize, new_name: &str) -> bool {
    match names.get_mut(index) {
        Some(slot) => {
            slot.clear();
            slot.push_str(new_name);
            true
        }
        None => false,
    }
}

// STEP 10
struct Train {
    name: String,
    stops: Vec<String>,
}

impl Train {
    fn name(&self) -> &str {
        &self.name
    }
    fn stops(&self) -> &[String] {
        &self.stops
    }
    fn add_stop(&mut self, stop: &str) {
        self.stops.push(stop.to_string());
    }
    fn first_stop(&self) -> Option<&String> {
        self.stops.first()
    }
    fn last_stop_mut(&mut self) -> Option<&mut String> {
        self.stops.last_mut()
    }
    fn into_name(self) -> String {
        self.name
    }
}

// STEP 11
fn split_first_two(items: &mut [i32]) -> (&mut i32, &mut i32) {
    let (a, rest) = items.split_at_mut(1);
    (&mut a[0], &mut rest[0])
}

fn swap_ends(items: &mut [i32]) {
    if items.len() >= 2 {
        let last = items.len() - 1;
        items.swap(0, last);
    }
}

// STEP 12
fn scoped_borrow_demo() -> String {
    let mut log = String::from("start");
    let len_before = {
        let r = &log;
        r.len()
    };
    log.push_str("-mid");
    let r1 = &log;
    let r2 = &log;
    let seen = format!("{}/{}", r1.len(), r2.len());
    log.push_str("-end");
    format!("{len_before} {seen} {log}")
}

// STEP 13
fn main() {
    println!("=== BORROWING DRILLS ===\n");

    println!("--- Shared Borrows ---");
    let title = String::from("Vande Bharat Express");
    println!("  vowels: {}", count_vowels(&title));
    println!("  vowels in literal: {}", count_vowels("Shatabdi"));
    println!("  total: {}", total_len(&title, &title, "ab"));
    println!("  title still mine: {title}");

    println!("\n--- Exclusive Borrow ---");
    let mut label = String::from("duronto");
    shout(&mut label);
    println!("  {label}");

    println!("\n--- Returned References ---");
    println!("  first word: {}", first_word(&title));
    println!("  first word of empty: '{}'", first_word("   "));
    let a = String::from("Chennai");
    let result;
    {
        let b = String::from("Katpadi");
        result = longer(&a, &b).to_string();
    }
    println!("  longer: {result}");

    println!("\n--- Searching Borrowed Data ---");
    let stations: Vec<String> = ["Chennai", "Katpadi", "Salem", "Coimbatore"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    match find_station(&stations, "Sa") {
        Some(s) => println!("  found {s}"),
        None => println!("  nothing"),
    }
    println!("  find 'Zz': {:?}", find_station(&stations, "Zz"));
    println!("  last: {:?}", last_station(&stations));
    println!("  last of none: {:?}", last_station(&[]));

    println!("\n--- Mutating Through Slices ---");
    let mut speeds = vec![80, 120, 100];
    bump_all(&mut speeds, 10);
    println!("  {}", describe("bumped", &speeds));
    if let Some(top) = fastest_mut(&mut speeds) {
        *top = 0;
    }
    println!("  {}", describe("fastest zeroed", &speeds));
    println!("  fastest of none: {:?}", fastest_mut(&mut []));

    println!("\n--- Renaming Through get_mut ---");
    let mut names = stations.clone();
    println!("  renamed 1: {}", rename_in_place(&mut names, 1, "Jolarpettai"));
    println!("  renamed 9: {}", rename_in_place(&mut names, 9, "Nowhere"));
    println!("  {names:?}");

    println!("\n--- Methods: &self, &mut self, self ---");
    let mut train = Train { name: "Kovai Express".to_string(), stops: Vec::new() };
    train.add_stop("Chennai");
    train.add_stop("Salem");
    train.add_stop("Coimbatore");
    println!("  {} has {} stops", train.name(), train.stops().len());
    println!("  first stop: {:?}", train.first_stop());
    if let Some(last) = train.last_stop_mut() {
        last.push_str(" Junction");
    }
    println!("  stops: {:?}", train.stops());
    let owned = train.into_name();
    println!("  consumed train, kept name: {owned}");

    println!("\n--- Two Mutable Borrows From One Slice ---");
    let mut nums = [1, 2, 3, 4];
    {
        let (x, y) = split_first_two(&mut nums);
        std::mem::swap(x, y);
        *x += 100;
    }
    println!("  {nums:?}");
    swap_ends(&mut nums);
    println!("  {nums:?}");

    println!("\n--- Borrow Scopes (NLL) ---");
    println!("  {}", scoped_borrow_demo());
    let mut v = vec![1, 2, 3];
    let first = v[0];
    v.push(first);
    let last = v.last().copied();
    v.push(9);
    println!("  {v:?} last was {last:?}");
}
