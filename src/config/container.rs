use super::utils::status_codes;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    //Targets to scan
    #[arg(short, long, required = true)]
    pub url: String,

    //Status codes to look for
    #[arg(short, long, num_args = 1.., value_delimiter = ',', default_value = "200,204,301,302,307,308,401,403,405")]
    pub status_codes: Vec<u16>,

    //Number of threads to use
    #[arg(short, long, default_value = "4")]
    pub threads: usize,

    //Path to the wordlist
    #[arg(short, long, required = true)]
    pub wordlist: String,

    //HTTP method to use
    #[arg(short, long, default_value = "GET")]
    pub method: String,
}

impl Configuration {
    pub fn default() -> Self {
        Configuration {
            url: String::new(),
            status_codes: status_codes(),
            threads: 4,
            wordlist: String::new(),
            method: String::new(),
        }
    }
    pub fn new() -> Self {
        Configuration::parse()
    }
}
