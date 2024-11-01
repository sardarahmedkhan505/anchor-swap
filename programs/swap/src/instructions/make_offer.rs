use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenInterface},
    token::{Mint,TokenAccount}
};

use crate::Offer;

#[derive(Accounts)]
pub struct MakeOffer<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a : InterfaceAccount<'info,Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b : InterfaceAccount<'info,Mint>,

    #[account(mut,
    associated_token::mint = token_mint_a,
    associated_token::authority = maker,
    associated_token::token_program = token_program
    )]
    pub maker_token_a : InterfaceAccount<'info,Mint>,
    #[account(
        init,
        payer = maker,
        space = 8 + Offer::INIT_SPACE,
        seeds = [b"offer",maker.key().as_ref(),id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer : Account<'info, System>
    
}

pub fn send_offered_tokens_to_vault(ctx: Context<Offer>) -> Result<()> {
    Ok(())
}
