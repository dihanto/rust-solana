fn greet(){
    println!("Hello, world!");
}
fn main() {
    println!("Hello, rust");
    greet();
    greet_custom_message("Damian", "welcome to the castle");

    let width = 10;
    let heigth = 20;
    let length = 30;
    let volume = calculate_box_volume(width, heigth, length);
    println!("The volume of the box is {volume}");  
    let volume2 = calculate_box_volume2(width, heigth, length);
    println!("The volume of the box is {volume2}");
    let volume3 = calculate_box_volume3(width, heigth, length);
    let message3 = format!("The volume of the box is {volume3}");
    greet_custom_message("Damian", message3.as_str());

    println!("{}", get_score_message(100.0));
    println!("{}", get_score_message(85.0));
    println!("{}", get_score_message(50.0));
}

fn greet_custom_message(name:&str, message:&str){
    println!("hi {name}, {message}");
}

fn calculate_box_volume(width: i32, heigth: i32, length: i32) -> i32 {
    let volume = width * heigth * length;
    return volume;
}

fn calculate_box_volume2(width: i32, heigth: i32, length: i32) -> i32 {
    let volume = width * heigth * length;
    volume
}

fn calculate_box_volume3(width: i32, heigth: i32, length: i32) -> i32 {
    width * heigth * length
}

fn get_score_message (score: f32) -> &'static str {
    if score == 100.0 {
        return "you got a perpect score"
    }

    if score > 76.0 {
        return "congrats, you passed the exam!"
    }

    "your score is below the passing grade"
}