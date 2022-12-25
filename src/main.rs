use clap::Parser;
use tesseract::Tesseract;

/// Shazam for videos
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to screenshot to extract from
    #[arg(short, long)]
    path: String,
}

fn main() -> () {
    let args = Args::parse();
    image_to_text(&args.path);
}

fn image_to_text(path: &str) {
    let tesseract = Tesseract::new(None, Some("eng")).unwrap();

    let text = tesseract
        .set_image(path).expect("REASON")
        .recognize().unwrap()
        .get_text().unwrap();
    println!("{}", text);
}