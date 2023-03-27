use crate::config::Configuration;
use crate::scanner::result::*;
use anyhow::Result;
use rayon::{prelude::*, ThreadPoolBuildError};
use std::sync::Arc;

pub struct Scanner {
    pub(super) target_url: String,
    wordlist: Arc<Vec<String>>,
    status_codes: Vec<u16>,
    method: String,
    threads: usize,
    request_number: u16,
}

impl Scanner {
    pub fn new(wordlist: Arc<Vec<String>>, config: Arc<Configuration>) -> Self {
        Self {
            target_url: if config.url.ends_with("/") {
                config.url.to_string()
            } else {
                format!("{}/", config.url)
            },
            wordlist,
            status_codes: config.status_codes.clone(),
            method: config.method.clone(),
            threads: config.threads,
            request_number: 1,
        }
    }
    pub fn scan_url(&self) -> Result<()> {
        //Create the global thread pool
        //self.create_global_pool()?;

        let mut request_number = 1 as u16;

        //Execute that code within the thread pool to control the number of threads used
        self.wordlist.iter().for_each(|word| {
            match self.probe(word.to_string(), request_number.clone()) {
                Ok(()) => (),
                Err(e) => println!("{}", e),
            }
            request_number += 1;
        });
        Ok(())
    }

    #[tokio::main]
    async fn probe(&self, word: String, request_number: u16) -> Result<()> {
        let response = reqwest::get(format!("{}{}", &self.target_url, word)).await?;
        let status_code = response.status().as_u16();
        //total_requests += 1;
        if self.status_codes.contains(&status_code) {
            let method = self.method.clone();
            let content_length = response.content_length().unwrap() as u16;
            let endpoint = word.to_string();
            let result = ResultEntry::new(
                status_code,
                method,
                content_length,
                endpoint,
                request_number,
            );
            println!("{}", result);
        }
        Ok(())
    }

    pub fn create_global_pool(&self) -> Result<(), ThreadPoolBuildError> {
        //Function to create thread pool based on the provided number of threads
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build_global()
    }

    pub async fn test_connection(&self) -> Result<()> {
        let response = reqwest::get(&self.target_url).await?;
        let status_code = response.status().as_u16();
        if status_code == 200 {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Connection to target failed"))
        }
    }
}
