use std::error;

use clap::Clap;
use reqwest::redirect::Policy;
use reqwest::Client;
use tokio::io::{self, AsyncWriteExt as _};

mod cli;

use cli::Opts;

fn get_user_agent() -> String {
    format!("{} {}", clap::crate_name!(), clap::crate_version!())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Opts = Opts::parse();

    // build out client
    let client = Client::builder()
        .user_agent(get_user_agent())
        // by default, don't follow redirects
        // TODO: make this a CLI arg
        .redirect(Policy::none())
        .build()?;

    let req = client.request(args.method, &args.url.to_string());
    let mut res = req.send().await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.chunk().await? {
        let chunk = next;
        io::stdout().write_all(&chunk).await?;
    }

    Ok(())
}
