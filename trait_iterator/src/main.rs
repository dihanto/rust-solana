fn main() {
    let data_arr = [1, 3, 5, 7, 9];
    let doubles: Vec<i32> = data_arr.iter().map(|e| e * e).collect();
    println!("doubles: {:?}", doubles);

    let data_vec = vec!["1", "2", "3", "4", "a"];
    let numbers: Vec<i32> = data_vec
        .iter()
        .map(|e| -> i32 {
            match e.parse::<i32>() {
                Ok(n) => n,
                Err(_) => 0,
            }
        })
        .filter(|e| *e > 0 && *e % 2 == 0)
        .rev()
        .collect::<Vec<i32>>();

    println!("{numbers:?}");

    let mut data_vec = vec![1, 2, 3, 4];

    data_vec.iter_mut().for_each(|d| {
        *d = *d * 2;
    });
    println!("{:?}", data_vec);

    for d in &mut data_vec {
        *d = *d * 2;
    }
    println!("{:?}", data_vec);
}
