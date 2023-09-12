use std::{env, process::exit};
use ansi_term::Colour::{Purple, Cyan};

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: {} <input uri>", args[0]);
        exit(1);
    }
    let url = &args[1];

    let resp = reqwest::get(url).await.unwrap();

    eprintln!("{} {}", Purple.paint(format!("{:?}", resp.version())), Cyan.paint(resp.status().to_string()));

    for (key, value) in resp.headers() {
        println!("{}: {}", Cyan.paint(key.to_string()), value.to_str().unwrap());
    }

    println!("");

    let body = resp.text().await.unwrap();
    println!("{}", body);
}
