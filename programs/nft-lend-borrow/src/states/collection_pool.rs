use anchor_lang::prelude::*;

#[account]
pub struct CollectionPool {
    /// NFT collectin ID
    pub collection_id: Pubkey,      // 32 bytes

    /// Pool Owner
    pub pool_owner: Pubkey,         // 32 bytes

    /// Loan Duration
    pub duration: i64,              // 8 bytes

    /// Total Loans
    pub total_offers : u64,         // 8 bytes

    /// Bump
    pub bump: u8                    // 1 byte
}

impl CollectionPool {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1;
}