// =============================================
// SOLUTION 03a - Ownership: Moves and Copies
// =============================================

// STEP 1
fn add_bang(mut s: String) -> String {
    s.push('!');
    s
}

// STEP 2
fn consume_len(s: String) -> usize {
    s.len()
}

// STEP 3
fn swap_pair(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

// STEP 4
fn sum_speeds(speeds: [u32; 4]) -> u32 {
    let mut total = 0;
    for s in speeds {
        total += s;
    }
    total
}

fn label_len(pair: (u32, String)) -> usize {
    pair.1.len()
}

// STEP 5
fn twins(s: &String) -> (String, String) {
    (s.clone(), s.clone())
}

// STEP 6
fn collect_names(a: String, b: String, c: String) -> Vec<String> {
    vec![a, b, c]
}

// STEP 7
fn longest_owned(names: Vec<String>) -> Option<String> {
    let mut best: Option<String> = None;
    for name in names {
        match &best {
            Some(b) if b.len() >= name.len() => {}
            _ => best = Some(name),
        }
    }
    best
}

fn take_second(names: &mut Vec<String>) -> Option<String> {
    if names.len() >= 2 {
        Some(names.remove(1))
    } else {
        None
    }
}

// STEP 8
fn join_all(parts: Vec<String>) -> String {
    let mut acc = String::new();
    for p in parts {
        if !acc.is_empty() {
            acc.push('+');
        }
        acc.push_str(&p);
    }
    acc
}

// STEP 9
#[derive(Debug, Clone, Copy, PartialEq)]
struct Platform(u8);

#[derive(Debug, Clone)]
struct Train {
    name: String,
    platform: Platform,
}

fn reassign(train: Train, platform: Platform) -> Train {
    Train { platform, ..train }
}

fn describe_train(t: Train) -> String {
    format!("{} @ {:?}", t.name, t.platform)
}

// STEP 10
struct Ticket {
    passenger: String,
    route: String,
    seat: u32,
}

fn split_ticket(t: Ticket) -> (String, String, u32) {
    let passenger = t.passenger;
    let route = t.route;
    (passenger, route, t.seat)
}

fn passenger_only(t: Ticket) -> (String, Ticket) {
    let who = t.passenger;
    let rest = Ticket {
        passenger: String::from("(transferred)"),
        route: t.route,
        seat: t.seat,
    };
    (who, rest)
}

// STEP 11
struct Coach(&'static str);

impl Drop for Coach {
    fn drop(&mut self) {
        println!("  dropped coach {}", self.0);
    }
}

fn inspect(coach: Coach) {
    println!("  inspecting {}", coach.0);
}

fn drop_demo() {
    let _a = Coach("A");
    let b = Coach("B");
    let _c = Coach("C");
    inspect(b);
    println!("  end of drop_demo");
}

// STEP 12
fn make_announcer(prefix: String) -> impl Fn(&str) -> String {
    move |train| format!("{prefix}: {train}")
}

// STEP 13
fn main() {
    println!("=== OWNERSHIP: MOVES AND COPIES ===\n");

    println!("--- Move In, Move Out ---");
    let name = String::from("Rajdhani");
    let name = add_bang(name);
    println!("  {name}");
    let n = consume_len(name);
    println!("  consumed, len was {n}");

    println!("\n--- Copy Types ---");
    let (x, y) = (3, 9);
    let (p, q) = swap_pair(x, y);
    println!("  x={x} y={y} -> p={p} q={q}");
    let speeds = [100, 120, 150, 90];
    let total = sum_speeds(speeds);
    println!("  total={total}, array still usable: {speeds:?}");
    let pair = (7, String::from("Express"));
    println!("  label_len={}", label_len(pair));

    println!("\n--- Clone ---");
    let original = String::from("Chennai");
    let (a, b) = twins(&original);
    println!("  {original} {a} {b}");

    println!("\n--- Moving Into and Out of Vec ---");
    let mut names = collect_names("Anand".to_string(), "Bala".to_string(), "Chitra".to_string());
    println!("  {names:?}");
    println!("  second: {:?}", take_second(&mut names));
    println!("  left: {names:?}");
    println!("  longest: {:?}", longest_owned(names));
    println!("  longest of none: {:?}", longest_owned(Vec::new()));

    println!("\n--- Accumulator ---");
    let parts = vec!["Chennai".to_string(), "Katpadi".to_string(), "Salem".to_string()];
    println!("  {}", join_all(parts));

    println!("\n--- Copy vs Clone Structs ---");
    let train = Train { name: "Duronto".to_string(), platform: Platform(3) };
    let spare = train.platform;
    let moved = reassign(train.clone(), Platform(5));
    println!("  original platform {:?}, copied {:?}", train.platform, spare);
    println!("  {}", describe_train(moved));
    println!("  {}", describe_train(train));

    println!("\n--- Partial Moves ---");
    let ticket = Ticket { passenger: "Devi".to_string(), route: "MAS-CBE".to_string(), seat: 42 };
    let (who, route, seat) = split_ticket(ticket);
    println!("  {who} / {route} / seat {seat}");
    let ticket = Ticket { passenger: "Esha".to_string(), route: "MAS-MDU".to_string(), seat: 7 };
    let (who, rest) = passenger_only(ticket);
    println!("  {who} -> {} / {} / seat {}", rest.passenger, rest.route, rest.seat);

    println!("\n--- Drop Order ---");
    drop_demo();
    let late = Coach("Z");
    println!("  explicit drop next");
    drop(late);
    println!("  after drop");

    println!("\n--- Closures That Own ---");
    let announce = make_announcer("Now arriving".to_string());
    println!("  {}", announce("Shatabdi"));
    println!("  {}", announce("Vande Bharat"));
}
