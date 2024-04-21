fn main() {
    let msg_1 = String::from("hello rust");
    let msg_2 = &msg_1;

    println!("{}", msg_1);
    println!("{}", msg_2);

    let mut msg_3 = String::from("hello");
    let msg_4 = &mut msg_3;

    *msg_4 = String::from("hello rust rust");
    println!("{}", msg_4);
    println!("{}", msg_3);
}
