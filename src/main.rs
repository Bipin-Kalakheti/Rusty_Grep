use std::env;
use std::process;

use Rusty_Grep::run;
use Rusty_Grep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });



    println!("Searching for {}", config.query);
    println!("In file {}",config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}



