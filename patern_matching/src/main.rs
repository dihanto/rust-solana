fn main() {
    let time = "morning";

    let time_in_indonesia = match time {
        "morning" => "pagi",
        "afternoon" => "siang",
        "evening" => "sore",
        _ => "ga tau kapan",
    };

    println!("{} in Indonesia is {}", time, time_in_indonesia);

    let value: Option<i32> = Option::Some(5);

    match value {
        Some(1) => println!("one"),
        Some(2) => println!("two"),
        Some(x) => println!("{x} greater than two"),
        None => println!("none"),
    }

    let value = 4;
    match value {
        1 | 2 => println!("one or two"),
        3..=5 => println!("three to five"),
        6 => println!("six"),
        _ => println!("other number"),
    }
    // jika ingin menyimpan value dari match gunakan @ (binding)
    // let value = 3;
    // match value {
    //     n @ (1 | 2) => println!("one or two ({})", n),
    //     n @ 3..=5   => println!("three through five ({})", n),
    //     6           => println!("six"),
    //     _           => println!("other number"),
    // }


    // ekuavalen dengan
    // let value: Option<i32> = Some(5);
    // match value {
    //     Some(1 | 2) => println!("one or two"),
    //     Some(3..=5) => println!("three through five"),
    //     Some(6)     => println!("six"),
    //     Some(x)     => println!("{x} greater than six"),
    //     _           => println!("none"),
    // }

    let value1 = Some(4);
    let message = match value1{
        Some(x) if x % 2 == 0 => println!("even number"),
        Some(x)  => println!("odd number"),
        None => println!("none"),
    };
    println!("{message:?}");
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point{
        x: 2,
        y: 7,
    };
    match p {
        Point{x, y: 0} => println!("On the x axis at {}", x),
        Point{x:0, y} => println!("On the y axis at {}", y),
        Point{x, y} => println!("On neither axis ({}, {})", x, y),
    }

    enum Color {
        Black,
        White,
        Rgb(i32, i32, i32)
    }

    let color = Color::Rgb(0, 160, 255);
    if let Color::Rgb(r, g, b) = color{
        println!("{} {} {}", r, g, b);
    }
    match color {
        Color::Rgb(r, g, b) => println!("{} {} {}", r, g, b),
        _ => println!("other color"),
    }

    let grades = ("A", "AA", "AAA");
    let (grade_a, grade_aa, grade_aaa) = grades;
    println!("grade_a: {grade_a}");
    println!("grade_aa: {grade_aa}");
    println!("grade_aaa: {grade_aaa}");

    match grades {
        (grade_a, grade_aa, grade_aaa) => {
            println!("grade_a: {grade_a}");
            println!("grade_aa: {grade_aa}");
            println!("grade_aaa: {grade_aaa}");
        }
    }

    let numbers = (2,4,32);
    let (_, second, _)= numbers;
    println!("second number: {second}");

    let numbers2 = (2,4,5,6,7,8,9,10);
    let (first, .., last) = numbers2;
    println!("first number: {first}");
    println!("last number: {last}");    
}
