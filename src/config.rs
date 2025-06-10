use clap::Parser;

use super::constants;

/// Simple bot for booking places in unspot
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// bearer token from unspot webpage
    #[arg(long, env)]
    pub unspot_token: String,

    /// UUID for spot from unspot
    #[arg(long, env)]
    pub spot_id: String,

    /// Url can be used for self hosted installations
    #[arg(long, env, default_value = constants::UNSPOT_URL)]
    pub unspot_url: String,
}
