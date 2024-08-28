use anchor_lang::prelude::*;

declare_id!("3v8pjuLoRLX112cZw5XiyrsGYckDoLFRtaPyHvbqgZFU");

#[program]
pub mod prediction_market {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
