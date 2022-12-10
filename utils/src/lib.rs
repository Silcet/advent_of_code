pub fn get_input(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).header("Cookie", "session=53616c7465645f5f950bfa8c27fe1ccd7e75f8b963efcc41eb046c34b1099e715c755879e4407b682adb3781792676ed4c772686fd0bc30d56b594c1094628b8").send()?;
    Ok(resp.text()?)
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
