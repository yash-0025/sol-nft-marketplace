use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata as MetadataAccount, mpl_token_metadata::types::DataV2};


declare_id!("CRdno1yNWDzcs4LJEz3VxLG6sC1fN311EG5PqxjQKTSn");

#[program]
pub mod nft_marketplace {
    use super::*;

    // 1.  Initialize marketplace
    

    // 2. Mint NFT with Metadata

    // 3. List NFT for sale

    // 4. Buy NFT from listings

    // 5. Cancel NFT listing

}


#[derive(Accounts)]
pub struct Initialize {}



// STATE ACCOUNTS
#[account]
#[derive(InitSpace)]
pub struct MarketPlace {
    pub authority: Pubkey,
    pub fee_percentage: u16,
    pub total_sales: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub is_active: bool,
    pub bump: u8,
}


#[error_code]
pub enum MarketplaceError {
    #[msg("Listing is not active")]
    ListingNotActive,
    #[msg("Unauthorized")]
    Unauthorized,
}


