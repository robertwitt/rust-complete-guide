use quicli::prelude::*;
use structopt::StructOpt;

const CONN_ADDR: &str = "127.0.0.1:3002";

/// This is a small CLI tool to shorten URLs usisng the hyperurl url shortening service
#[derive(Debug, StructOpt)]
struct Cli {
    /// The url to shorten
    #[structopt(long = "url", short = "u")]
    url: String,
    // Setting logging for this CLI tool
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    println!("Shortening: {}", args.url);
    let client = reqwest::Client::new();
    let mut res = client
        .post(&format!("http://{}/shorten", CONN_ADDR))
        .body(args.url)
        .send()?;
    let a: String = res.text().unwrap();
    println!("http://{}", a);
    Ok(())
}
