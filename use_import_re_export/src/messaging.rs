pub use self::sub_module::say_hello_message;

mod sub_module{
    pub fn say_hello_message(){
        println!("Hello message");
    }
}