// Exercise 2.2
fn incrementMut(numbers : &mut [int]) {
    for n in numbers.mut_iter() {
        *n += 1;
    }
    // This would also work:
    /*
    for i in range(0, numbers.len()) {
        numbers[i] = numbers[i] + 1;
    }
    */
}

fn main() {
   let mut p = ~[1, 2, 3];
   incrementMut(p);
   for &x in p.iter() {
      print!("{:d} ", x);
   }
}
