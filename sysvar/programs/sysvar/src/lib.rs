use anchor_lang::prelude::*;

declare_id!("CUSj5Pft7H3C4bK4pCjbvvpmzqHUYS8m6ou7MMX1c54L");

#[program]
pub mod sysvar {
    use super::*;
    use anchor_lang::solana_program::clock;
    use chrono::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        msg!(
            "Block timestamp: {}",
            clock.unix_timestamp,
        );
        Ok(())
    }
    
    pub fn get_day_of_the_week(
        _ctx: Context<Initialize>) -> Result<()> {
    
        let clock: Clock= Clock::get()?;
        let time_stamp: i64 = clock.unix_timestamp; // current timestamp
    
        let date_time: NaiveDateTime = chrono::NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
        let day_of_the_week:Weekday = date_time.weekday();
    
        msg!("Week day is: {}", day_of_the_week);
    
        Ok(())
    }
    

}

#[derive(Accounts)]
pub struct Initialize {}
