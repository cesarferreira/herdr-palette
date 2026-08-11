fn main() {
    if let Err(error) = herdr_palette::run() {
        eprintln!("herdr-palette failed: {}", error.0);
        std::process::exit(1);
    }
}
