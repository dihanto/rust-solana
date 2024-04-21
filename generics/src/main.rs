struct Point<T, U> {
    x: T,
    y: T,
    z: U
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T{
        &self.x
    }
    fn get_y(&self) -> &T{
        &self.y
    }
    fn get_z(&self) -> &U{
        &self.z
    }
}
fn main() {
    let num_one : Point<i32, f64> = Point{x: 502, y:120, z: 4.5};
    println!("{} {} {}", num_one.get_x(), num_one.get_y(), num_one.get_z());

    let num_two = Point{x:1.2, y:4.3, z: 534};
    println!("{} {} {}", num_two.get_x(), num_two.get_y(), num_two.get_z());   
}
