use clap::Parser;
use std::io::Result;

mod config;
mod constants;
mod is_day_off;
mod seater;
mod timer;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::parse();

    let unspot_data = timer::UnspotDateData::from_today();

    match is_day_off::is_day_off(&unspot_data.date).await {
        Ok(x) => {
            if x {
                println!("Завтра выходной");
                std::process::exit(0);
            }
        }
        Err(_) => {
            std::process::exit(1);
        }
    };

    match seater::book_seat(&unspot_data, &config).await {
        Ok(_) => println!("Yes"),
        Err(_) => println!("Shit"),
    }

    Ok(())
}
