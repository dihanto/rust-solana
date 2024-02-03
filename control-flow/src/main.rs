fn main() {
    let should_print = true;
    if should_print {
        println!("Printing!");
    }
    let value = 10;
    if value == 0 {
        println!("Zero");
    } else if value > -10 && value < 10 {
        println!("Single digit!");
    } else {
        println!("Multiple digits!");
    }
    let mut i = 1;
    loop {
        println!("I am looping");
        i += 1;
        if i == 2 {
            break;
        }
    }
    while i < 5 {
        println!("Looping using while");
        i += 1;
    }
    for i in (1..=10).rev() {
        println!("{i}...");
    }
}
