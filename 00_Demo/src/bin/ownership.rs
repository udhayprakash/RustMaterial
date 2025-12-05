fn take_ownership(s: String) {
    println!("I took: {}", s);
}

fn borrow_str(s: &String) {
    println!("I borrowed: {}", s);
}

fn main() {
    let s = String::from("hello");
    borrow_str(&s); // borrow, s is still valid
    take_ownership(s); // move, s no longer valid after this

    // Uncommenting the next line causes a compile error because `s` was moved
    // println!("s after move: {}", s);
}
