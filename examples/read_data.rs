use chrono::prelude::DateTime;
use chrono::{TimeZone, Utc};
use chrono_tz::MST;
use std::time::{Duration, UNIX_EPOCH};
use stock_lib::convert_timestamp_to_mst;
use stock_lib::stock_data::StockContext;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::Quote;

#[tokio::main]
async fn main() {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2022, 9, 30).and_hms_milli(23, 59, 59, 999);
    let response = provider.get_quote_history("AAPL", start, end).await;
    // extract just the latest valid quote summery
    // including timestamp,open,close,high,low,volume
    match response {
        Ok(response) => {
            let quotes: Vec<Quote> = response.quotes().unwrap();
            println!("Apple's quotes in January: {:?}", &quotes);
            let data = StockContext::new(&quotes);
            for dat in &data.step_information {
                println!("{:?}", dat);
            }
            data.head(5);
            data.tail(5);
        }
        Err(e) => {
            println!("Hit error: {:?}", e);
        }
    };
}
