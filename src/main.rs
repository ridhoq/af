use hyper::http::Uri;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The URI to fetch
    uri: Uri
}

fn main() {
    let args = Cli::from_args();
    println!("{}", &args.uri);
}
