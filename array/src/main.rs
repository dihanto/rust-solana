fn main() {
    let mut numbers = [24, 12 ,32, 7, 9];
    println!("array {:?}", numbers);   
    // println!("array {numbers:?}"); 

    let data0 = numbers[0];
    println!("data0 {}", data0);

    let data1 = numbers[1];
    println!("data1 {}", data1);

    numbers[1]= 100;
    numbers[3]= 200;
    println!("array {:?}", numbers);

    let mut alphabet = ["a", "b", "c", "d", "e", "f"];
    let boolean: [bool; 2] = [true, false];
    let data_numberik1: [i32; 10] = [0; 10];
    println!("{data_numberik1:?}");
    let data_numerik2= [4;5];
    println!("{data_numerik2:?}");
    let length = alphabet.len();
    println!("length {}", length);
    let names = ["John", "Jane", "Joe", "damian", "daniel"];
    for i in 0..names.len(){
        println!("array index ke-{} {}", i, names[i]);
    }
    let mut i = 0;
    while i < names.len(){
        println!("array index ke-{} {}", i, names[i]);
        i += 1;
    }
    let mut j = 0;
    loop{
        if j >= names.len(){
            break;
        }

        println!("array index ke-{} {}", j, names[j]);
        j += 1;
    }

    for (i, name) in names.iter().enumerate(){
        println!("array names index ke-{i}: {name}");
    }

    let data_arr = [
        ["salad", "noodles", "rice"],
        ["apple", "banana", "melon"],
        ["milk", "tea", "coffee"],
    ];
    for sub_arr in data_arr{
        for item in sub_arr{
            print!("{item}, ");
        }
        println!();
    }
    
}
