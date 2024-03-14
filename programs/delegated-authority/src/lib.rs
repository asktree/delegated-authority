use anchor_lang::prelude::*;

declare_id!("5Pf7TWveTKfvquduUiFv4GYoQyMxuYHF14iqnXFhrTBv");

#[program]
pub mod delegated_authority {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
