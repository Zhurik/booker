use clap::Parser;

use super::constants;

#[derive(Parser)]
pub struct Config {
    #[arg(long, env)]
    pub unspot_token: String,

    #[arg(long, env)]
    pub spot_id: String,

    #[arg(long, env, default_value = constants::UNSPOT_URL)]
    pub unspot_url: String,
}
