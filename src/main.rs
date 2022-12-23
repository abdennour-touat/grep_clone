use std::env;
use std::error::Error;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1)
    });
    println!("searching {:?}", conf.query);
    println!("in file {:?}", conf.file_name);
    if let Err(err) = run(conf) {
        println!("application error: {}", err);
        process::exit(1);
    }
    // run(conf);
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_name)?;

    println!("with test: {}", content);
    Ok(())
}
struct Config {
    file_name: String,
    query: String,
}
impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic!("not enough arguements!");
            return Err("not enough arguments!");
        }
        let file_name = args[2].clone();
        let query = args[1].clone();
        Ok(Config {
            file_name: file_name,
            query: query,
        })
    }
}
