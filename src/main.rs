use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("searching {:?}", query);
    println!("in file {:?}", file_name);

    let content = fs::read_to_string(file_name).expect("file name wrong or doesn't exist");

    println!("with test: {}", content);
}
