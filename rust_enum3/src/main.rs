mod constants;
fn main() {
    let company = constants::Company::Apple;
    
    match company {
        constants::Company::Apple => {
            println!("Apple")
        },
        _ => {
        println!("other than apple")
        }
    }
}
