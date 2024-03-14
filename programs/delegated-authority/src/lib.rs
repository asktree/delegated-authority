use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

declare_id!("5Pf7TWveTKfvquduUiFv4GYoQyMxuYHF14iqnXFhrTBv");

#[program]
pub mod delegated_authority {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        delegate: Pubkey,
        delegation_authority: Pubkey,
    ) -> Result<()> {
        let delegation_record = &mut ctx.accounts.delegation_record;
        **delegation_record = DelegationRecord {
            delegate,
            delegation_authority,
            seed_key: *ctx.accounts.seed_key.to_account_info().key,
        };
        Ok(())
    }

    pub fn update_delegate(ctx: Context<UpdateDelegate>, new_delegate: Pubkey) -> Result<()> {
        let delegation_record = &mut ctx.accounts.delegation_record;
        delegation_record.delegate = new_delegate;
        Ok(())
    }

    pub fn update_delegation_authority(
        ctx: Context<UpdateDelegationAuthority>,
        new_delegation_authority: Pubkey,
    ) -> Result<()> {
        let delegation_record = &mut ctx.accounts.delegation_record;
        delegation_record.delegation_authority = new_delegation_authority;
        Ok(())
    }

    pub fn execute(ctx: Context<Execute>, ix: Instruction) -> Result<()> {
        let delegation_record = &ctx.accounts.delegation_record;

        let program_id = ix.program_id;
        let accounts = ix.accounts;
        let data = ix.data;
        let seeds = &[timelock_key.as_ref(), &[ctx.accounts.timelock.signer_bump]];
        let signer = &[&seeds[..]];
        let accounts = ctx.remaining_accounts;
        solana_program::program::invoke_signed(&ix, accounts, signer)?;
        Ok(())
    }
}

#[account]
pub struct DelegationRecord {
    pub delegate: Pubkey,
    pub delegation_authority: Pubkey,
    pub seed_key: Pubkey,
}

#[derive(Accounts)]
#[instruction(delegate: Pubkey, delegation_authority: Pubkey)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    seed_key: Signer<'info>,
    #[account(
        init, 
        payer = payer, 
        space = 8 + 8 + 32 + 32 + 32,
        seeds = [seed_key.key().as_ref()],
        bump,
    )]
    delegation_record: Account<'info, DelegationRecord>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(new_delegate: Pubkey)]
pub struct UpdateDelegate<'info> {
    delegation_authority: Signer<'info>,
    #[account(
        mut,
        has_one = delegation_authority,
    )]
    delegation_record: Account<'info, DelegationRecord>,
}

#[derive(Accounts)]
#[instruction(new_delegation_authority: Pubkey)]
pub struct UpdateDelegationAuthority<'info> {
    delegation_authority: Signer<'info>,
    #[account(
        mut,
        has_one = delegation_authority,
    )]
    delegation_record: Account<'info, DelegationRecord>,
}

#[derive(Accounts)]
#[instruction(ix: Instruction)]
pub struct Execute<'info> {
    delegation: Signer<'info>,
    #[account(
        mut,
        has_one = delegation,
    )]
    delegation_record: Account<'info, DelegationRecord>,
}
