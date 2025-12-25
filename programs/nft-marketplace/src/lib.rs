use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{
    create_metadata_accounts_v3,
    CreateMetadataAccountsV3,
    Metadata as MetadataAccount,
    mpl_token_metadata::types::DataV2,
};

declare_id!("CRdno1yNWDzcs4LJEz3VxLG6sC1fN311EG5PqxjQKTSn");

#[program]
pub mod nft_marketplace {
    use super::*;

// Creates the Marketplace PDA and sets initial values
    pub fn initialize(ctx: Context<InitializeMarketplace>, fee_percentage: u16) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = ctx.accounts.authority.key();
        marketplace.fee_percentage = fee_percentage;
        marketplace.total_sales = 0;
        marketplace.bump = ctx.bumps.marketplace;

        msg!("MarketPlace Initialized with {}% fees", fee_percentage as f64 / 100.0);
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNFT>, name: String, symbole: String, uri: String, ) -> Result<()> {
        // 1. Mint 1
    }
}

// State Structures
#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub authority: Pubkey,  // Address of who controls the marketplace
    pub fee_percentage: u16,    // Fee in basis points [100 = 1%]
    pub total_sales: u64, // counter  for total sales
    pub bump: u8, // PDA bump seed
}

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(
        init,               // Create new Account
        payer = authority, space = 8 + Marketplace::INIT_SPACE,  // Who pays for account creation
        seeds=[b"marketplace"],       // PDA seeds
        bump,                         // Auto-find bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(mut)]          // Mutable (will pay for account)
    pub authority: Signer<'info>,  //  Must sign transaction

    pub system_program: Program<'info, System>,   // Needed for account creation
 
}
