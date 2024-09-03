use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::PredictionMarketError;

pub fn handler(ctx: Context<PlaceBet>, amount: u64, bet_yes: bool) -> Result<()> {
    let market = &mut ctx.accounts.market;
    require!(!market.resolved, PredictionMarketError::MarketAlreadyResolved);
    require!(Clock::get()?.unix_timestamp < market.end_time, PredictionMarketError::MarketClosed);

    if bet_yes {
        market.yes_amount += amount;
    } else {
        market.no_amount += amount;
    }

    // Transfer funds from bettor to market account
    // Implement fund transfer logic here

    Ok(())
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub market: Account<'info, PredictionMarket>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}