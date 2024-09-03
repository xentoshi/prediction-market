use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("3v8pjuLoRLX112cZw5XiyrsGYckDoLFRtaPyHvbqgZFU");

#[program]
pub mod prediction_market {
    use super::*;

    pub fn initialize_market(ctx: Context<InitializeMarket>, question: String, end_time: i64) -> Result<()> {
        instructions::initialize_market::handler(ctx, question, end_time)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64, bet_yes: bool) -> Result<()> {
        instructions::place_bet::handler(ctx, amount, bet_yes)
    }

    pub fn resolve_market(ctx: Context<ResolveMarket>, outcome: bool) -> Result<()> {
        instructions::resolve_market::handler(ctx, outcome)
    }
}
