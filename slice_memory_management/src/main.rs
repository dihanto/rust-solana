fn main() {
    let mut number = [24, 12, 8 ,7 ,6 ,5 ,4 ,3 ,2 ,1];
    println!("{:?}", number);
    let n1  = &mut number;
    println!("{:?}", n1);
    n1[0] = 3000;
    println!("{:?}", number);
}
