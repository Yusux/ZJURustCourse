use std::env;
use std::io::Write;
// use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use simple_rust_redis::{client_fns::*, printresp};

#[volo::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(fmt::layer())
    //     .init();

    let args: Vec<String> = env::args().collect();

    // Check args
    match check_args(&args) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    loop {
        print!("redis> ");
        let _ = std::io::stdout().flush();  // flush stdout

        // Read input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // Deal with input
        let input: Vec<&str> = input.trim()
                                    .split(" ")
                                    .filter(|s| !s.is_empty())
                                    .collect();
        // Check input is not empty
        if input.is_empty() {
            continue;
        }
        // Match input to command
        match input[0].to_uppercase().as_str() {
            "GET" => {
                if input.len() != 2 {
                    println!("(error) ERR wrong number of arguments for command");
                    continue;
                }
                let key = input[1];
                let value = get(key).await;
                printresp!(value);
            },
            "SET" => {
                if input.len() != 3 && input.len() != 4 {
                    println!("(error) ERR wrong number of arguments for command");
                    continue;
                }
                let value = match input.len() == 3 {
                    true => {
                        let key = input[1];
                        let value = input[2];
                        let value = set(key, value).await;
                        value
                    },
                    false => {
                        let key = input[1];
                        let value = input[2];
                        let ex = input[3];
                        let value = set_ex(key, value, ex).await;
                        value
                    },
                };
                printresp!(value);
            },
            "DEL" => {
                if input.len() != 2 {
                    println!("(error) ERR wrong number of arguments for command");
                    continue;
                }
                let key = input[1];
                let value = del(key).await;
                printresp!(value);
            },
            "PING" => {
                if input.len() > 2 {
                    println!("(error) ERR wrong number of arguments for command");
                    continue;
                }
                let to_ping = if input.len() == 2 {
                    input[1]
                } else {
                    "PONG"
                };
                let value = ping(to_ping).await;
                printresp!(value);
            },
            "SUBSCRIBE" => {
                if input.len() != 2 {
                    println!("(error) ERR wrong number of arguments for command");
                    continue;
                }
                let channel = input[1];
                println!("1) \"subscribe\"\n2) \"{}\"\n3) (integer) 1", channel);
                let message = subscribe(channel).await;
                match message {
                    Some(message) => println!("1) \"message\"\n2) \"{}\"\n3) \"{}\"", channel, message),
                    None => panic!("Invalid message"),
                }
            },
            "PUBLISH" => {
                if input.len() != 3 {
                    println!("(error) ERR wrong number of arguments for command");
                    continue;
                }
                let channel = input[1];
                let message = input[2];
                let message = publish(channel, message).await;
                printresp!(message);
            },
            "QUIT" => {
                println!("OK");
                break;
            },
            _ => {
                println!("(error) ERR unknown command '{}'", input[0]);
            },
        }
    }
}
