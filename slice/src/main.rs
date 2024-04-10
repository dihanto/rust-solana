fn main() {
    let numbers = [12, 16, 8, 3, 9, 11];
    println!("numbers : {:?}, length : {}", numbers, numbers.len());
    println!("numbers[0] : {:?}", numbers[0]);

    let slice_a = &numbers[0..3];
    println!("slice_a : {:?}, length : {}", slice_a, slice_a.len());
    
    let slice_b = &slice_a[1..=2];
    println!("slice_b : {:?}, length : {}", slice_b, slice_b.len());

    let mut numbers2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("====before====");
    println!("numbers2 : {:?}, length : {}", numbers2, numbers2.len());

    let slice_c = &mut numbers2[..=3];
    slice_c[2] = 100;
    println!("====after====");
    println!("slice_c : {:?}, length : {}", slice_c, slice_c.len());
    println!("numbers2 : {:?}, length : {}", numbers2, numbers2.len());
    
    let scores1 = [7,8,9,10];
    for score in &scores1[..]{
        print!("score : {:?}, ", score);
    }
    println!("");
    let mut scores2 = [7,8,9,10];
    println!("(before) scores2 : {:?}", scores2);
    let slice_d = &mut scores2[..];
    for score in &mut slice_d[..]{
        *score += 1;
    }
    println!("(after) scores2 : {:?}", scores2);
}