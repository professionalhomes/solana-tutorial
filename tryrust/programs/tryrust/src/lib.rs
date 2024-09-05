use anchor_lang::prelude::*;

declare_id!("Gv6oGiZrAXqbY9fRv5iweVZQi8t7mGDAB5S8DAaSupi1");

#[program]
pub mod tryrust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
