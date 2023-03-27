use dir_buster::{banner::Banner,config::Configuration};
use std::sync::Arc;
fn main() {
    
    //Create configuration
    let config = Arc::new(Configuration::new());
    //Print banner to std error
    let std_stderr = std::io::stderr();
    let banner = Banner::new(&config);
    banner.print_to(std_stderr, config).unwrap();
}
