fn main() {
    let number_a = 3;
    if number_a < 5 {
        println!("condition was true number_a is under 5");
    }

    let result_a = number_a >= 5;
    if result_a{
        println!("condition was true number_a is greater than 5");
    }

    let number_b = 3;
    if number_b ==2 {
        println!("number_b is 2");
    }  else if number_b < 2 {
        println!("number_b is less than 2");
    } else{
        println!("number_b is greater than 2");
    }

    // Nested IF
    let number_c = 8;
    if number_c > 6 {
        println!("Selamat, anda lulus");
        if number_c == 10{
        println!("dengan nilai sempurna");
    } else if number_c > 7 {
        println!("dengan nilai baik");
    } else{
        println!("dengan nilai cukup");
    } 
    } else {
        println!("and tidak lulus");
        if number_c < 4 {
            println!("belajar lagi ya");
        } else {
            println!("jangan malas belajar ya");
        }
    }
    
    let number_d = 3;
    let result_d ;
    if number_d == 2{
        result_d = true;
    } else{
        result_d = false;
    }
    println!("result_d is {result_d}" );

    let number_e = 3;
    let result_e: &str = if number_e == 2{
        "angka adalah 2"
    } else if number_e < 2{
        "angka kurang dari 2"
    } else{
        "angka lebih dari 2"
    };
    println!("angka adalah {result_e}" );

    let max  = 100.0;
    let string_f = "nilai minimum kelulusan";
    let result_f :f64 = if string_f == "nilai maksimum kelulusan"{
        max
    } else{
        max * 3.0 /4.0
    };
    println!("angka adalah {result_f}" );
}
