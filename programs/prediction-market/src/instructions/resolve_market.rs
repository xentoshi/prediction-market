use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::PredictionMarketError;

pub fn handler(ctx: Context<ResolveMarket>, outcome: bool) -> Result<()> {
    let market = &mut ctx.accounts.market;
    require!(!market.resolved, PredictionMarketError::MarketAlreadyResolved);
    require!(Clock::get()?.unix_timestamp >= market.end_time, PredictionMarketError::MarketStillOpen);

    market.resolved = true;
    market.outcome = Some(outcome);

    // Implement payout logic here

    Ok(())
}

#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    #[account(mut)]
    pub market: Account<'info, PredictionMarket>,
    pub user: Signer<'info>,
}