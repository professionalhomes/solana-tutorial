use anchor_lang::prelude::*;

declare_id!("GCe9NmdJEAs38CPTZeh5JqToKY2kD9iKqoYjLQtA5jxL");

#[program]
pub mod fivethday {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello World!, Nice to meet you");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
