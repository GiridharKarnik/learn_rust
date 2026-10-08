// =============================================
// EXAMPLES 08: Tuples and Ranges
// =============================================
//
// Run with: cargo run
//
// These examples demonstrate tuple literals/access/destructuring and the
// Range/RangeInclusive family, using a railway theme. Read through them,
// then try the exercise!

use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Stop(String, u8, u32); // tuple struct recap: name, platform, speed limit

fn main() {
    // =========================================
    // 1. TUPLE LITERALS AND ACCESS
    // =========================================
    println!("=== 1. Tuple Literals and Access ===\n");

    let stop: (&str, u8, u32) = ("Chennai Central", 3, 80);
    println!("Station:      {}", stop.0);
    println!("Platform:     {}", stop.1);
    println!("Speed limit:  {} km/h", stop.2);

    // =========================================
    // 2. DESTRUCTURING
    // =========================================
    println!("\n=== 2. Destructuring ===\n");

    let (name, platform, speed_limit) = ("Katpadi Junction", 1, 100);
    println!("{name} is on platform {platform}, speed limit {speed_limit} km/h");

    // Destructuring in function parameters
    fn describe_stop((name, platform, speed): (&str, u8, u32)) -> String {
        format!("{name} (platform {platform}) \u{2014} limit {speed} km/h")
    }
    println!("{}", describe_stop(("Jolarpettai", 2, 90)));

    // =========================================
    // 3. TUPLES AS MULTI-VALUE RETURNS
    // =========================================
    println!("\n=== 3. Tuples as Multi-Value Returns ===\n");

    fn min_max(speeds: &[u32]) -> (u32, u32) {
        let min = *speeds.iter().min().unwrap();
        let max = *speeds.iter().max().unwrap();
        (min, max)
    }

    let speeds = [80, 130, 110, 160, 95];
    let (slowest, fastest) = min_max(&speeds);
    println!("Slowest: {slowest} km/h, Fastest: {fastest} km/h");

    // =========================================
    // 4. TUPLE STRUCT RECAP (Lesson 04)
    // =========================================
    println!("\n=== 4. Tuple Struct Recap ===\n");

    let platform_a = Stop("Salem Junction".to_string(), 4, 70);
    println!(
        "{} \u{2014} platform {}, limit {} km/h",
        platform_a.0, platform_a.1, platform_a.2
    );

    // =========================================
    // 5. RANGE LITERAL FORMS
    // =========================================
    println!("\n=== 5. Range Literal Forms ===\n");

    let exclusive = 1..5;
    let inclusive = 1..=5;
    println!("1..5   -> start={} end={}", exclusive.start, exclusive.end);
    println!(
        "1..=5  -> start={} end={}",
        inclusive.start(),
        inclusive.end()
    );
    println!("1..5 collects to:  {:?}", (1..5).collect::<Vec<_>>());
    println!("1..=5 collects to: {:?}", (1..=5).collect::<Vec<_>>());

    // =========================================
    // 6. RANGES AS ITERATORS
    // =========================================
    println!("\n=== 6. Ranges as Iterators ===\n");

    print!("step_by(5) over 0..20: ");
    for n in (0..20).step_by(5) {
        print!("{n} ");
    }
    println!();

    print!("rev() over 1..=5:      ");
    for n in (1..=5).rev() {
        print!("{n} ");
    }
    println!();

    let doubled: Vec<i32> = (1..5).map(|n| n * 2).collect();
    println!("(1..5).map(|n| n * 2).collect() = {doubled:?}");

    // =========================================
    // 7. RANGE AS A FUNCTION PARAMETER
    // =========================================
    println!("\n=== 7. Range as a Function Parameter ===\n");

    fn sum_range(r: RangeInclusive<u32>) -> u32 {
        r.sum()
    }
    println!("sum_range(1..=100) = {}", sum_range(1..=100));

    // =========================================
    // 8. RANGE FOR SLICE INDEXING (recap of Lesson 07)
    // =========================================
    println!("\n=== 8. Range for Slice Indexing ===\n");

    let stations = vec!["Chennai", "Katpadi", "Jolarpettai", "Salem", "Erode"];
    println!("stations[1..4] = {:?}", &stations[1..4]);
    println!("stations[..2]  = {:?}", &stations[..2]);
    println!("stations[3..]  = {:?}", &stations[3..]);

    // =========================================
    // 9. RANGES IN MATCH GUARDS
    // =========================================
    println!("\n=== 9. Ranges in Match Guards ===\n");

    fn classify_speed(speed_kmh: u32) -> &'static str {
        match speed_kmh {
            0..=59 => "slow",
            60..=120 => "moderate",
            121..=200 => "fast",
            _ => "very fast",
        }
    }

    for speed in [40, 90, 150, 250] {
        println!("{speed} km/h is classified as: {}", classify_speed(speed));
    }

    // =========================================
    // SUMMARY
    // =========================================
    println!("\n=== Summary ===");
    println!("Tuples: fixed-size, mixed-type groupings. Access with .0, .1, .2.");
    println!("Ranges: real values (Range<T> / RangeInclusive<T>), not just loop syntax.");
    println!("Ranges implement Iterator, work as fn params, and as match patterns.");
    println!("Match patterns require ..= (inclusive) \u{2014} plain .. is a syntax error there.");
}
