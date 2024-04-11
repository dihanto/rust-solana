mod lego;
mod model;
fn main() {
    lego::LegoSet::what_is_lego();

    let rough_terrain_crane = lego::LegoSet{
        code: 42081,
        name: String::from("Rough Terrain Crane"),
        category: String::from("Construction"),
        age_minimum: 3,
    };

    println!("{:?}", rough_terrain_crane);

    let xtreme_offroader = lego::LegoSet::new(42099, String::from("4x4 Offroader"), String::from("Transport"), 2);
    println!("{:?}", xtreme_offroader);

    let red = model::Color::red();
    let green = model::Color::green();
    let blue = model::Color::blue();
    println!("{:?}", red);
    println!("{:?}", green);
    println!("{:?}", blue);
    let random_color = model::Color::new(12,25,47);
    println!("{:?}", random_color);
}
