use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::approve_checked;
use anchor_spl::token::transfer_checked;
use anchor_spl::token::ApproveChecked;
use anchor_spl::token::CloseAccount;
use anchor_spl::token::TransferChecked;
use anchor_spl::token_interface::TokenAccount;
use anchor_spl::token_interface::Mint;
use anchor_spl::token_interface::TokenInterface;

use crate::OfferVault;


#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    //
    // Maker's tokens
    //
    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_b: Box<InterfaceAccount<'info, TokenAccount>>,

    //
    // Taker's tokens
    // 
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_b: Box<InterfaceAccount<'info, TokenAccount>>,


    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_a,
        has_one = token_mint_b
    )]
    pub offer_vault: Account<'info, OfferVault>,


    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

pub fn delegate_tokens(ctx: &Context<TakeOffer>) -> Result<()> {
    let approve_accounts = ApproveChecked {
        to: ctx.accounts.taker_token_b.to_account_info(),
        mint: ctx.accounts.token_mint_b.to_account_info(),
        delegate: ctx.accounts.offer_vault.to_account_info(),
        authority: ctx.accounts.taker.to_account_info()
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        approve_accounts
    );

    approve_checked(
        cpi_context,
        ctx.accounts.offer_vault.token_b_wanted_amount, 
        ctx.accounts.token_mint_b.decimals
    )
}

pub fn exchange_tokens(ctx: &Context<TakeOffer>) -> Result<()> {
    let signer_seeds: [&[&[u8]]; 1] = [&[
        b"offer_vault",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &ctx.accounts.offer_vault.id.to_le_bytes()[..],
        &[ctx.accounts.offer_vault.bump]
    ]];

    let exhg_a_to_taker = TransferChecked {
        from: ctx.accounts.maker_token_a.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
        to: ctx.accounts.taker_token_a.to_account_info(),
        authority: ctx.accounts.offer_vault.to_account_info()
    };

    let cpi_a_to_taker = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        exhg_a_to_taker,
        &signer_seeds
    );

    transfer_checked(
        cpi_a_to_taker, 
        ctx.accounts.offer_vault.token_a_giving_amount, 
        ctx.accounts.token_mint_a.decimals
    )?;

    let exhg_b_to_maker = TransferChecked {
        from: ctx.accounts.taker_token_b.to_account_info(),
        mint: ctx.accounts.token_mint_b.to_account_info(),
        to: ctx.accounts.maker_token_b.to_account_info(),
        authority: ctx.accounts.offer_vault.to_account_info()
    };

    let cpi_b_to_maker = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        exhg_b_to_maker,
        &signer_seeds
    );

    transfer_checked(
        cpi_b_to_maker, 
        ctx.accounts.offer_vault.token_b_wanted_amount, 
        ctx.accounts.token_mint_b.decimals
    )
}