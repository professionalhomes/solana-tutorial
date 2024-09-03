use anchor_lang::prelude::*;

declare_id!("6UBzv2xUq8QmSHM9dEgoVUyQ2wPAcU6ZE8GjKS1sJuGn");

#[program]
pub mod secondday {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
