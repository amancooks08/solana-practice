use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("3KwcL4cGq1yuM8s4Wt5ZfpsH53SfSPtu874CLq8PtU8S");

#[program]
pub mod staking_program {
    use super::*;

    /// This is the initializer for our staking program, you can think of it like 
    /// a constructor.
    pub fn initialize(ctx: Context<Initialize>, lockup_period: u64, reward_rate: u64) -> Result<()> {
        msg!("Instruction: Initialize");

        let pool_info = &mut ctx.accounts.pool_info;

        pool_info.admin = ctx.accounts.admin.key();
        pool_info.reward_rate = reward_rate;
        pool_info.lockup_period = lockup_period;
        pool_info.token = ctx.accounts.staking_token.key();

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        msg!("Instruction: Stake");

        // Check if the amount is greater than 0
        if amount == 0 {
            return Err(MyErrors::AmountMustBeGreaterThanZero.into());
        }

        let user_info = &mut ctx.accounts.user_info;
        let pool_info = &mut ctx.accounts.pool_info;
        let clock = Clock::get()?;
        if user_info.amount > 0 {
            update_reward(user_info, pool_info)?;

            let cpi_accounts = MintTo {
                mint: ctx.accounts.staking_token.to_account_info(),
                to: ctx.accounts.user_staking_wallet.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::mint_to(cpi_ctx, user_info.reward_debt)?;
            // Change the state of rewards in the user_info
            user_info.reward_debt = 0;
        }
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_staking_wallet.to_account_info(),
            to: ctx.accounts.admin_staking_wallet.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        user_info.amount += amount;
        user_info.start_time = clock.slot;
        user_info.reward_debt = 0;

        Ok(())
    }


    /// This is the unstake instruction, it is used to unstake tokens from the pool.
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        msg!("Instruction: Unstake");

        let user_info = &mut ctx.accounts.user_info;
        let pool_info = &mut ctx.accounts.pool_info;
        
        update_reward(user_info, pool_info)?;

        let cpi_accounts = MintTo {
            mint: ctx.accounts.staking_token.to_account_info(),
            to: ctx.accounts.user_staking_wallet.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, user_info.reward_debt)?;

        let cpi_accounts = Transfer {
            from: ctx.accounts.admin_staking_wallet.to_account_info(),
            to: ctx.accounts.user_staking_wallet.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, user_info.amount)?;

        user_info.amount = 0;
        user_info.start_time = 0;
        user_info.reward_debt = 0;

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

fn update_reward(user_info: &mut UserInfo, pool_info: &PoolInfo) -> Result<()> {
    msg!("Instruction: RedeemReward");

    // Check if the amount is greater than 0
    if user_info.amount == 0 {
        return Err(MyErrors::AmountMustBeGreaterThanZero.into());
    }

    // calculate reward
    let reward = calculate_reward(user_info, pool_info)?;
    user_info.reward_debt += reward;
    user_info.start_time = Clock::get()?.slot;
    Ok(())
}

fn calculate_reward(user_info: &UserInfo, pool_info: &PoolInfo) -> Result<u64> {
    let clock = Clock::get()?;
    let elapsed_time = clock.slot - user_info.start_time;
    let reward = (user_info.amount * elapsed_time * pool_info.reward_rate) / (365 * 100);

    Ok(reward)
}

/// This is the staking instruction, it is used to stake tokens.
#[derive(Accounts)]
pub struct Stake<'info> {

    /// The user field stores the user's account.
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK:
    /// The admin field stores the admin's account.
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    /// The user_info field stores the user's staking information account.
    #[account(init, payer = user, space = 8 + UserInfo::LEN)]
    pub user_info: Account<'info, UserInfo>,

    /// The user_staking_wallet field stores the user's staking wallet account.
    #[account(mut)]
    pub user_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    
    /// The pool_info field stores the pool's account.
    #[account(mut)]
    pub pool_info: Account<'info, PoolInfo>,

    /// The admin_staking_wallet field stores the admin's staking wallet account.
    #[account(mut)]
    pub admin_staking_wallet: InterfaceAccount<'info, TokenAccount>,
    
    /// The staking_token field stores the staking token's account.
    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, Mint>,

    /// The token_program field stores the token program account.
    pub token_program: Interface<'info, TokenInterface>,

    /// The system_program field stores the system program account.
    pub system_program: Program<'info, System>,
}

/// This is the unstake instruction, it is used to unstake tokens.
#[derive(Accounts)]
pub struct Unstake<'info> {
    /// The user field stores the user's account.
    /// CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,

    /// The admin field stores the admin's account.
    /// CHECK:
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    /// The user_info field stores the user's staking information account.
    #[account(mut)]
    pub user_info: Account<'info, UserInfo>,

    /// The pool_info field stores the pool's account.
    #[account(mut)]
    pub pool_info: Account<'info, PoolInfo>,

    /// The user_staking_wallet field stores the user's staking wallet account.
    #[account(mut)]
    pub user_staking_wallet: InterfaceAccount<'info, TokenAccount>,

    /// The admin_staking_wallet field stores the admin's staking wallet account.
    #[account(mut)]
    pub admin_staking_wallet: InterfaceAccount<'info, TokenAccount>,

    /// The staking_token field stores the staking token's account.
    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, Mint>,

    /// The token_program field stores the token program account.
    pub token_program: Interface<'info, TokenInterface>,
}

/// This struct is used to store the pool's information.
#[account]
pub struct PoolInfo {

    /// The admin field stores the public key of the admin.
    pub admin: Pubkey,

    /// The reward_rate field stores the reward rate of the pool.
    pub reward_rate: u64,

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

/// This enum is used to store the error codes that can be returned by the program.
/// It is used to provide more context to the user when an error occurs.
#[error_code]
pub enum MyErrors {
    #[msg("The amount must be greater than 0")]
    AmountMustBeGreaterThanZero,
}