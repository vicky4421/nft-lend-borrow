use anchor_lang::prelude::*;

declare_id!("CXViv1YMFLyt4vRPGCjNxCK4UQZQJSfDAXV1J8HA6kzD");

#[program]
pub mod nft_lend_borrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
