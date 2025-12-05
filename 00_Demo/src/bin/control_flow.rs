fn main() {
    let n = 7;

    if n % 2 == 0 {
        println!("{} is even", n);
    } else {
        println!("{} is odd", n);
    }

    // loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }
    println!("looped {} times", count);

    // for
    for i in 0..3 {
        println!("for i = {}", i);
    }
}
