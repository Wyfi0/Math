// Use function arr2 from crate ndarray
use ndarray::arr2;

fn main() {
    // Declare a mutable matrix made up of two arrays and format it pretty
    let a = arr2(&[[1, 2, 3],
                   [4, 5, 6]]);
    let b = arr2(&[[1, 2, 3],
                   [4, 5, 6]]);
    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);
    
    println!("Multiplied: {}", a.dot(&b));
    
}
