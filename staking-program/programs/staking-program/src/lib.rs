use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("3KwcL4cGq1yuM8s4Wt5ZfpsH53SfSPtu874CLq8PtU8S");

#[program]
pub mod staking_program {
    use super::*;

    /// This is the initializer for our staking program, you can think of it like 
    /// a constructor.
    pub fn initialize(ctx: Context<Initialize>, lockup_period: u64) -> Result<()> {
        msg!("Instruction: Initialize");

        let pool_info = &mut ctx.accounts.pool_info;

        pool_info.admin = ctx.accounts.admin.key();
        pool_info.lockup_period = lockup_period;
        pool_info.token = ctx.accounts.staking_token.key();

        Ok(())
    }
}

/// This struct is used to store the accounts that are passed to the initializer.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The admin field stores the admin's account.
    #[account(mut)]
    pub admin: Signer<'info>,
    /// The pool_info field stores the pool's account.
    #[account(init, payer = admin, space = 8 + PoolInfo::LEN)]
    pub pool_info: Account<'info, PoolInfo>,
    /// The staking_token field stores the staking token's account.
    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, Mint>,
    ///The admin_staking_wallet field stores the admin's staking wallet account.
    #[account(mut)]
    pub admin_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    /// The system_program field stores the system program account.
    pub system_program: Program<'info, System>,
}



/// This struct is used to store the pool's information.
#[account]
pub struct PoolInfo {
    /// The admin field stores the public key of the admin.
    pub admin: Pubkey,
    /// The lockup_period field stores the lockup period of the pool.
    pub lockup_period: u64,
    /// The token field stores the public key of the staking token.
    pub token: Pubkey,
}

/// The UserInfo struct stores the user's staking information.
#[account]
pub struct UserInfo {
    /// The amount field stores the amount of staking tokens the user has staked.
    pub amount: u64,
    /// The reward_claimed field stores the amount of rewards the user has already claimed.
    pub reward_debt: u64,
    /// The start_time field stores the time the user started staking.
    pub start_time: u64,
}

impl UserInfo {
    pub const LEN: usize = 8 + 8 + 8;
}

impl PoolInfo {
    pub const LEN: usize = 32 + 8 + 8 + 32;
}