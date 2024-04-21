#[derive(Debug)]
struct Book<'abc, 'def>{
    title: &'abc str,
    description: &'abc str,
    price: &'def i32,
}
impl<'abc, 'def> Book<'abc, 'def>{
    fn get_info(&self) -> String {
        let info = format!("{} {} {}", self.title, self.description, self.price);
        info
    }
    fn get_price(&self) -> &i32 {
        self.price
    }
}
fn get_book_info(book: &Book) -> String {
    let info = book.get_info();
    return info;
}
fn get_book_price<'abc>(book: &'abc Book) -> &'abc i32{
    book.get_price()
}

fn find_greater_number <'a, T>(x: &'a T, y: &'a T) -> &'a T where T: std::cmp::PartialOrd{
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let number = get_number();
    println!("{}", number);
    let book = Book{
        title: "hello",
        description: "hello",
        price: number,
    };
    println!("{:?}", book);

    {
        let x = 13;
        let y = 20;
        let result = find_greater_number::<i32>(&x, &y);
        println!("result in integer {}", result);
    }
    {
        let x = 3.14;
        let y = 2.71;
        let result = find_greater_number::<f64>(&x, &y);
        println!("result in float {}", result);
    }
}
fn get_number<'my_lifetime>() -> &'my_lifetime i32{
    &13
}
fn do_something_v1(x: &str) -> &str {
    x
}
fn do_something_v2<'a>(x: &'a str) -> &'a str {
    x
}
fn do_something_v4<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str {
    "hello"
}
fn do_something_v5<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'b str {
    y
}
fn do_something_v6<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len()  > y.len() {
        x
    } else {
        y
    }
}