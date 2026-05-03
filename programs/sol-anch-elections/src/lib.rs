use anchor_lang::prelude::*;

declare_id!("GaQ54Jta5EcPTbizuNAA6Cq1DZbxUUVfhTJoHSzC8roF");

#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}

// accounts required by the program
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
// accounts created by the program
#[account]
pub struct NewAccount {
    data: u64,
}
