mod cache;
mod read;

use clap::{App, Arg};

#[tokio::main]
async fn main() {
    let matches = App::new("simple")
        .arg(Arg::with_name("mode").long("mode").takes_value(true).required(true))
        .arg(Arg::with_name("times").long("times").takes_value(true))
        .get_matches();

    let mode = matches.value_of("mode").unwrap();
    match mode {
        "cache" => {
            let times = matches.value_of("times").unwrap_or("10").parse::<u64>().unwrap();
            cache::run_cache(times).await;
        }
        "read" => {
            read::run_read();
        }
        _ => {
            println!("Invalid mode. Use 'cache' or 'read'.");
        }
    }
}
