use anchor_lang::prelude::*;
pub mod states;
pub use states::*;

pub mod instructions;
pub use instructions::*;

pub mod errors;
pub use errors::ErrorCodes;

declare_id!("CXViv1YMFLyt4vRPGCjNxCK4UQZQJSfDAXV1J8HA6kzD");

#[program]
pub mod nft_lend_borrow {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>, 
        collection_id: Pubkey, 
        duration: i64
    ) -> Result<()> {
        instructions::create_pool::handler(ctx, collection_id, duration)
    }

    pub fn offer_loan(ctx: Context<OfferLoan>, offer_amount: u64) -> Result<()> {
        instructions::offer_loan::handler(ctx, offer_amount)
    }
}
