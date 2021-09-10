pub fn fetch_resp_text(url: &str) -> String {
    let resp = reqwest::blocking::get(url).unwrap();
    resp.text().unwrap()
}

#[cfg(test)]
mod tests {
    use super::fetch_resp_text;
    use std::ffi::OsString;
    use std::env;

    #[test]
    fn test_fetch_resp_text() {
        let url = match env::var_os("REMOTE_URL"){
            None => OsString::from("http//127.0.0.1").to_owned(),
            Some(url) => url
        };
        let text = fetch_resp_text(url.to_str().unwrap().to_owned().as_str());
        assert_ne!(text.len(), 0);
    }
}
