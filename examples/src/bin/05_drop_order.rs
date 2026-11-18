// Experiment 5: Watching exactly when a value is dropped
//
// Question: When ownership moves, when does the memory actually get freed?
//
// Implementing the Drop trait lets us print a message at the moment
// Rust cleans up a value.
//
// Run:  cargo run --bin 05_drop_order

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  dropping {}", self.0);
    }
}

fn consume(n: Noisy) {
    println!("  inside consume({})", n.0);
} // n is dropped here

fn main() {
    println!("1) Value dropped at end of its scope:");
    {
        let _a = Noisy("a");
        println!("  end of inner scope");
    }

    println!("2) Moved value is dropped by its NEW owner, only once:");
    {
        let b = Noisy("b");
        let _b2 = b; // ownership moves to _b2
        println!("  end of inner scope (watch: 'b' drops only once)");
    }

    println!("3) Moved into a function: dropped when the function ends:");
    let c = Noisy("c");
    consume(c);
    println!("  back in main");

    println!("4) Explicit early drop with drop():");
    let d = Noisy("d");
    drop(d); // drop() just takes ownership and lets it go out of scope
    println!("  after drop(d)");

    println!("end of main");
}
