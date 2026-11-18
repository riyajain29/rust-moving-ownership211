# Moving Ownership in Rust

## Introduction

Ownership is Rust's system for managing memory safely without needing a garbage collector. Every value has an owner, and when ownership is moved from one variable to another, the original variable can no longer access that value. I chose this topic because I wanted to better understand how Rust manages memory behind the scenes and how its approach prevents common memory-related bugs that can occur in languages like C. 

## What I explored

I set out to answer several questions about how ownership works in practice:
- What actually happens in memory when I write let s2 = s1;? Does Rust copy the underlying data or transfer ownership?
- Why can I no longer use s1 after moving its value into s2?
- How does moving ownership in Rust compare to copying a pointer in C?
- How does Rust determine whether a value should be copied or moved?
- What happens when a value is moved multiple times, particularly when it comes to freeing memory?
- Why does Rust prevent certain operations, such as moving a value directly out of a vector?
- Can C programs with similar memory-management problems compile successfully, and what errors can AddressSanitizer detect at runtime?

## Most interesting discoveries

1. Before running the experiments, I assumed that assigning a variable to another variable might create a completely new copy of the underlying data. However, Experiment 3 showed that when moving a String, its heap allocation stays at the same memory address. Instead of duplicating the heap data, Rust transfers ownership of the existing allocation to the new variable. This means moving can be very efficient because it avoids unnecessary memory copying.

2. Initially, I thought that once a variable was moved, its value might be deleted or become inaccessible altogether. Instead, the value remains alive and is accessible through its new owner. Rust prevents access through the original variable to ensure that there is always a clear owner responsible for the memory. This helped me understand that moving is about transferring responsibility, not destroying data.

3. Experiment 5 helped me understand how Rust prevents double-free errors. I initially wondered whether both the original and new variables would eventually try to free the same memory. However, Rust ensures that only the variable that currently owns the value is responsible for dropping it. Even if ownership moves multiple times, the value is dropped exactly once when its final owner goes out of scope. This is important because attempting to free the same memory twice can cause serious memory errors.

4. Experiment 7 demonstrated that Rust rejects let s = v[0]; when the vector contains String values. I initially expected this to work similarly to accessing an element in an array. However, allowing this operation would move the string out of the vector and leave an invalid or partially empty element behind. Since the vector still needs to maintain ownership of its elements, Rust prevents this. Instead, I would need to borrow the element or explicitly remove it using an appropriate method.

5. One of the biggest differences I noticed was that the C programs could compile even when they contained unsafe memory operations, such as using a pointer after freeing its memory. However, running them with AddressSanitizer exposed issues like use-after-free and double-free errors. In contrast, Rust's ownership system catches many of these problems before the program even runs. This showed me how Rust shifts much of memory safety from runtime debugging to compile-time enforcement.

## Rust vs. C

The biggest difference between Rust and C is how each language handles memory ownership and prevents memory-related bugs. Rust enforces strict ownership rules at compile time, while C gives programmers much more manual control over memory management.

Overall, exploring moving ownership helped me understand that Rust's ownership system is more than just a way to manage memory. It establishes clear rules about who is responsible for data at any given time. While C provides greater flexibility by allowing programmers to manage pointers directly, Rust uses compile-time restrictions to prevent entire categories of memory-safety bugs. The experiments helped me see why these restrictions exist and how they make Rust both efficient and safer.

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



## Resources

- [The Rust Programming Language, ch. 4.1: What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust by Example: Ownership and moves](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Rust error index: E0382 (use of moved value)](https://doc.rust-lang.org/error_codes/E0382.html)
- [Rust error index: E0507 (cannot move out of borrowed content)](https://doc.rust-lang.org/error_codes/E0507.html)
- [`Copy` trait documentation](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [`Drop` trait documentation](https://doc.rust-lang.org/std/ops/trait.Drop.html)
- [AddressSanitizer (Clang docs)](https://clang.llvm.org/docs/AddressSanitizer.html)
