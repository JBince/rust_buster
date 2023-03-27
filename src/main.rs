use rust_buster::{banner::Banner, config::Configuration, scanner::Scanner};
use std::sync::Arc;

fn read_wordlist(config: Arc<Configuration>) -> Arc<Vec<String>> {
    //Read wordlist
    let wordlist = std::fs::read_to_string(&config.wordlist).unwrap();
    //Split wordlist into lines
    let wordlist: Vec<String> = wordlist.lines().map(|s| s.to_string()).collect();
    //Create Arc to share wordlist
    let wordlist = Arc::new(wordlist);
    wordlist
}

fn main() {
    //Create configuration
    let config = Arc::new(Configuration::new());
    //Print banner to std error
    let std_stderr = std::io::stderr();
    let banner = Banner::new(Arc::clone(&config));
    banner.print_to(std_stderr).unwrap();

    //Read wordlist
    let wordlist = read_wordlist(Arc::clone(&config));

    //Create scanner
    let scanner = Scanner::new(wordlist, Arc::clone(&config));

    //Scan url
    scanner.scan_url().unwrap();
}
