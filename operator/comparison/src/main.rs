fn main() {
    let (number_a, number_b) = (12, 24);

    let res_one = number_a == number_b;
    println!("res_one = {}", res_one);

    let res_two = number_a != number_b;
    println!("res_two = {}", res_two);

    // Named argument macro println
    let res_three = number_a > number_b;
    println!("res_three = {res_three}");

    let res_four = number_a < number_b;
    println!("res_four = {}", res_four);

    let res_five = number_a >= number_b;
    println!("res_five = {}", res_five);

    let res_six = number_a <= number_b;
    println!("res_six = {}", res_six);
}
