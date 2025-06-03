use crate::constants::IS_DAY_OFF_URL;
use std::io::{Error, ErrorKind, Result};

pub async fn is_day_off(day: &String) -> Result<bool> {
    let response = match reqwest::get(format!("{}/{}", IS_DAY_OFF_URL, day,)).await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!(
                "Couldn't get response from {}: {}",
                IS_DAY_OFF_URL,
                e.to_string(),
            );
            return Err(Error::new(ErrorKind::Other, e.to_string()));
        }
    };

    let raw_text = match response.text().await {
        Ok(raw) => raw,
        Err(e) => {
            eprintln!(
                "Couldn't parse response from {}: {}",
                IS_DAY_OFF_URL,
                e.to_string()
            );
            return Err(Error::new(ErrorKind::Other, e.to_string()));
        }
    };

    Ok(raw_text == "1")
}
