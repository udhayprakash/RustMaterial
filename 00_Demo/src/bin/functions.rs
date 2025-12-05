fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let s = add(2, 3);
    println!("2 + 3 = {}", s);

    // function with Result for fallible work
    match parse_positive("42") {
        Ok(v) => println!("parsed positive = {}", v),
        Err(e) => eprintln!("error: {}", e),
    }
}

fn parse_positive(s: &str) -> Result<u32, &'static str> {
    let n: i32 = s.parse().map_err(|_| "not an integer")?;
    if n >= 0 {
        Ok(n as u32)
    } else {
        Err("negative number")
    }
}
