use anchor_lang::prelude::*;

declare_id!("DKSArRiSwe78AWstaCBfSpUKJNYR3ZLQ5aydAwki2nEA");

#[program]
pub mod emit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        emit!(MyEvent { value: 42 });
        emit!(MySecondEvent { value: 3, message: "hello world".to_string() });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[event]
pub struct MyEvent {
    pub value: u64,
}

#[event]
pub struct MySecondEvent {
		pub value: u64,
    pub message: String,
}