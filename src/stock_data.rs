use crate::convert_timestamp_to_mst;
use ta::indicators::{ExponentialMovingAverage, SimpleMovingAverage, StandardDeviation};
use ta::Next;
use yahoo_finance_api::Quote;

// our custom reference to a yahoo quote
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone)]
pub struct StockContext {
    pub step_information: Vec<StockStepContext>,
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
            let sd20 = calculate_rolling_std(20, pos, data);
            let momentum: f64 = step.close - 1.0;
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
                sd20,
                upper_band: step.close + sd20 * 2.0,
                lower_band: step.close - sd20 * 2.0,
                ema: exponential_moving_average_mean(data, pos).unwrap(),
                momentum,
                log_momentum: momentum.log10(),
            };
            stock_data.push(point);
        }
        stock_data
    }

    pub fn head(&self, count: usize) {
        for x in 0..count {
            println!("{:?}", self.step_information[x])
        }
    }

    pub fn tail(&self, count: usize) {
        for x in 0..count {
            println!(
                "{:?}",
                self.step_information[self.step_information.len() - 1 - x]
            )
        }
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
        for step in current_position..current_position + period + 1 {
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
        for step in current_position..current_position + period + 1 {
            result = sma.next(data[step].close);
        }
        result
    }
}

fn calculate_rolling_std(period: usize, current_position: usize, data: &Vec<Quote>) -> f64 {
    let limit = data.len();
    let mut sma = StandardDeviation::new(period).unwrap();
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
        for step in current_position..current_position + period + 1 {
            result = sma.next(data[step].close);
        }
        result
    }
}

fn exponential_moving_average_mean(data_set: &Vec<Quote>, window_size: usize) -> Option<f64> {
    let data_set: Vec<f64> = data_set.iter().map(|f| f.close).collect();
    if window_size > data_set.len() {
        return None;
    }

    let mut result: Vec<f64> = Vec::new();

    let weighted_multiplier = 2.0 / (window_size as f64 + 1.0);
    let first_slice = &data_set[0..window_size];
    let first_sma: f64 = first_slice.iter().sum::<f64>() / window_size as f64;
    result.push(first_sma);
    for i in window_size..data_set.len() {
        let previous_ema = result[result.len() - 1];
        let ema: f64 =
            (data_set[i] * weighted_multiplier) + previous_ema * (1.0 - weighted_multiplier);
        result.push(ema);
    }

    let mut sum: f64 = 0.0;
    for x in &result {
        sum = sum + x;
    }
    Some(sum / result.len() as f64)
}
