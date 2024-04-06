fn main() {
    let (value_left , value_right) = (12, -12);
    let res_one = -value_left == value_right;
    let res_two = !(value_left == value_right);
    println!("{res_one} {res_two}");

    let (bool_left, bool_right) = (false, true);
    // Whitespace character tab \t
    println!("AND result \t: {}", bool_left && bool_right);
    println!("OR result \t: {}", bool_left || bool_right);
}
