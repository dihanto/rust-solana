mod calculation_spec;
mod two_dimensional;

use crate::calculation_spec::Area;

fn main() {
    let circle_one = two_dimensional::Circle{radius: 10};
    println!("circle area: {}", circle_one.calculate());

    let square_one = two_dimensional::Square{length: 5};
    println!("square area: {}", square_one.calculate());
    println!("Hello, world!");
}
