use anchor_lang::prelude::*;

declare_id!("GaQ54Jta5EcPTbizuNAA6Cq1DZbxUUVfhTJoHSzC8roF");

#[program]
pub mod sol_anch_elections {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
