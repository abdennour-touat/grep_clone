use mini_grep::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });
    // println!("searching {:?}", conf.query);
    // println!("in file {:?}", conf.file_name);
    if let Err(err) = mini_grep::run(conf) {
        println!("application error: {}", err);
        process::exit(1);
    }
}
