use anchor_lang::prelude::*;

#[error_code]
pub enum PredictionMarketError {
    #[msg("Market already resolved")]
    MarketAlreadyResolved,
    #[msg("Market is closed")]
    MarketClosed,
    #[msg("Market still open")]
    MarketStillOpen,
}   