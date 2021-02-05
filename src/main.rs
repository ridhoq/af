use hyper::{body::HttpBody as _, Client, http::Uri};
use hyper_tls::HttpsConnector;
use structopt::StructOpt;
use tokio::io::{self, AsyncWriteExt as _};

#[derive(StructOpt)]
struct Cli {
    // The URI to fetch
    uri: Uri
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::from_args();

    // Set up the HTTPS connector with the client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut res = client.get(args.uri).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    Ok(())
}
