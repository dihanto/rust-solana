fn main() {
    let mut x = 0;
    let max = 18;
    
     let result = loop {
        let mut j = 18;
        let mut k = 0;
        while k < x {
            print!(" ");
            k += 1;
        }
        loop {
            print!("*");
            j -= 1;
            if j <= x {
                break;
            }
        }
        println!();
        x += 1;
        if x == max {
            break x * 3;
        }
    };


    println!("{}", result);

    'loop2 : loop {
        x += 1;
        if x % 2 == 0 {
            continue;
        }
        if x > max {
            break 'loop2
        }
        println!("{}", x);
    }
}