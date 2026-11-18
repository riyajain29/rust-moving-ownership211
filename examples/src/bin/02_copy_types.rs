// Experiment 2: Copy types don't move
//
// Question: Why does assigning an i32 NOT invalidate the original,
// when assigning a String does?
//
// Run:  cargo run --bin 02_copy_types

fn main() {
    // i32 implements the Copy trait: assignment duplicates the bits.
    let x: i32 = 5;
    let y = x;
    println!("x = {x}, y = {y}"); // both still usable

    // Other Copy types: bool, char, f64, and tuples/arrays made only of Copy types.
    let a = (1, true, 'r');
    let b = a;
    println!("a = {a:?}, b = {b:?}");

    // A tuple that contains a String is NOT Copy, so it moves.
    let t1 = (1, String::from("owned"));
    let t2 = t1;
    println!("t2 = {t2:?}");

    // UNCOMMENT to see the error:
    // println!("t1 = {t1:?}");
}
