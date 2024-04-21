fn main() {
    let str1 = String::new();
    println!("{}", str1);

    let str2 = String::from_utf8(vec![78, 55, 51]).unwrap();
    println!("{str2}");

    let mut str3 = String::new();
    println!("{}", str3);
    str3 = String::from("hello");
    println!("{}", str3);

    let str4 = String::from("my phone is Pixel 6");
    let str5 = str4.replace("Pixel 6", "iPhone 13");
    println!("{}", str4);
    println!("{}", str5);

    let mut str6 = String::from("Pixel 6");
    println!("{}", str6);
    str6.insert_str(0, "my phone");
    println!("{}", str6);
    str6.insert_str(8, " is ");
    println!("{}", str6);

    let mut str7 = String::from("3310");
    str7.insert(0, 'N');
    str7.insert(1, 'o');
    str7.insert(2, 'k'); 
    str7.insert(3, 'i'); 
    str7.insert(4, 'a'); 
    str7.insert(5, ' '); 

    let mut str8 = String::from("Pixel 6");
    str8.push_str(" is good phone");
    println!("{}", str8);
    str8.clear();
    println!("{}", str8);

    let is_exists = str7.contains("3310");
    println!("{}", is_exists);

    let str9 = String::from("iPhone");
    let str10 = String::from("12");
    let str11 = String::from("Pro");
    let str: String = [str9, str10, str11].join(" ");
    println!("{}", str);
    
}
