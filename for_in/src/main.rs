fn main() {
    'perulangan: for i in 0..5 {
        if i == 2 {
            println!("perulangan diberhentikan pada perulangan ke {i}");
            break 'perulangan;
        }
        println!("i = {}", i);
    }
    for i in 0..=5 {
        println!("i = {}", i);
    }
    let array = ["jason", "budi", "joko"];
    for name in array {
        println!("{name}");
    }
}
