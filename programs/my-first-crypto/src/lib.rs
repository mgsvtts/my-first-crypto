use anchor_lang::prelude::*;

declare_id!("Cinkm2bTijCvTyrRSVU7RcchdMVUtZ3C979FzCiBWbby");

#[program]
pub mod my_first_crypto {
    use super::*;

    pub fn create(ctx: Context<Create>, message: String) -> Result<()> {
        msg!("Start of creating message, {}", message);
        let message_account_data = &mut ctx.accounts.message_account;
        message_account_data.user = ctx.accounts.user.key();
        message_account_data.message = message;
        message_account_data.bump = ctx.bumps.message_account;
        Ok(())
    }
    pub fn update(ctx: Context<Update>, message: String) -> Result<()> {
        msg!("Start of updating message, {}", message);
        let message_account_data = &mut ctx.accounts.message_account;
        message_account_data.message = message;
        Ok(())
    }
    pub fn delete(_ctx: Context<Delete>) -> Result<()> {
        msg!("Start of deletion message, {}");
        Ok(())
    }
}

#[account]
pub struct MessageAccount {
    pub user: Pubkey,
    pub message: String,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(message: String)]
pub struct Create<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [b"message", user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 32 + 4 + message.len() + 1
    )]
    pub message_account: Account<'info, MessageAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(message: String)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"message", user.key().as_ref()],
        bump = message_account.bump,
        realloc = 8 + 32 + 4 + message.len() + 1,
        realloc::payer = user,
        realloc::zero = true,
    )]
    pub message_account: Account<'info, MessageAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"message", user.key().as_ref()],
        bump = message_account.bump,
        close= user,
    )]
    pub message_account: Account<'info, MessageAccount>,
}
