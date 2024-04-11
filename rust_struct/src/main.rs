use crate::models::color::Color;

mod models;
struct User{
    name: String,
    sign_in_count: u64,
    affliation: Vec<String>,
    active: bool,
}
fn main() {
    let user_one = User{
        name: String::from("Orgrim Doomhammer"),
        sign_in_count: 1,
        affliation: vec![String::from("Doomhammer"),
        String::from("Blackrock Chieftains")],
        active: true,
    };

    println!("{}", user_one.name);
    println!("{}", user_one.sign_in_count);
    println!("{:?}", user_one.affliation);
    println!("{}", user_one.active);

    let mut user_two = User{
        name: String::from("Orgrim Doomhammer"),
        sign_in_count: 1,
        affliation: vec![String::from("Doomhammer"),
        String::from("Blackrock Chieftains")],
        active: true,
    };
    user_two.name = String::from("Orgrim Doomhammer 2");
    user_two.affliation.pop();
    user_two.active = false;
    println!("{}", user_two.name);
    println!("{}", user_two.sign_in_count);
    println!("{:?}", user_two.affliation);
    println!("{}", user_two.active);

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }
    
    let point_one = Point {x: 1.1, y: 2.2};
    let Point { x: _, y: y_one} = point_one;
    println!("{}", y_one);

    let red = Color(255,0,0);
    println!("{:?} {:?} {:?}", red.0, red.1, red.2);

    let ps5 = models::game::GamingConsole{
        name: String::from("PS 5")
    };
    println!("{:#?}", ps5.name);
}
