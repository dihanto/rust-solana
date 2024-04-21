fn divider(a: i32, b:i32) -> Option<i32>{
    if b == 0 {
        return None;
    }

    let result = a/b;
    return Some(result);
}
fn main() {
    let result = match divider(10, 5) {
        None => {
            println!("cannot divide by 0");
            0
        },
        Some(x) => x,
    };
    println!("Result: {}", result);

    let result1 = divider(10, 5);
    match result1 {
        None => println!("Cannot divide by 0"),
        Some(x) => println!("Result: {}", x),
    }
    let result2 = divider(10, 0);
    match result2 {
        None => println!("Cannot divide by 0"),
        Some(x) => println!("Result: {}", x),
    }

}

