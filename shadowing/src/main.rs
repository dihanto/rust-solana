fn main() {
    let name = "Orgrim Doomhammer";
    let mut race = "Orc";
    let mut number = 27;

    println!("Hello, {}! You are a {}. You are {} years old.", name, race, number);

    {
        let name = "Sylvanas Windrunner";
        race = "Undead";
        let number = 24;
        println!("Hello, {}! You are a {}. You are {} years old.", name, race, number);
    }

    println!("Hello, {}! You are a {}. You are {} years old.", name, race, number);
}
