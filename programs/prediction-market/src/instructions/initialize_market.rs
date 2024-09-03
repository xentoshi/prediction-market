use anchor_lang::prelude::*;
use crate::state::*;

pub fn handler(ctx: Context<InitializeMarket>, question: String, end_time: i64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    market.question = question;
    market.end_time = end_time;
    market.yes_amount = 0;
    market.no_amount = 0;
    market.resolved = false;
    market.outcome = None;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 1 + 1 + 200)]
    pub market: Account<'info, PredictionMarket>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

