use arboard::Clipboard;
use clap::Parser;

const DEFAULT_LENGTH: usize = 32;
const CHARSERT: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const SPECIAL_CHARSET: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

#[derive(Parser)]
struct Cli {
    #[arg(default_value_t = DEFAULT_LENGTH)]
    length: usize,

    #[arg(short, long)]
    exclude_special: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut charset = CHARSERT.to_string();

    if !cli.exclude_special {
        charset.push_str(SPECIAL_CHARSET);
    }

    let mut result = String::with_capacity(cli.length);
    for _ in 0..cli.length {
        result.push(
            charset
                .chars()
                .nth(fastrand::usize(0..charset.len()))
                .unwrap(),
        );
    }

    if let Ok(mut clipboard) = Clipboard::new() {
        let _ = clipboard.set_text(&result);
    }
    println!("{}", result);
}
