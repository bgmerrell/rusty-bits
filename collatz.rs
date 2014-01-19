// Take the number of collatz steps as input, and return the minimum number
// that can be solved using the collatz conjecture in that number of steps as
// output.

use std::os;

fn main() {
    if os::args().len() < 2 {
        println("Error: Please provide a number of steps as an argument.");
        return;
    }

    let n_steps = from_str::<int>(os::args()[1]).unwrap();
    let mut i = 0;
    
    loop {
        i += 1;
        if collatz(i) == n_steps {
            println!("{} was the lowest number taking {} steps", i, n_steps)
            return
        }
    }
}

// Causes a stack overflow for certain input, e.g., 9999999999999999
fn collatz(N: int) -> int {
    if N == 1 { return 0; }
    match N % 2 {
        0 => { 1 + collatz(N/2) }
        _ => { 1 + collatz(N*3+1) }
    }
}
