use anchor_lang::prelude::*;

declare_id!("9hJn93HEodU1L8GoDYXNaQhyrstF17VqLqNzc1fFT7AQ");

#[program]
pub mod firstday {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
