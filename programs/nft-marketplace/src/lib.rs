use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata as MetadataAccount, mpl_token_metadata::types::DataV2};


declare_id!("CRdno1yNWDzcs4LJEz3VxLG6sC1fN311EG5PqxjQKTSn");

#[program]
pub mod nft_marketplace {
    use super::*;

    // 1.  Initialize marketplace

    pub fn initialize_marketplace(ctx: Context<InitializeMarketPlace>, 
        fee_percentage: u16 // basis points (100 = 1%)
    ) -> Result<()> {
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.authority = ctx.accounts.authority.key();
        marketplace.fee_percentage = fee_percentage;
        marketplace.total_sales = 0;
        marketplace.bump = ctx.bumps.marketplace;
        
        msg!("Marketplace initialized with {}% fees", fee_percentage);
        Ok(())
    }


    // 2. Mint NFT with Metadata

    pub fn mint_nft(ctx: Context<MintNFT>,
        metadata_uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        // Mint 1 token to the user's token account
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        // Creating Metadata Account
        let data_v2 = DataV2 {
            name: name.clone(),
            symbol: symbol.clone(),
            seller_fee_basis_points: 500,
            creators: some(vec![
                mpl_token_metadata::types::Creator{
                    address: ctx.accounts.payer.key(),
                    verified: true,
                    share: 100,
                }
            ]),
            collection: None,
            uses: None,
        };

        let cpi_accounts = CreateMetadataAccountsV3{
            metadata: ctx.accounts.metadata.to_account_info(),
            mint: ctx.accounts.to_account_info(),
            mint_authority:ctx.accounts.payer.to_account_info(),
            update_authority: ctx.accounts.payer.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_metadata_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);


        create_metadata_accounts_v3(
            cpi_ctx,
            data_v2,
            true,
            true,
            None,
        );
        msg!("NFT minted: {} ({})", name, symbol);
        Ok(())
    }

    // 3. List NFT for sale

    // 4. Buy NFT from listings

    // 5. Cancel NFT listing

}

// CONTEXT [ACCOUNT VALIDATIONS]

#[derive(Accounts)]
pub struct InitializeMarketPlace<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [b"marketplace"],
        bump,
    )]
    pub marketplace: Account<'info, MarketPlace>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_metadata_program: UncheckedAccount<'info>,
}



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


