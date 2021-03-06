use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
let config = parse_config(&args);
    let contents = fs::read_to_string(config.fileName).expect("Something went wrong reading the file");
    println!("With text:\n{}",contents);
}
struct Config {
    query:String,
          fileName:String,
}
fn parse_config(args:&[String])-> Config {
    let query = args[1].clone();
    let fileName = args[2].clone();
Config     {query,fileName}
}
