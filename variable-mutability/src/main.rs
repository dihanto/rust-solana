fn main() {
    let x;
    x = 3;
    // type ascription
    let x2: i16 = 2;
    // mutability
    let x3 = 3; //immutable
    let mut x4 = 4; //mutable
    x4 = 5;
    // scope
    let x5 = 2;
    println!("{}", x5); // 2
                        // Create a new scope
    {
        let y = 3;
        // We can use x here
        println!("{}", x5); // 2
        println!("{}", y); // 3
    }
    println!("{}", x5); // 2
                        // println!("{}", y); // won't compile because y is "not in scope"
    let x6 = 2;
    let x6 = 3;
    let (a, b) = (2, 3);
    struct Person {
        name: &'static str,
        age: u32,
        likes_brownies: bool,
    }
}
