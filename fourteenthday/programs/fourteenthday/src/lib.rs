use anchor_lang::prelude::*;

declare_id!("62j8ueb6kF2pztYDgTYSD3fY7j1CRneCZJCZwzRot7Mz");

#[program]
pub mod fourteenthday {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;

        msg!("The signer1: {:?}", *the_signer1.key);
        msg!("The signer2: {:?}", *the_signer2.key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}