// Experiment 4: Moves into and out of functions
//
// Question: What happens to a value passed to a function?
//
// Run:  cargo run --bin 04_functions

fn takes_ownership(s: String) {
    println!("takes_ownership got: {s}");
} // s goes out of scope here and the String is dropped (memory freed)

fn takes_and_gives_back(s: String) -> String {
    println!("takes_and_gives_back got: {s}");
    s // ownership moves back out to the caller
}

fn borrows(s: &String) {
    println!("borrows got: {s}");
} // nothing is dropped: borrows never owned the String

fn main() {
    // 1) Move into a function
    let a = String::from("A");
    takes_ownership(a);
    // UNCOMMENT to see the error:
    // println!("{a}");

    // 2) Move in, then move back out
    let b = String::from("B");
    let b = takes_and_gives_back(b);
    println!("still have b: {b}");

    // 3) Borrow instead of moving
    let c = String::from("C");
    borrows(&c);
    println!("still have c: {c}");

    // 4) Copy types are copied into functions, not moved
    let n = 42;
    print_number(n);
    println!("still have n: {n}");
}

fn print_number(n: i32) {
    println!("print_number got: {n}");
}
