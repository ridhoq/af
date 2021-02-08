use std::error;
use std::fmt;

use http::{Method, Uri};
use hyper::{body::HttpBody as _, Client, Request};
use hyper_tls::HttpsConnector;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use tokio::io::{self, AsyncWriteExt as _};

#[derive(Debug, Clone)]
struct InvalidHttpMethodError;

impl fmt::Display for InvalidHttpMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid or unknown HTTP Method")
    }
}

/// Parse HTTP method from string. Throws error if invalid/unknown HTTP method
fn parse_method(src: &str) -> Result<Method, InvalidHttpMethodError> {
    match src.to_uppercase().as_str() {
        "GET" => Ok(Method::GET),
        "PUT" => Ok(Method::PUT),
        "POST" => Ok(Method::POST),
        "HEAD" => Ok(Method::HEAD),
        "PATCH" => Ok(Method::PATCH),
        "TRACE" => Ok(Method::TRACE),
        "DELETE" => Ok(Method::DELETE),
        "OPTIONS" => Ok(Method::OPTIONS),
        "CONNECT" => Ok(Method::CONNECT),
        _ => Err(InvalidHttpMethodError),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::AllowMissingPositional)]
/// A (http) fetch CLI 😀👍
struct Cli {
    /// HTTP method. If no HTTP method is provided, GET is used by default
    #[structopt(name = "METHOD", index = 1, default_value = "GET", parse(try_from_str = parse_method))]
    method: Method,

    /// URI to fetch
    #[structopt(name = "URI", index = 2, required = true, parse(try_from_str))]
    uri: Uri,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Cli::from_args();

    // Set up the HTTPS connector with the client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(args.method)
        .uri(args.uri)
        // TODO: couldn't figure out how to not pass a body with the Request::builder
        // TODO: pass in a real body when doing POST/PUT/etc
        .body(hyper::Body::from(""))
        .unwrap();

    let mut res = client.request(req).await?;

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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_valid_http_method() {
        let get_lower = "get";
        let get_upper = "GET";

        assert_eq!(parse_method(get_lower).unwrap(), Method::GET);
        assert_eq!(parse_method(get_upper).unwrap(), Method::GET);
    }

    #[test]
    fn test_invalid_http_method() {
        let poop = "poop";
        let empty = "";

        assert!(parse_method(poop).is_err());
        assert!(parse_method(empty).is_err());
    }
}
