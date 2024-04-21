fn main() {
    let str1 = String::from("luwe");
    println!("{str1}");
    do_something(str1);

    let str2 = String::from("ngelak");
    do_something(str2);
    println!("{str2}");
    
}

fn do_something(str: String) {
    println!("{str}");
}
