use anchor_lang::prelude::*;

declare_id!("D3YdyXTsbpeUcjp5hUU6YVgAUxq7VXADtCUqRkrsW7tG");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
