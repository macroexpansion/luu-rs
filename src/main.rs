use clap::Parser;
use std::io::{Error, ErrorKind};

mod content_copy;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    src: Option<String>,
    dst: Option<String>,
}

fn main() {
    let args = Args::parse();

    let handler = match args.dst {
        Some(dst) => content_copy::file_copy(&args.src.unwrap(), &dst),
        None => {
            if let Some(src) = args.src {
                content_copy::clipboard_copy(&src)
            } else {
                let err = Error::new(ErrorKind::NotFound, "no source file provided");
                Err(err)
            }
        }
    };

    if let Err(error) = handler {
        eprintln!("{error}");
    }
}
