fn main() {
    let x = ~10;
    let mut y = x.clone();
    println!("x: {:d}", *x);
    *y = 13;
    println!("y: {:d}", *y);
}
