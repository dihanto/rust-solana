use std::{collections::VecDeque, vec};

fn main() {
    let mut data_one = vec!["batman", "superman", "lobo"];
    println!("data : {:?}", data_one);
    println!("length : {}, capacity : {}", data_one.len(), data_one.capacity());

    data_one.pop();
    println!("data : {:?}", data_one);
    println!("length : {}, capacity : {}", data_one.len(), data_one.capacity());
    
    data_one.remove(1);
    println!("data : {:?}", data_one);
    println!("length : {}, capacity : {}", data_one.len(), data_one.capacity());
    data_one.push("constantine");

    data_one.push("trigon");
    data_one.push("darkseid");
    println!("data : {:?}", data_one);
    println!("length : {}, capacity : {}", data_one.len(), data_one.capacity());

    data_one[2] = "red hood";
    println!("data : {:?}", data_one);
    println!("length : {}, capacity : {}", data_one.len(), data_one.capacity());

    let is_vector_empty = data_one.is_empty();
    println!("is vector empty : {}", is_vector_empty);

    data_one.clear();
    println!("data : {:?}", data_one);
    println!("length : {}, capacity : {}", data_one.len(), data_one.capacity());

    let mut result_one = vec![3,1,2];

    let mut data_two = vec![7,6,8];
    result_one.append(&mut data_two);

    println!("result : {:?}", result_one);
    println!("length : {}, capacity : {}", result_one.len(), result_one.capacity());

    result_one.append(&mut vec![4,5]);

    println!("result : {:?}", result_one);
    println!("length : {}, capacity : {}", result_one.len(), result_one.capacity());
    result_one.sort();
    println!("result : {:?}", result_one);

    // let mut vector_4 = vec![1,2,3,4,5];
    // let mut vector_5 : Vec<i64> = vec![1,2,3,4,5];
    // let mut vector_7: Vec<&str> = vec![] ;
    // let mut vector_8: Vec<&str> = Vec::new();

    let vec_eight = vec![1,2,3,4,5];
    for e in vec_eight{
        print!("{e} ");
    }

    let vec_nine = vec![5,6,7,8,9];
    for i in 0..vec_nine.len(){
        print!("{}", vec_nine[i]);
    }

    let vec_ten = vec![1,2,3,4,5];
    for e in &vec_ten {
        print!("{e} ");
    }
    for i in 0..vec_ten.len(){
        print!("{}", vec_ten[i]);
    }

    let vec_population = vec![2,1,3,4,5];
    let vec_sample = &vec_population[0..1];
    println!("sample : {:?}", vec_sample);

    let mut vec_10 = VecDeque::from(vec!["a", "b", "c"]);
    vec_10.pop_front();
    vec_10.push_front("z");
    println!("vec : {:?}", vec_10);

    vec_10.pop_back();
    vec_10.push_back("h");
    println!("vec : {:?}", vec_10);
}
