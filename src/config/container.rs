use clap::Parser;
use super::utils::status_codes;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {

    //Targets to scan
    #[arg(short, long, required = true)]
    pub url: String,

    //Status codes to look for
    #[arg(short, long, default_value = "200")] //The logic for default values needs to be fixed
    pub status_codes: Vec<u16>,

    //Number of threads to use
    #[arg(short, long, default_value = "4")]
    pub threads: usize,

    //Path to the wordlist
    #[arg(short, long, required = true)]
    pub wordlist: String,
}

impl Configuration {
    pub fn default() -> Self {
        Configuration {
            url: String::new(),
            status_codes: status_codes(),
            threads: 10,
            wordlist: String::new(),
        }
    }
    pub fn new() -> Self {
        Configuration::parse()
    }
}