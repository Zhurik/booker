use crate::config::Config;
use crate::timer::UnspotDateData;
use reqwest;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};

#[derive(Serialize, Deserialize)]
struct TimeRanges {
    #[serde(rename = "startDate")]
    start_date: i64,

    #[serde(rename = "endDate")]
    end_date: i64,
}

#[derive(Serialize, Deserialize)]
struct NewDeskRequest {
    comment: Option<String>,

    desk: String,

    force: bool,

    #[serde(rename = "guestEmail")]
    guest_email: Option<String>,

    #[serde(rename = "guestName")]
    guest_name: Option<String>,

    #[serde(rename = "isBookingByQr")]
    is_booking_by_qr: bool,

    organizer: Option<String>,

    #[serde(rename = "placeFrom")]
    place_from: String,

    title: String,

    #[serde(rename = "timeRanges")]
    time_ranges: Vec<TimeRanges>,
}

impl NewDeskRequest {
    fn new(desk_id: &String, start: i64, end: i64) -> NewDeskRequest {
        let mut time_ranges: Vec<TimeRanges> = Vec::new();
        time_ranges.push(TimeRanges {
            start_date: start,
            end_date: end,
        });

        NewDeskRequest {
            comment: None,
            desk: desk_id.to_string(),
            force: false,
            guest_email: None,
            guest_name: None,
            is_booking_by_qr: false,
            organizer: None,
            place_from: String::from("MAP_DESKTOP"),
            title: String::from("4.121 забронирован в 4 этаж"),
            time_ranges: time_ranges,
        }
    }
}

pub async fn book_seat(unspot_data: &UnspotDateData, config: &Config) -> Result<()> {
    let client = reqwest::Client::new();

    let request_body = NewDeskRequest::new(&config.spot_id, unspot_data.start, unspot_data.end);

    let url = format!("{}api/bookings/desk/new", config.unspot_url);

    let response = match client
        .post(url)
        .json(&request_body)
        .bearer_auth(&config.unspot_token)
        //.body(body)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!(
                "Couldn't get response from {}: {}",
                config.unspot_url,
                e.to_string(),
            );
            return Err(Error::new(ErrorKind::Other, e.to_string()));
        }
    };

    let status = response.status();

    println!("{status}");

    Ok(())
}
