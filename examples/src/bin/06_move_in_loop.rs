// Experiment 6: Moving inside a loop
//
// Question: What happens if a value is moved on the first loop iteration?
//
// Run:  cargo run --bin 06_move_in_loop
//
// Then swap in the commented-out loop to see the error.

fn consume(s: String) {
    println!("consumed: {s}");
}

fn main() {
    let s = String::from("loop value");

    // Works: borrow on every iteration.
    for i in 0..3 {
        println!("iteration {i}: {}", &s);
    }

    // UNCOMMENT to see the error:
    // for _ in 0..3 {
    //     consume(s);
    // }
    //
    // Expected: error[E0382]: use of moved value: `s`
    //           (points at consume(s): "value moved here, in previous iteration of loop")

    consume(s); // fine: moved exactly once
}
