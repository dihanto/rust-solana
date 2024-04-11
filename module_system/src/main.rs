// use std::io::*;

use rand::Rng;

use crate::my_number::{ conversion_utility::string_to_number, is_odd_number};

mod my_io;
mod my_number;

fn generate_random_number() -> i32 {
    rand::thread_rng().gen_range(0..100)
}

fn main() {
    println!("enter a number:");
    let message = my_io::read_entry();
    println!("your number: {}", message);

    let number = string_to_number(message);
    let result = is_odd_number(number);
    println!("is is_odd_number: {}", result);

    for i in 0..5 {
        println!("random number {}: {}",i, generate_random_number());
    }
}
