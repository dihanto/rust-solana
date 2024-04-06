fn main() {
    
    let integer: i64 = 1;
    println!("{}", integer);

    let float: f32 = 2.0834;
    println!("{}", float);

    let array: [i32; 4] = [4, 3, 2, 1];
    println!("{}", array[0]);
    println!("{}", array[1]);

    let string: &str = "oYYyyy";
    println!("{}", string);

    let tuple: (bool, i16, f32) = (false, 1, 1.1);
    println!("{}", tuple.0);

    struct MyTuple(bool, u32, f32);
    let tuple2 = MyTuple(true, 2, 2.2);
    println!("{}", tuple2.1);

    struct MyStruct {
        should_do_groceries: bool,
        birth_year: u32,
        height_in_meters: f64,
    }

    let my_struct = MyStruct {
        should_do_groceries: true,
        birth_year: 2000,
        height_in_meters: 1.68,
    };
    println!("{}", my_struct.birth_year);
    println!("{}", my_struct.should_do_groceries);
    println!("{}", my_struct.height_in_meters);

    enum CardinalDirection {
        North,
        East,
        South,
        West,
    }

    let d = CardinalDirection::East;
    if let CardinalDirection::East = d {
        print!("We are going east");
    } else {
        println!("We are not going east but in some other direction!");
    }

    enum Shape {
        Square { side: f64 },
        Rectangle { width: f64, height: f64 },
        Circle { radius: f64 },
    }

    let s = Shape::Rectangle {
        width: 800.0,
        height: 60.0,
    };

    match s {
        Shape::Square { side } => {
            println!("A {}x{} square!", side, side);
        }
        Shape::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        }
        Shape::Circle { radius } => {
            println!(
                "A circle of radius {} and diameter {}!",
                radius,
                radius * 2.0
            );
        }
    }
}
