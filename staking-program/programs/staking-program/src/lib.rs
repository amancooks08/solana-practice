use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("3KwcL4cGq1yuM8s4Wt5ZfpsH53SfSPtu874CLq8PtU8S");

#[program]
pub mod staking_program {
    use super::*;

    /*
        This function is used to initialize the program, and the pool.
     */
    pub fn initialize(ctx: Context<Initialize>, lockup_period: u64) -> Result<()> {
        msg!("Instruction: Initialize");

        let pool_info = &mut ctx.accounts.pool_info;

        pool_info.admin = ctx.accounts.admin.key();
        pool_info.lockup_period = lockup_period;
        pool_info.token = ctx.accounts.staking_token.key();

        Ok(())
    }
}

#[derive(Accounts)]

/*
    This struct is used to store the accounts that are passed to the initialize function.
    The admin field stores the admin's account.
    The pool_info field stores the pool's account.
    The staking_token field stores the staking token's account.
    The admin_staking_wallet field stores the admin's staking wallet account.
    The system_program field stores the system program account.
 */
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = 8 + PoolInfo::LEN)]
    pub pool_info: Account<'info, PoolInfo>,
    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub admin_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}


/*
    This struct is used to store the pool's information.
    The admin field stores the admin's public key.
    The lockup_period field stores the lockup period of the pool.
    The token field stores the public key of the staking token.
*/
#[account]
pub struct PoolInfo {
    pub admin: Pubkey,
    pub lockup_period: u64,
    pub token: Pubkey,
}
/*
    This struct is used to store the user's staking information.
    The amount field stores the amount of staking tokens the user has staked.
    The reward_claimed field stores the amount of rewards the user has already claimed.
*/
#[account]
pub struct UserInfo {
    pub amount: u64,
    pub reward_debt: u64,
}

impl UserInfo {
    pub const LEN: usize = 8 + 8 + 8;
}

impl PoolInfo {
    pub const LEN: usize = 32 + 8 + 8 + 32;
}