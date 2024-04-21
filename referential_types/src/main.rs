fn main() {
    //imutable
    // struct Config {
    //     port: u16,
    // }
    // let config: Config = Config { port: 8080 };
    // let config_reference: &Config = &config;
    // println!("Using port {}.", config_reference.port);

    // let val = 10;
    // let r1 = &val;
    // let r2 = &r1;
    // println!("{r1} should be the same as {r2}.");

    // struct Config {
    //     port: u16,
    // }
    // let mut config: Config = Config { port: 8080 };
    // let config_reference: &mut Config = &mut config;
    // config_reference.port = 4000;
    // println!("Using port {}.", config.port);

    // let mut val = 10;
    // let r1 = &mut val;
    // // let r2 = &mut val;
    // *r1 = 5;
    // // *r2 = 6;
    // println!("{r1}");

    let val = 10;
    let r1 = &val;
    let val2 = *r1;
    println!("{val2}");
}
