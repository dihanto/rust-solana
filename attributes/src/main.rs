#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Superhero {
    Superman,
    OmniMan,
    Hyperion,
}
impl std::fmt::Display for Superhero {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
fn main() {
    let value: Superhero = Superhero::Superman;
    if value == Superhero::Superman {
        println!("Superman");
    }

    println!("{value} (via Display trait)");
    #[cfg(target_os = "windows")]
    {
        println!("{value:#?} (via Debug trait) and in windows");
    }

    match value {
        Superhero::Superman => println!("Superman"),
        Superhero::OmniMan => println!("OmniMan"),
        _ => println!("Not Superman or OmniMan"),
    }
}
