fn main() {
    let str1 = String::from("Lisa Blackpink");
    let slice1 = &str1[..];
    let slice2 = &str1[4..7];
    println!("{:?}", str1);
    println!("{:?}", slice1);
    println!("{:?}", slice2);

    let bytes = vec![69, 108, 117, 118, 101, 105, 116, 105, 101, 32, 243, 159, 164, 152];
    let str2 = String::from_utf8(bytes).unwrap();
    println!("str2: {}", str2);

    let str2 = str1.as_str();
    println!("str2: {}", str2);
    let mut str3 = String::from("Son Goku");
    let str4 = str3.as_mut_str();
    println!("str4: {}", str4);
    println!("str3: {}", str3);

    let str5 = "John Towner Williams";
    let str6 = str5.to_string();
    println!("str6: {}", str6);
}
