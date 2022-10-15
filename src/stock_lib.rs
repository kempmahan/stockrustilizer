pub mod stock_data;

use chrono::{DateTime, Utc};
use chrono_tz::MST;
use std::time::{Duration, UNIX_EPOCH};

pub fn convert_timestamp_to_mst(stamp: &u64) -> String {
    let datetime = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(*stamp));
    let mst = datetime.with_timezone(&MST);
    mst.format("%Y-%m-%d").to_string()
}
