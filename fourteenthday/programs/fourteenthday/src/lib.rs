use anchor_lang::prelude::*;

declare_id!("62j8ueb6kF2pztYDgTYSD3fY7j1CRneCZJCZwzRot7Mz");

#[program]
pub mod fourteenthday {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        msg!("The signer1: {:?}", *the_signer1.key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
}