fn main() {
    let (num1, num2) = (24, 8);

    let value_adition = num1 + num2;
    println!("{} + {} = {}", num1, num2, value_adition);

    let value_sub = num1 - num2;
    println!("{} - {} = {}", num1, num2, value_sub);

    let value_mut = num1 * num2;
    println!("{} * {} = {}", num1, num2, value_mut);

    let value_div = num1 / num2;
    println!("{} / {} = {}", num1, num2, value_div);

    let value_modulus = num1 % num2;
    println!("{} % {} = {}", num1, num2, value_modulus);
}
