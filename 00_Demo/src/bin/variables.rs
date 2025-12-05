fn main() {
    // Immutable by default
    let x = 5;
    println!("x = {}", x);

    // Mutable binding
    let mut y = 10;
    println!("y before = {}", y);
    y = 15;
    println!("y after = {}", y);

    // Shadowing
    let z = 1;
    let z = z + 2; // shadow previous z
    println!("z = {}", z);
}
