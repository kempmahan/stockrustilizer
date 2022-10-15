use yahoo_finance_api::Quote;

// our custom reference to a yahoo quote
pub struct StockContext<'a> {
    pub timestamp: &'a u64,
    pub open: &'a f64,
    pub high: &'a f64,
    pub low: &'a f64,
    pub volume: &'a u64,
    pub close: &'a f64,
    pub adjclose: &'a f64,
}

impl <'a> From<&'a Quote> for StockContext<'a> {
    fn from(quote: &'a Quote) -> StockContext<'a>  {
        StockContext {
            timestamp: &quote.timestamp,
            open: &quote.open,
            high: &quote.high,
            low: &quote.low,
            volume: &quote.volume,
            close: &quote.close,
            adjclose: &quote.adjclose,
        }
    }
}