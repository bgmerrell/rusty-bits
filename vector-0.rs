// Exercise 2.1
fn increment(numbers : ~[int]) -> ~[int] {
    // create a new vector to be returned.
    let mut v = numbers.clone();
    for n in v.mut_iter() {
        *n += 1;
    }
    // This would also work:
    /*  
    for i in range(0, v.len()) {
        v[i] = v[i] + 1;
    }
    */
    return v;
}

fn main() {
    let p = ~[1, 2, 3];
    let q = increment(p);
    for &x in q.iter() {
        print!("{:d} ", x);
    } 
    println!("");
}
