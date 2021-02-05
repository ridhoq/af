use hyper::{body::HttpBody as _, Client};
use http::{Method, Uri, method::InvalidMethod};
use hyper_tls::HttpsConnector;
use structopt::StructOpt;
use tokio::io::{self, AsyncWriteExt as _};

fn parse_method(src: &str) -> Result<Method, InvalidMethod> {
    Method::from_bytes(src.as_bytes())
}

#[derive(StructOpt)]
/// A (http) fetch CLI 😀👍
struct Cli {
    /// HTTP method
    #[structopt(parse(try_from_str = parse_method))]
    method: Option<Method>,
    /// The URI to fetch
    #[structopt()]
    uri: Uri,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
