use clap::Parser;
use std::io::Result;

mod config;
mod constants;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::parse();
    println!("{}", config.unspot_token);
    println!("{}", config.unspot_url);

    let response = match reqwest::get(constants::IS_DAY_OFF_URL).await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!(
                "Couldn't get response from {}: {}",
                constants::IS_DAY_OFF_URL,
                e.to_string(),
            );
            std::process::exit(1);
        }
    };

    let raw_text = match response.text().await {
        Ok(raw) => raw,
        Err(e) => {
            eprintln!(
                "Couldn't parse response from {}: {}",
                constants::IS_DAY_OFF_URL,
                e.to_string()
            );
            std::process::exit(1);
        }
    };

    println!("{}", raw_text);

    Ok(())
}
