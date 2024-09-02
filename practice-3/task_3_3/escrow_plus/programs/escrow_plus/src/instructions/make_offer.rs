use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{approve_checked, ApproveChecked}, token_interface::{TokenAccount, TokenInterface}};
use anchor_spl::token_interface::Mint;

use crate::{OfferVault, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
#[instruction(id: u64, token_a_giving_amount: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub  token_mint_a: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub  token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
        constraint = maker_token_a.amount >= token_a_giving_amount
    )]
    pub  maker_token_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR * OfferVault::INIT_SPACE,
        seeds = [b"offer_vault", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer_vault: Account<'info, OfferVault>,


    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

pub fn make_offer(ctx: Context<MakeOffer>, id: u64, token_a_giving_amount: u64, token_b_wanted_amount: u64) -> Result<()> {
    let approve_accounts = ApproveChecked {
        to: ctx.accounts.maker_token_a.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        delegate: ctx.accounts.offer_vault.to_account_info(),
        authority: ctx.accounts.maker.to_account_info()
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        approve_accounts
    );

    approve_checked(cpi_context, token_a_giving_amount, ctx.accounts.token_mint_a.decimals)?;

    ctx.accounts.offer_vault.set_inner(OfferVault { 
        id, 
        maker: ctx.accounts.maker.key(), 
        token_mint_a: ctx.accounts.token_mint_a.key(), 
        token_mint_b: ctx.accounts.token_mint_b.key(), 
        token_a_giving_amount, 
        token_b_wanted_amount, 
        bump: ctx.bumps.offer_vault 
    });
    Ok(())
}