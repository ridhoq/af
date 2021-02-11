use std::fmt;

use clap::{AppSettings, Clap};
use reqwest::{Method, Url};

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

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!(), setting = AppSettings::AllowMissingPositional)]
/// A (http) fetch CLI 😀👍
pub struct Opts {
    /// HTTP method. If no HTTP method is provided, GET is used by default
    #[clap(name = "METHOD", index = 1, default_value = "GET", parse(try_from_str = parse_method))]
    pub method: Method,

    /// URL to fetch
    #[clap(name = "URL", index = 2, required = true, parse(try_from_str))]
    pub url: Url,
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
