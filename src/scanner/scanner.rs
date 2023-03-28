use crate::config::Configuration;
use crate::scanner::result::*;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
#[allow(unused)]
use rayon::{prelude::*, ThreadPool, ThreadPoolBuildError};
use std::sync::Arc;
use tokio::runtime::Builder;

pub struct Scanner {
    pub(super) target_url: String,
    wordlist: Arc<Vec<String>>,
    status_codes: Vec<u16>,
    method: String,
    threads: usize,
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
        }
    }

    pub fn scan_url(&self) -> Result<()> {
        //Uncomment to create a thread pool for use with rayon
        //self.create_global_pool()?;

        let mut request_number = 1 as u16;

        let bar = ProgressBar::new(self.wordlist.len() as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap(),
        );

        //Creates multithreaded runtime context for testing endpoints
        let rt = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(self.threads)
            .build()
            .unwrap();

        //Needs to be changed to .spawn() instead of block_on() to facilitate multithreading

        rt.block_on(async {
            for word in self.wordlist.iter() {
                match self.probe(word.to_string(), request_number.clone()).await {
                    Ok(result) => {
                        ProgressBar::println(&bar, result);
                    }
                    Err(_) => (),
                };
                request_number += 1;
                bar.inc(1);
            }
        });
        bar.finish();
        Ok(())
    }

    //#[tokio::main]
    async fn probe(&self, word: String, request_number: u16) -> Result<String> {
        let response = reqwest::get(format!("{}{}", &self.target_url, word)).await?;
        let status_code = response.status().as_u16();
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
            Ok(format!("{}", result))
        } else {
            Err(anyhow::anyhow!("Unacceptable status code"))
        }
    }

    //Generates global thread pool for use with Rayon multithreading. Generally less efficient than using
    //Asynchronous requests, but might be useful for some cases. Must explore further.

    pub fn create_global_pool(&self) -> Result<ThreadPool, ThreadPoolBuildError> {
        //Function to create thread pool based on the provided number of threads
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build()
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
