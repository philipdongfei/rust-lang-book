use std::env;
use std::process;

use minigrep;
use minigrep::Config;


fn main() {
    
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //let config = parse_config(&args);

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }

}



/*
fn parse_config(args: &[String]) -> Config{
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
*/
