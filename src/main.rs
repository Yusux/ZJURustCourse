mod find;
mod check_args;

use regex::RegexSet;
use std::env;
use std::process;
use ansi_term::Colour;
use tracing::{info, error, span};
use tracing_appender;
use tracing_subscriber;
use chrono;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut regex_vec = Vec::new();
    let mut path_vec = Vec::new();
    let mut is_verbose = false;
    let mut is_color = false;
    let mut is_log = false;

    // check arguments
    match check_args::check_args(&args, &mut regex_vec, &mut path_vec, &mut is_verbose, &mut is_color, &mut is_log) {
        Ok(()) => {},
        Err(string) => {
            eprintln!("ERROR: {}", string);
            process::exit(1);
        }
    }

    // change the path to store the log file
    // if no "-l/--log" argument is given, store the log file in "/dev/null" to discard the log
    let file_appender = if is_log {
        tracing_appender::rolling::never("./", format!("myfind-{}.log", chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")))
    } else {
        // not to log
        tracing_appender::rolling::never("/dev", "null")
    };
    
    // prepare the tracing subscriber
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .init();
    
    // set span for main
    let span = span!(tracing::Level::TRACE, "main");
    let _entry = span.enter();

    info!("args: {:?}", args);

    // use RegexSet to store the regexes, which is more convenient and simplifies the code
    let regex: RegexSet = RegexSet::new(&regex_vec).unwrap();

    info!("start finding");

    // find matching files
    match find::find(&path_vec, &regex, &is_verbose, &is_color) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("No matching file found.");
            } else {
                println!("found matching files:");
                if is_color {
                    for file in matches {
                        println!("{}", Colour::Green.bold().paint(file));
                    }
                } else{
                    for file in matches {
                        println!("{}", file);
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("Error occurred: {}", error);
            error!("Error occurred: {}", error);
            process::exit(1);
        }
    }

    info!("finish finding");
}