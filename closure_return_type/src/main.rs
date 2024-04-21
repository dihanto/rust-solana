fn main() {
    let my_closure = do_something();
    println!("hello (from main)");
    my_closure();
    let my_closure_v2 = do_something_v2();
    let message = my_closure_v2(123, "hello rust".to_owned());
    println!("{}", message);

    run_x_times(4, |i: i32|{
        println!("{}", i);
    });

    let numbers = [24, 13, 2, 53, 3];
    let number_to_find = 53;
    let index = find_index(&numbers, |e: &i32| -> bool {
        if *e == number_to_find {
            true
        } else {
            false
        }
    });
    println!("number to find : {number_to_find} at index : {index}");
}


fn do_something() -> impl Fn(){
    println!("hello (from do something)");

    return || {
        println!("hello (from closure)");
    }
}

fn do_something_v2() -> impl Fn(i32, String) -> String {
    println!("hello (from do something v2)");
    return |a: i32, b: String| -> String {
        let message = format!("{} {}", a, b);
        message
    }
}

fn run_x_times<F>(x: i32, my_closure: F)
where
    F: Fn(i32),
    {
        for i in 0..x {
            my_closure(i);
        }
    }

    fn find_index<T, F>(data: &[T], cond_fn: F) -> i32
    where
        F: Fn(&T) -> bool,
        {
            for i in 0..data.len() {
                if cond_fn(&data[i]) {
                    return i as i32
                }
            }
            return -1
        }