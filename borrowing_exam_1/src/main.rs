fn main() {
    let msg_1 = String::from("hello rust");

    let msg_2 = &msg_1;
    let msg_3 = &msg_1;
    let msg_4 = &msg_1;

    println!(" msg_2: {}, msg_3: {}, msg_4: {}", msg_2, msg_3, msg_4);

    let mut msg_5 = String::from("hello rust");
    let msg_6 = &mut msg_5;
    // let msg_7 = &mut msg_5; satu tidak tidak boleh memiliki lebih dari satu mutable reference
    println!("msg_6: {}", msg_6);

    let mut msg_7 = String::from("hello rust");
    let msg_8 = &msg_7;
    // let msg_9 = &mut msg_7;
    println!("msg_8: {}", msg_8);
}

