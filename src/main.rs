use clap::Parser;

mod content_copy;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    src: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(src) = args.src {
        content_copy::clipboard_copy(&src).unwrap();
    } else {
        eprintln!("no source file provided");
    }
}
