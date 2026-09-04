# 🚂 Learn Rust — Zero to Hero

A hands-on Rust course with railway-themed exercises, building toward programming an **ESP32-S3 development board**. Each lesson introduces core Rust concepts through practical examples and exercises — all aboard!

## Repository Structure

Each lesson follows a consistent layout:

```
XX-lesson-name/
├── lesson.md          # Concept explanation & notes
├── examples/          # Runnable example programs
│   └── src/main.rs
└── exercise/          # Your workspace — TODOs to fill in
    └── src/main.rs
```

- **`lesson.md`** — Read this first. It covers the theory and key points.
- **`examples/`** — Fully working programs you can run with `cargo run`.
- **`exercise/`** — Starter code with `TODO` comments for you to complete.
- **`solution.rs`** — The reference solution. Peek only if you're stuck!

## Curriculum

| #   | Lesson | Topic |
|-----|--------|-------|
| 01  | `01-variables-and-types` | Variables & Types |
| 02  | `02-functions-and-control-flow` | Functions & Control Flow |
| 02β | `02-bonus-loops-practice` | Loops Practice (bonus) |
| 03  | `03-ownership-and-borrowing` | Ownership & Borrowing |
| 04  | `04-structs-and-methods` | Structs & Methods |
| 05  | `05-enums-and-pattern-matching` | Enums & Pattern Matching |
| 06  | *Coming soon* | Collections |
| 07  | *Coming soon* | Error Handling |
| 08  | *Coming soon* | Traits & Generics |

## Prerequisites

Install Rust via [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify your installation:

```sh
rustc --version
cargo --version
```

## How to Use

For each lesson:

1. **Read** `lesson.md` to understand the concepts.
2. **Run the examples** to see them in action:
   ```sh
   cd 01-variables-and-types/examples
   cargo run
   ```
3. **Do the exercise** — open the exercise's `src/main.rs` and fill in the `TODO`s:
   ```sh
   cd 01-variables-and-types/exercise
   cargo run
   ```
4. **Check your work** — if it compiles and runs correctly, you're on track! Compare with `solution.rs` if needed.

## Resetting Exercises

Made a mess? No worries. Use the `reset.sh` script to restore exercises to their original state:

```sh
# Reset ALL exercises back to their clean templates
./reset.sh

# Reset just lesson 01
./reset.sh 01

# Reset specific lessons
./reset.sh 01 03 04
```

The script copies clean templates from `.templates/` back into each exercise's `src/main.rs`.

## Target Hardware

This course builds toward programming the **Freenove ESP32-S3 CYD** — a development board featuring:

- ESP32-S3 microcontroller
- 3.5" IPS touch display (320×480)
- WiFi + Bluetooth connectivity

The early lessons focus on core Rust fundamentals. Later lessons will introduce embedded programming concepts targeting this board.
