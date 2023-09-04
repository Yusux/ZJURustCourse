pub fn check_args(
    args: &[String],
    regex_vec: &mut Vec<String>,
    path_vec: &mut Vec<String>,
    is_verbose: &mut bool,
    is_color: &mut bool,
    is_log: &mut bool
) -> Result<(), String> {
    let mut args = args.to_owned(); // args will be modified, so clone it

    // check the second argument
    while args.len() > 1 {
        match &args[1] as &str {
            // if the second argument is "-n" or "--name", check the third argument to get regex
            "-n" | "--name" => {
                if args.len() < 3 {
                    return Err("Missing argument for -n/--name.".to_string());
                }
                regex_vec.push(args[2].to_string());
                args.drain(1..=2);
            },

            // if the second argument is "-p" or "--path", check the third argument to get path
            "-p" | "--path" => {
                if args.len() < 3 {
                    return Err("Missing argument for -p/--path.".to_string());
                }
                path_vec.push(args[2].to_string());
                args.drain(1..=2);
            },

            // if the second argument is "-v" or "--verbose", show more details
            "-v" | "--verbose" => {
                *is_verbose = true;
                args.remove(1);
            },

            // if the second argument is "-c" or "--color", color the output
            "-c" | "--color" => {
                *is_color = true;
                args.remove(1);
            },

            // if the second argument is "-l" or "--log", log the output to the file named by the current time in the current directory
            "-l" | "--log" => {
                *is_log = true;
                args.remove(1);
            },

            // any other arguments are invalid
            _ => {
                return Err(format!("The way to use: {} [-p/--path path] [-n/--name regex] [-v/--verbose] [-c/--color] [-l/--log]", args[0]));
            }
        }
    }

    if regex_vec.is_empty() {
        return Err(format!("At least one regex is needed.\nThe way to use: {} [-p/--path path] [-n/--name regex] [-v/--verbose] [-c/--color] [-l/--log]", args[0]));
    }

    if path_vec.is_empty() {
        return Err(format!("At least one path is needed.\nThe way to use: {} [-p/--path path] [-n/--name regex] [-v/--verbose] [-c/--color] [-l/--log]", args[0]));
    }

    Ok(())
}