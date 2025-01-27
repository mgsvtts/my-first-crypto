use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, TokenAccount},
    token_2022::{Burn, CloseAccount, Token2022, TransferChecked},
};

declare_id!("Cinkm2bTijCvTyrRSVU7RcchdMVUtZ3C979FzCiBWbby");

#[program]
pub mod my_first_crypto {
    use anchor_spl::token_2022::{self};

    use super::*;

    pub fn stack(ctx: Context<Stack>, amount: u64, rate: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(rate > 0, ErrorCode::InvalidRate);

        token_2022::transfer_checked(ctx.accounts.into_cpi(), amount, ctx.accounts.mint.decimals)?;

        let user_pool_data = &mut ctx.accounts.user_pool_account;

        user_pool_data.user = ctx.accounts.user.key();
        user_pool_data.stacked = amount;
        user_pool_data.rate = rate;
        user_pool_data.stacked_at = Clock::get()?.unix_timestamp;
        user_pool_data.bump = ctx.bumps.user_pool_account;

        Ok(())
    }

    pub fn unstack(ctx: Context<Unstack>) -> Result<()> {
        let user_pool_data = &mut ctx.accounts.user_pool_account;

        let (to_return, to_burn) = calculate_stacking_result(
            user_pool_data.stacked,
            user_pool_data.rate,
            user_pool_data.stacked_at,
        )?;

        token_2022::transfer_checked(
            ctx.accounts.into_transfer_cpi(),
            to_return,
            ctx.accounts.mint.decimals,
        )?;

        token_2022::burn(ctx.accounts.into_burn_cpi(), to_burn)?;

        token_2022::close_account(ctx.accounts.into_close_cpi())?;

        Ok(())
    }
}

fn calculate_stacking_result(amount: u64, rate: u64, start: i64) -> Result<(u64, u64)> {
    let stacking_duration = Clock::get()?.unix_timestamp - start;

    let penalty = rate * stacking_duration as u64;

    let to_return = amount * penalty;
    let to_burn = amount - to_return;

    Ok((to_return, to_burn))
}

#[account]
#[derive(Debug)]
pub struct UserPoolAccount {
    pub user: Pubkey,
    pub stacked: u64,
    pub rate: u64,
    pub stacked_at: i64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(amount: u64, rate: u64)]
pub struct Stack<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [b"user_pool", user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8 + 1,
    )]
    pub user_pool_account: Account<'info, UserPoolAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_pool_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stack<'info> {
    fn into_cpi(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let accounts = TransferChecked {
            from: self.user_token_account.to_account_info(),
            to: self.token_pool_account.to_account_info(),
            authority: self.user.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        CpiContext::new(self.system_program.to_account_info(), accounts)
    }
}

#[derive(Accounts)]
pub struct Unstack<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_pool", user.key().as_ref()],
        bump = user_pool_account.bump,
    )]
    pub user_pool_account: Account<'info, UserPoolAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> Unstack<'info> {
    fn into_transfer_cpi(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let accounts = TransferChecked {
            from: self.pool_token_account.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.pool_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        CpiContext::new(self.token_program.to_account_info(), accounts)
    }

    fn into_burn_cpi(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let accounts = Burn {
            mint: self.mint.to_account_info(),
            from: self.pool_token_account.to_account_info(),
            authority: self.pool_token_account.to_account_info(),
        };

        CpiContext::new(self.token_program.to_account_info(), accounts)
    }

    fn into_close_cpi(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let accounts = CloseAccount {
            account: self.user_pool_account.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };

        CpiContext::new(self.token_program.to_account_info(), accounts)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Stack rate must be greater than zero")]
    InvalidRate,
}
