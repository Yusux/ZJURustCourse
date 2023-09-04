use regex::RegexSet;
use std::fs;
use std::{path::Path, collections::BTreeSet};
use ansi_term::Colour;
use tracing::{info, span};

pub fn walk_tree(
    dir: &Path,
    regex: &RegexSet,
    matches: &mut BTreeSet<String>,
    is_verbose: &bool,
    is_color: &bool
) -> Result<(), Box<dyn std::error::Error>> {
    // set span for walk_tree
    let span = span!(tracing::Level::TRACE, "walk_tree");
    let _entry = span.enter();
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path().canonicalize().unwrap();

            // if the path is a directory, enter into it
            if path.is_dir() {
                // show detailed information when entering into a directory if is_verbose is true
                if *is_verbose {
                    if *is_color {
                        println!("{}", Colour::Cyan.bold().paint(format!("Enter into {}", path.to_string_lossy())));
                    } else {
                        println!("Enter into {}", path.to_string_lossy());
                    }
                }

                info!("Enter into {}", path.to_string_lossy());

                walk_tree(&path, regex, matches, is_verbose, is_color)?;

            // if the path is a file, get the file name and check if it matches the regex
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    let insert_status = matches.insert(path.to_string_lossy().to_string());

                    // if the file is matched, show "Matched" and the file name
                    // if the file is already matched, show "Already matched" and the file name
                    let match_status = if insert_status {
                        "Matched"
                    } else {
                        "Already matched"
                    };

                    // show detailed information when a file is matched if is_verbose is true
                    if *is_verbose {
                        if *is_color {
                            println!("{}: {}", match_status, Colour::Green.paint(path.to_string_lossy()));
                        } else {
                            println!("{}: {}", match_status, path.to_string_lossy());
                        }

                    }

                    info!("{}: {}", match_status, path.to_string_lossy());
                } else {
                    // show detailed information when a file is not matched if is_verbose is true
                    if *is_verbose {
                        if *is_color {
                            println!("Not matched: {}", Colour::Red.paint(path.to_string_lossy()));
                        } else {
                            println!("Not matched: {}", path.to_string_lossy());
                        }
                    }

                    info!("Not matched: {}", path.to_string_lossy());
                }
            }
        }
    }

    if *is_verbose {
        // show detailed information when exiting from a directory if is_verbose is true
        if *is_color {
            println!("{}", Colour::Blue.bold().paint(format!("Exit from {}", dir.to_string_lossy())));
        } else {
            println!("Exit from {}", dir.to_string_lossy());
        }
    }

    info!("Exit from {}", dir.to_string_lossy());

    Ok(())
}