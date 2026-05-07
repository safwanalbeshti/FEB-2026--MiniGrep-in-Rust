use std::process;
use minigrep::{search,search_case_insensitive};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
            eprintln!("The error was {:?}", err);
            process::exit(1);
        }
    );
    if let Err(e) = run(&config) {
        eprintln!("The error was {e}");
        process::exit(1);
    };
}

fn run(config: &Config) -> Result<(),Box<dyn std::error::Error>>{
    println!("We're searching for the string {:?} in the file {:?}",config.query,config.filename);
    let file_contents = std::fs::read_to_string(& config.filename)?;
    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(& config.query, &file_contents)
    } else {
        search(& config.query, &file_contents)
    };
    for line in &results {
        println!("{line}");
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Self,&str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}