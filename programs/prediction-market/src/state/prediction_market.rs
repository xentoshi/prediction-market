use anchor_lang::prelude::*;

#[account]
pub struct PredictionMarket {
    pub question: String,
    pub end_time: i64,
    pub yes_amount: u64,
    pub no_amount: u64,
    pub resolved: bool,
    pub outcome: Option<bool>,
}