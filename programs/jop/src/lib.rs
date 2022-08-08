use anchor_lang::prelude::*;


declare_id!("39dsge9boZb483PJALjX4jKLnVNoCqFBz3Lx6ar1HnVa");

#[program]
pub mod jop {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, vote_account_bump: u8) -> Result<()> {
        bump = ctx.accounts.vote_account.bump;

        Ok(())
    }

    pub fn vote_for_red(ctx: Context<VoteForRed>) -> Result<()> {
        ctx.accounts.vote_account.red += 1;
        Ok(())
    }

    pub fn vote_for_blue(ctx: Context<VoteForBlue>) -> Result<()> {
        ctx.accounts.vote_account.blue +=1;
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(vote_account_bump: u8)]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"jop".as_ref()], bump = bump, payer = user)]
    jop: Account<'info, VotingState>,
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut, seeds=[b"vote_account".as_ref(), bump = vote_account.bump])] 
    vote_account: Account<'info, VotingState>,
}

#[account]
#[derive(Default)]
pub struct VotingState {
    red: u64,
    blue: u64,
    bump: u8
}