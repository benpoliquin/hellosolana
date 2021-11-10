use anchor_lang::prelude::*;

declare_id!("EHyhQcq2wU6Vtavi8eJBUis4PnfcAuKU2tMNzG7LK7hy");

#[program]
pub mod helloworld {
    use super::*;
    pub fn start_stuff(ctx: Context<StartStuff>) -> ProgramResult {
        // set total = 0
        let base_account = &mut ctx.accounts.base_account;
        base_account.total = 0;

        Ok(())
    }

    // total += 1
    pub fn add_one(ctx: Context<AddOne>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total += 1;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuff<'info> {
    #[account(init, payer = user, space = 8 + 8)] // first 8 bytes are reserved for a unique account discriminator.
    pub base_account: Account<'info, BaseAccount>,

    // Signer and System Program
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddOne<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    // #[account(mut)]
    // pub user: Signer<'info>,
}

#[account]
pub struct BaseAccount {
    pub total: u64,
}
