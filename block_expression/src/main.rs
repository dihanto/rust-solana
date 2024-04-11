fn main() {
    let x = 24;
    println!("The value of x is: {}", x);

    {
        println!("from block: The value of x is: {}", x);

        let y = 12;
        let z = x + y;
        println!("The value of y is: {}", y);
        println!("The value of z is: {}", z);
    }

    let a: i32 = {
        let n = 10;
        n * 2
    };

    println!("The value of a is: {}", a);

    {
        let b = 12;
        let mut total: i32 = 0;
        {
            let c = 13;
            {
                let d = 14;
                total = b + c + d;
            }
        }
        println!("The value of total is: {}", total);
    }
}
