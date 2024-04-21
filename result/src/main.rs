
#[derive(Debug)]
enum MathError{
    DivisionByZero,
    InfinityNumber,
    OtherError,
}
fn main() {
    let result: f64 = match divider(10.0, 5.0) {
        Err(m) => {
            println!("ERROR! {:?}", m);
            0.0
        },
        Ok(r) => r,
    };

println!("result: {:?}", result);

    let result2 : Result<f64, MathError> = divider(10.0, 0.0);
    println!("{:?}", result2);
}

fn divider(a:f64, b:f64) -> Result<f64, MathError>{
    if b == 0.0 {
        return Err(MathError::DivisionByZero)
    }

    let result = a/b;
    return Ok(result);
}
