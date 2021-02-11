use anyhow::Result;
use reqwest::redirect::Policy;
use reqwest::{Client, Method};
use tokio::io::{self, AsyncWriteExt as _};

use crate::cli::Opts;

fn get_user_agent() -> String {
    format!("{} {}", clap::crate_name!(), clap::crate_version!())
}

pub async fn fetch(args: Opts) -> Result<()> {
    // build out client
    let client = Client::builder()
        .user_agent(get_user_agent())
        // by default, don't follow redirects
        .redirect(Policy::none())
        .build()?;

    // TODO: improve body handling
    let mut body = "";
    if args.method == Method::POST || args.method == Method::PUT {
        body = " ";
    }

    let req = client
        .request(args.method, &args.url.to_string())
        .body(body);

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
