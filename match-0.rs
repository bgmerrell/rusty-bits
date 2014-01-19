fn main() {
    // TODO: pass in these values from the command line
    let x = (48, true);

    match x {
        (20..26, true) => println("True and in range"),
        (_, true) => println("True and out of range"),
        (40..49, _) => println("Between 40 and 49"),
        (_, _) => println("Unknown")
    }
}
