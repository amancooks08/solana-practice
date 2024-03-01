use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token};

declare_id!("78D2NexR2uwVAauba88UrPs7cWKnJtvGyjKsUbqpdXsJ");

#[program]
pub mod spl_token {
    use anchor_lang::system_program;
    use anchor_spl::{token::{initialize_mint, InitializeMint, mint_to, MintTo}, associated_token};

    use super::*;

    /**
        The create_token function will mint the new SPL token with in a mint 
        account account and associated token account.
     */
    pub fn create_token(ctx: Context<CreateToken>,decimals:u8,amount:u64) -> Result<()> {

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), 
                system_program::CreateAccount { from: ctx.accounts.signer.to_account_info(), to: ctx.accounts.mint_token.to_account_info() }
            ), 
            10_000_000, 
            82, 
            ctx.accounts.token_program.key
        )?;

        initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint{mint:ctx.accounts.mint_token.to_account_info(),rent:ctx.accounts.rent.to_account_info()}
            ), 
            decimals, 
            ctx.accounts.signer.key, 
            Some(ctx.accounts.signer.key)
        )?;

        // In the create_token function first creating the system account for the mint 
        // address and initializing mint account by calling function initialize_mint.
        // After successfully mint account created the next it will create associated 
        // token account by calling function associated_token::create.

        associated_token::create(
            CpiContext::new(
                ctx.accounts.associate_token_program.to_account_info(), 
                associated_token::Create { 
                    payer: ctx.accounts.signer.to_account_info(), 
                    associated_token: ctx.accounts.token_account.to_account_info(), 
                    authority: ctx.accounts.signer.to_account_info(), 
                    mint: ctx.accounts.mint_token.to_account_info(), 
                    system_program: ctx.accounts.system_program.to_account_info(), 
                    token_program: ctx.accounts.token_program.to_account_info() 
                }
            )
        )?;

        // Now we have mint account and token account so we need to add supply of 
        // tokens to the mint account. By calling mint_to function we can set number
        // of tokens need to add in circulation. In create_token function have two 
        // parameters decimals[0–9] and amount(supply).

        mint_to(
            CpiContext::new(
                ctx.accounts.token_account.to_account_info(), 
                MintTo{authority:ctx.accounts.signer.to_account_info(),mint:ctx.accounts.mint_token.to_account_info(),to:ctx.accounts.token_account.to_account_info()}
            ), 
            amount
        )?;

        Ok(())
    }
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub mint_token:Signer<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(mut)]
    pub token_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    pub rent:Sysvar<'info,Rent>
}