fn main() {
    let mut number = 24;
    println!("value: {:?}", number);

    let pointer_number: &mut i32 = &mut number;
    println!("pointer: {:p}", pointer_number);

    *pointer_number = 12;

    let underlying_value = *pointer_number;
    println!("underlying value: {}", underlying_value);
    println!("value: {:?}", number);
}
