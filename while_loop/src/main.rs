use std::{thread::sleep, time::Duration};

fn main() {
    let mut i = 1;
    let max = 5;
    // while i < max {
    //     println!("i = {}", i);
    //     i += 1;
    // }
    // nested while 
    while i < max {
        let mut j = 5;
        let mut k = 1;
        let max_innet = i;

        while j > max_innet {
            while k < max_innet {
                print!(" ");
                k += 1 ;
            }
            print!("*");
            j -= 1;
            sleep(Duration::from_secs(1));
        }

        println!("");
        i   += 1;
    }
}
