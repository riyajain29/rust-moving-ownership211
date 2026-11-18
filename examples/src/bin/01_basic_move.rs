// Experiment 1: Basic move
//
// Question: What happens to `s1` after `let s2 = s1;`?
//
// Run:  cargo run --bin 01_basic_move
//
//  uncomment the marked line and run again to see the compiler error (E0382).

fn main() {
    let s1 = String::from("hello");

    // The String's stack data (pointer, length, capacity) is copied into s2.
    // The heap data ("hello") is NOT copied. s1 is now considered moved.
    let s2 = s1;

    println!("s2 = {s2}");

    // UNCOMMENT to see the error:
    // println!("s1 = {s1}");
    //
    // Expected: error[E0382]: borrow of moved value: `s1`
}
