fn main() {
    let mut val1 = 10;
    let mut val2 = 20;
    // Must is required on both left and right hand sides, because the value
    // of *borrowed will be changed later on.  If that was not the case, we'd
    // only need the mut on the lhs (since we're reassigning borrowed...)
    let mut borrowed = &mut val1;
    // "borrowed" is a mutable box, so must is needed here, otherwise:
    // mismatched types: expected `&mut <VI0>` but found `&<VI1>`
    borrowed = &mut val2;
    *borrowed = 11;
    println!("{:d}", *borrowed)
}
