use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        escrow_account.initializer = *ctx.accounts.initializer.key;
        escrow_account.amount = amount;
        msg!("Escrow Initialized for amount: {}", amount);
        Ok(())
    }
}

#[account]
pub struct EscrowAccount {
    pub initializer: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, space = 8 + 32 + 8)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
