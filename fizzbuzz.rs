use std::iter;

fn main() {
    for i in iter::range_inclusive(1, 100) {
        match (i % 3, i % 5) {
            (0, 0) => println!("{}: FizzBuzz", i),
            (0, _) => println!("{}: Fizz", i),
            (_, 0) => println!("{}: Buzz", i),
            (_, _) => {}
        }
    }
}
