use crate::convert_timestamp_to_mst;
use ta::indicators::{ExponentialMovingAverage, SimpleMovingAverage};
use ta::Next;
use yahoo_finance_api::Quote;

// our custom reference to a yahoo quote
pub struct StockStepContext {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,
    pub close: f64,
    pub adjclose: f64,
    pub ma7: f64,   // 7 day moving average
    pub ma21: f64,  // 21 day moving average
    pub ema26: f64, // 26 day exponential moving average
    pub ema12: f64, // 12 day exponential moving average
    pub mcad: f64,
    pub sd20: f64,
    pub upper_band: f64,
    pub lower_band: f64,
    pub ema: f64,
    pub momentum: f64,
    pub log_momentum: f64,
}

pub struct StockContext {
    step_information: Vec<StockStepContext>,
}

impl StockContext {
    pub fn new(data: &Vec<Quote>) -> Self {
        Self {
            step_information: Self::generate_stock_step_context_from_data(data),
        }
    }

    fn generate_stock_step_context_from_data(data: &Vec<Quote>) -> Vec<StockStepContext> {
        let mut stock_data: Vec<StockStepContext> = vec![];
        for (pos, step) in data.iter().enumerate() {
            let ema26 = calculate_exp_moving_average(26, pos, data);
            let ema12 = calculate_exp_moving_average(12, pos, data);
            let point = StockStepContext {
                date: convert_timestamp_to_mst(&step.timestamp),
                open: step.open,
                high: step.high,
                low: step.low,
                volume: step.volume,
                close: step.close,
                adjclose: step.adjclose,
                ma7: calculate_moving_average(7, pos, data),
                ma21: calculate_moving_average(21, pos, data),
                ema26,
                ema12,
                mcad: (ema12 - ema26),
                sd20: 0.0,
                upper_band: 0.0,
                lower_band: 0.0,
                ema: 0.0,
                momentum: 0.0,
                log_momentum: 0.0,
            };
            stock_data.push(point);
        }
        stock_data
    }
}

fn calculate_moving_average(period: usize, current_position: usize, data: &Vec<Quote>) -> f64 {
    let limit = data.len();
    if current_position + period > limit - 1 {
        let plus_index = limit - current_position;
        if plus_index == 0 {
            return data[current_position].close;
        }
        let mut sma = SimpleMovingAverage::new(period).unwrap();
        let mut result = 0.0;
        for step in current_position..current_position + plus_index {
            result = sma.next(data[step].close);
        }
        result
    } else {
        let mut sma = SimpleMovingAverage::new(period).unwrap();
        let mut result = 0.0;
        for step in current_position..current_position + period {
            result = sma.next(data[step].close);
        }
        result
    }
}

fn calculate_exp_moving_average(period: usize, current_position: usize, data: &Vec<Quote>) -> f64 {
    let limit = data.len();
    let mut sma = ExponentialMovingAverage::new(period).unwrap();
    if current_position + period > limit - 1 {
        let plus_index = limit - current_position;
        if plus_index == 0 {
            return sma.next(data[current_position].close);
        }
        let mut result = 0.0;
        for step in current_position..plus_index + current_position {
            result = sma.next(data[step].close);
        }
        result
    } else {
        let mut result = 0.0;
        for step in current_position..current_position + period {
            result = sma.next(data[step].close);
        }
        result
    }
}
