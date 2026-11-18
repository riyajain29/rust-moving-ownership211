# Moving Ownership in Rust

> ✏️ **Write these sections yourself.** Everything marked ✏️ is a prompt for you to answer
> in your own words. Delete the prompts when you're done. The "How to run" and
> "Repository layout" sections are already filled in.

## Introduction

✏️ *In 2–4 sentences: what is ownership in Rust, and what does "moving" ownership mean?
Why did you pick this topic?*

## What I explored

✏️ *Briefly list the questions you set out to answer. For example:*
- *What actually happens in memory when I write `let s2 = s1;`?*
- *Why can't I use `s1` afterward?*
- *How does this compare to copying a pointer in C?*
- *When does Rust copy instead of move?*

## Most interesting discoveries

✏️ *Pick 3–5 things that surprised you or clicked for you while running the experiments.
For each, say what you expected, what actually happened, and why. Some places to look:*
- *Experiment 3 prints heap addresses. What do they show about whether a move copies data?*
- *Experiment 5 shows when `drop` runs. How many times does a moved value get dropped?*
- *Experiment 7: why does Rust refuse `let s = v[0];`, and what would go wrong if it allowed it?*
- *The C programs compile with no errors. What does AddressSanitizer report when you run them?*

## Rust vs. C

✏️ *A short comparison in your own words (a table works well): who frees memory, what
happens when you copy a pointer or move a value, and when bugs get caught.*

## How to run

### Rust experiments

Requires [Rust](https://www.rust-lang.org/tools/install) (installs `cargo`).

```bash
cd examples
cargo run --bin 01_basic_move
```

| Experiment | Command | What it shows |
|---|---|---|
| 1. Basic move | `cargo run --bin 01_basic_move` | Using a value after it moves is a compile error (E0382) |
| 2. Copy types | `cargo run --bin 02_copy_types` | `i32`, `bool`, `char` copy instead of move |
| 3. Clone | `cargo run --bin 03_clone` | `clone()` allocates new heap memory; a move doesn't |
| 4. Functions | `cargo run --bin 04_functions` | Moving into a function, giving ownership back, and borrowing |
| 5. Drop order | `cargo run --bin 05_drop_order` | Exactly when a value is freed, and that it's freed only once |
| 6. Move in a loop | `cargo run --bin 06_move_in_loop` | A value can only be moved once, even in a loop |
| 7. Move out of a Vec | `cargo run --bin 07_move_out_of_vec` | Why `v[0]` can't be moved out (E0507), and four fixes |

Several files have a line marked **`UNCOMMENT to see the error`**. Uncomment it and run
again to see the compiler reject the code.

You can also paste any file into the [Rust Playground](https://play.rust-lang.org) instead of installing Rust.

### C comparison

Requires `gcc` (or `clang`). AddressSanitizer reports the memory bug at runtime:

```bash
cd c-comparison
gcc -fsanitize=address -g double_free.c -o double_free.out && ./double_free.out
gcc -fsanitize=address -g use_after_free.c -o use_after_free.out && ./use_after_free.out
```

Both programs compile without any errors. The bugs only show up when they run.

## Repository layout

```
.
├── README.md
├── examples/            Rust experiments (one binary per file in src/bin/)
├── c-comparison/        The same bugs written in C
└── slides/              Presentation slides
```

## Presentation

✏️ *Link your slides here, e.g. [Slides (PDF)](slides/moving-ownership.pdf)*

## Resources

- [The Rust Programming Language, ch. 4.1: What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust by Example: Ownership and moves](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Rust error index: E0382 (use of moved value)](https://doc.rust-lang.org/error_codes/E0382.html)
- [Rust error index: E0507 (cannot move out of borrowed content)](https://doc.rust-lang.org/error_codes/E0507.html)
- [`Copy` trait documentation](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [`Drop` trait documentation](https://doc.rust-lang.org/std/ops/trait.Drop.html)
- [AddressSanitizer (Clang docs)](https://clang.llvm.org/docs/AddressSanitizer.html)

✏️ *Add anything else you actually used: videos, blog posts, class notes, and a note on how
you used AI to help you understand the topic if your course asks for that.*
