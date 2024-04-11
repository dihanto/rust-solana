fn main() {
    const SuperheroSuperman: &str = "Superman";
    const SuperheroOmniman: &str = "Omniman";
    const SuperheroHomelander: &str = "Homelander";
    const SuperheroHyperion: &str = "Hyperion";

    enum Superhero {
        Superman,
        Omniman,
        Homelander,
        Hyperion,
    };

    // definisi variable menggunakan konstanta
    let value1 = SuperheroSuperman;
    let value2 = SuperheroOmniman;

    // definisi variable menggunakan enum
    let value3 = Superhero::Superman;
    let value4 = Superhero::Omniman;
}
