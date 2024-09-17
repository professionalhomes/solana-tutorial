use anchor_lang::prelude::*;

declare_id!("H7BFV3mP7dDBkVnyKoP62eRLSRwYp174jfTEwRd3u2M6");

#[program]
pub mod basic_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
