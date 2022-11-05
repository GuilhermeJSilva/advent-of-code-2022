use reqwest::{cookie::Jar, Client, Url};
use std::{sync, path, str::FromStr};
use super::filecache;


pub struct AocCLient {
    base_url: Url,
    client: Client,
    file_cache: filecache::FileCache,
}

impl AocCLient {
    pub fn new(session_token: &str) -> Self {
        let jar = Jar::default();
        let base_url = Url::parse("https://adventofcode.com/")
            .expect("The BASE_URL should never fail to deserialize");
        jar.add_cookie_str(&format!("session={}", session_token), &base_url);
        let cache_base_dir = path::PathBuf::from_str("tmp/aoc").expect("Base dir should be a valid path").into_boxed_path();
        return AocCLient {
            base_url,
            client: Client::builder()
                .cookie_provider(sync::Arc::new(jar))
                .build()
                .unwrap(),

            file_cache: filecache::FileCache::new(cache_base_dir),
        };
    }

    async fn get_input_remote(&self, year: u16, day: u8) -> String {
        let resp = self
            .client
            .get(format!("{}{}/day/{}/input", self.base_url, year, day))
            .send()
            .await.unwrap()
            .text()
            .await.unwrap();
        return resp;
    }

    fn get_input_filename(year: u16, day: u8) -> String {
        return format!("input-{}-{}.txt", year, day);
    }

    pub async fn get_input(&self, year: u16, day: u8) -> String {
        let filename = AocCLient::get_input_filename(year, day);
        match self.file_cache.retrieve(&filename)  {
            Some(contents) => contents,
            None => {
                let input = self.get_input_remote(year, day).await;
                self.file_cache.store(&filename, &input);
                return input;
            }
        }
    }
}

