use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::{search,search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    // static methods are :: i guess like c++
    let config: GrepConfig = GrepConfig::build(&args).unwrap_or_else(|e| {
        // print to stderr
        eprintln!("problem parsing args: {e}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// dyn Error means something that implements Error
// kinda interesting to just take stuff out of main() and put it in
// the also very-generic-sounding run()
fn run(config: GrepConfig) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).expect("should have been able to read the file");
        
    let results = if config.ignore_case {
        // MAKE SURE THERE'S NO SEMICOLON
        // that's how the return value works
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct GrepConfig {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl GrepConfig {
    // &'static str means a reference to a string that will live for the entire program
    fn build(args: &[String]) -> Result<GrepConfig, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments, needs [word] [filename]");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        // value or false
        // seems kinda hacky but swagever
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(GrepConfig {
            query,
            file_path,
            ignore_case
        })
    }
}
