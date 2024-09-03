use anchor_lang::prelude::*;

declare_id!("CB6VEmLmzDJvzRzhGnFBh6HfH1cMY1rt2vc9T9YjAWPw");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
