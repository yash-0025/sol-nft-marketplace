#  Step 1 - Understanding Basics

### PDA - Program Derived Addresses
- Deterministic addresses derived from seeds
- Only your program can sign for them
- Used for storing state(marketplace, listings)
- Eg : [b"marketplace"] creates a unique PDA

### CPI - Cross Program Invocations
- Program calling other program (Token Program, Metaplex)
- Like Function calls between programs
- Used for minting , transferring tokens, creating metadatas

### Account Validation
- Anchor validates accounts before your functions runs
- Ensures security and correctness
- Uses #[derive(Accounts)] structs


# Step 2 - Minting NFT's with Metaplex metadata
 
### What is NFT on Solana?
- An SPL Token mint with supply = 1 and decimals = 0
- A metadata account with name, image, attributes, etc.

**Two programs work together**
-  SPL token program creates the mint and token account.
-  Metaplex Token Metadata program creates the metadata account.

### Understanding Token Program basics
1. SPL Token Program basics
- Mint Account: 
    - Represents a token type [like a currency].
    - Stores supply, decimals, mint authority
    - For NFT's supply = 1 , decimals = 0
- Token Account:
    - Holds tokens of a specific mint
    - Each user has separate token accounts per mint
    - For NFT's holds 1 token 
- Associated Token Account [ATA]
    - Standard way to derive token account address
    - Formula: [wallet, token_program, mint] -> ATA address
    - Makes it easy to find a user's token account for a mint
2. Metaplex Token Metadata Program
- Metaplex Token Metadata Program
    - Stores NFT info (name, symbol, image, URI, attributes, creators, royalties)
    - Linked to the mint via PDA derivation
    - Standard format: [b"metadata", metadata_program, mint] -> metadata PDA
3. Cross-Program Invocations [CPIs]
- Our Program calls other programs :
    - Token  Program to mint tokens
    - Metaplex Program to create metadata

```rust
    let cpi_accounts = SomeStruct {
        account1: ctx.accounts.account1.to_account_info(),
        account2: ctx.accounts.account2.to_account_info(),
        // ...
    };

    // 2. Get the program to call
    let cpi_program = ctx.accounts.other_program.to_account_info();

    // 3. Create CPI contexts
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // 4. Call the function
    some_function(cpi_ctx, parameters)?;
```

#### Imports
- anchor_spl::token - SPL Token Program helpers
- anchor_spl::associated_token - ATA helpers
- anchor_spl::metadata - Metaplex metadata helpers

