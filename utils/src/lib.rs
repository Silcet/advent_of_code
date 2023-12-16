pub fn get_input(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).header("Cookie", "session=53616c7465645f5fef123f45ead43b143a5574a2f33b1c141db66c57008303c91e463e7090048d5526c9203df3f6d11bf14f05b87dc1e611c8055370fee363e7").send()?;
    Ok(resp.text()?)
}

pub fn get_day_input(year: u32, day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    get_input(url)
}

#[cfg(test)]
mod tests {
    use crate::get_input;
    #[test]
    fn test_it_returns() {
        let result = get_input("https://adventofcode.com/2022/day/1/input".to_string());
        assert!(result.is_ok());
    }
}
