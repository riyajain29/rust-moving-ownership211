// Experiment 7: You can't move out of a Vec by index
//
// Question: Why does `let s = v[0];` fail for a Vec<String>?
//
// If it were allowed, both `s` and the Vec would think they own the same
// String, and it would be freed twice when both go out of scope.
//
// Run:  cargo run --bin 07_move_out_of_vec

fn main() {
    let mut v = vec![
        String::from("first"),
        String::from("second"),
        String::from("third"),
    ];

    // UNCOMMENT to see the error:
    // let s = v[0];
    //
    // Expected: error[E0507]: cannot move out of index of `Vec<String>`

    // Fix 1: borrow instead of moving
    let r = &v[0];
    println!("borrowed: {r}");

    // Fix 2: clone
    let c = v[0].clone();
    println!("cloned: {c}");

    // Fix 3: actually remove it from the Vec (the Vec gives up ownership)
    let removed = v.remove(0);
    println!("removed: {removed}, vec is now {v:?}");

    // Fix 4: swap in an empty String and take the old one
    let taken = std::mem::take(&mut v[0]);
    println!("taken: {taken:?}, vec is now {v:?}");

    // Moving the whole Vec moves every String inside it at once
    let v2 = v;
    println!("v2 = {v2:?}");
}
