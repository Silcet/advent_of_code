use std::fs;
use std::path::PathBuf;

const SESSION_KEY: &str = "53616c7465645f5f56a795a2d21d25f0e721f189c29881064fd18bca0b0ca1cbdadf9eea89e0529b5256f301fc5e9ed3915e295f7582b0f301697436a0b4f6a0";

pub struct AdventOfCode {
    url: String,
    cache: PathBuf,
    input: Option<String>,
}

impl AdventOfCode {
    pub fn new(year: u32, day: u8) -> Self {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let filename = format!("day{day}_{year}.txt");
        let mut path = PathBuf::from(".cache");
        path.push(&filename);

        Self {
            url,
            cache: path,
            input: None,
        }
    }

    pub fn get_input(&mut self) -> Result<&String, Box<dyn std::error::Error>> {
        if self.input.is_none() {
            if self.cache.exists() {
                println!("Using cache");
                self.input = Some(self.fetch_from_cache()?);
            } else {
                println!("Fetching input");
                self.input = Some(self.fetch_from_url()?);
                self.cache_input()?;
            }
        }

        Ok(&self.input.as_ref().unwrap())
    }

    fn fetch_from_url(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(&self.url)
            .header("Cookie", format!("session={SESSION_KEY};"))
            .send()?;
        Ok(resp.text()?)
    }

    fn fetch_from_cache(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from_utf8_lossy(&fs::read(self.cache.as_path())?).to_string())
    }

    fn cache_input(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(fs::write(
            self.cache.as_path(),
            self.input.as_ref().unwrap(),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_full_flow() {
        let mut aoc = AdventOfCode::new(2023, 1);
        let input = aoc.get_input();
        assert!(input.is_ok());
    }

    #[test]
    fn test_cache() {
        let content = "random text to test things";
        let cache_path = PathBuf::from(".cache/day1_2023.txt");
        println!("{:?}", std::env::current_dir());
        fs::write(cache_path.as_path(), content).unwrap();

        let mut aoc = AdventOfCode::new(2023, 1);
        let input = aoc.get_input();
        assert!(input.is_ok(), "{:?}", input.err());
        assert_eq!(input.unwrap(), content);
    }

    // #[test]
    // fn test_it_returns() {
    //     let result = get_input("https://adventofcode.com/2022/day/1/input".to_string());
    //     assert!(result.is_ok());
    // }
}
