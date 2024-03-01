use anchor_lang::prelude::*;

declare_id!("78D2NexR2uwVAauba88UrPs7cWKnJtvGyjKsUbqpdXsJ");

#[program]
pub mod spl_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}