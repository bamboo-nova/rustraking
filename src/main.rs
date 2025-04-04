fn main() {
    // PyErrなので、ここでBox<dyn Error>に戻す
    if let Err(e) = track::get_args().and_then(|args| track::run(args).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
