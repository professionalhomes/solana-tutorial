use anchor_lang::prelude::*;

declare_id!("6UBzv2xUq8QmSHM9dEgoVUyQ2wPAcU6ZE8GjKS1sJuGn");

#[program]
pub mod secondday {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,
                    a:u64,
                    b:u64,
                    y:u64,
                    z:u64,
                    message:String
                    ) -> Result<()> {
                        let x: u64 = y + z; // will silently overflow
                        let xSafe: u64 = y.checked_add(z).unwrap();
        msg!("You said {:?}", message);
        msg!("You sent {} and {}", a, b);
        msg!("result of add {} and {}", x, xSafe);

        Ok(())
    }
    pub fn array(ctx: Context<Initialize>,
        arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
        }
}

#[derive(Accounts)]
pub struct Initialize {}
