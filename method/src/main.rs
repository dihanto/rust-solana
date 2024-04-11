mod models;

fn main() {
    let mut car = models::Car::new(String::from("Mercedes-Benz"), String::from("Vision Gran Turismo"));
    println!("Car: {:?}", car);

    let info = car.info();
    println!("Info: {}", info);
    car.congratulate(String::from("Sylvanas Windrunner"));
    car.set_manufacture_year(2022);
    let detailed_info = car.info();
    println!("Info: {}", detailed_info);
}
