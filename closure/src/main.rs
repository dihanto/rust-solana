fn main() {
    fn pow_v1(x: i32) -> i32 {
    x.pow(2)
    }

    let pow_v2 = |x: i32| -> i32 {
        x.pow(2)
    };

    let pow_v3 = |x: i32| {
        x.pow(2)
    };

    let pow_v4 = |x: i32| x.pow(2);
    pow_v1(3);
    pow_v2(3);
    pow_v3(3);
    pow_v4(3);

    let mut num = 5;
    // let increase_by = |num : &mut i32, x: i32| *num +=x;
    // num += 5;
    let mut increase_by = move |x:i32| {
        num += x;
        println!("{num} from closure");
    };
    increase_by(10);
    println!("num = {}", num);
}
