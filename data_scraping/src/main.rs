fn main() {
    if let Err(e) = data_scraping::get_args().and_then(data_scraping::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
