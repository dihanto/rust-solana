fn main() {
    let result = do_something_with_number_v1(13, |d: i32| d * 2);
    println!("result: {result}");
    let result = do_something_with_number_v1(13, double);
    println!("result: {result}");
    let result = do_something_with_number_v1(13, pow_number);
    println!("result: {result}");

    let mut x = 5;
    {
        let mut square_x = || x *= x;
        square_x();
    }
    println!("x: {x}");
    do_something_with_number_v2(4, |y: i32| x += y );
    println!("x: {x}");

    let mut number = 1;
    do_something_with_number_v3(14, |x| number += x);
    println!("number: {number}");
}

fn do_something_with_number_v1<F>(n: i32, f: F) -> i32
where
    F: Fn(i32) -> i32, 
{
    return f(n);
}
fn do_something_with_number_v2<F>(n: i32, mut f: F)
where
    F: FnMut(i32),
    {
        f(n);
    }
fn do_something_with_number_v3<F>(n: i32, f: F)
where
        F: FnOnce(i32),
        {
            f(n);
        }

fn double (d: i32) -> i32{
    d * 2
}
fn pow_number(d: i32) -> i32{
    d.pow(2)
}