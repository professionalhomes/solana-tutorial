use anchor_lang::prelude::*;

declare_id!("5dicJHfrtiHLNJBGe6v7eAYkr9kgoW9m95B3MEyViTxt");

#[program]
pub mod compute_unit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut a = Vec::new();
        a.push(1);
        a.push(2);
        a.push(3);
        a.push(4);
        a.push(5);

        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
