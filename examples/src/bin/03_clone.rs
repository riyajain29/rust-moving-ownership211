// Experiment 3: clone() makes an independent copy
//
// Question: How do I keep using the original after giving a copy away?
//
// Run:  cargo run --bin 03_clone

fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1.clone(); // deep copy: new heap allocation

    s2.push_str(", world");

    // Both are valid, and they are separate values with separate heap memory.
    println!("s1 = {s1}");
    println!("s2 = {s2}");

    // Compare the heap addresses: they are different.
    println!("s1 heap addr = {:p}", s1.as_ptr());
    println!("s2 heap addr = {:p}", s2.as_ptr());

    // Compare with a move: the heap address stays the same.
    let s3 = String::from("moved");
    let before = s3.as_ptr();
    let s4 = s3;
    println!("before move = {:p}, after move = {:p}", before, s4.as_ptr());
}
