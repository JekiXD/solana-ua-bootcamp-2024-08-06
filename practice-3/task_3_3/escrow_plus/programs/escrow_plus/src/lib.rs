pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2GwmwMwY8AgMtKcPG7CP7CBHtUcYV7ANqakTUtyuRVJL");

#[program]
pub mod escrow_plus {
    use super::*;


    pub fn make_offer(ctx: Context<MakeOffer>, id: u64, token_a_giving_amount: u64, token_b_wanted_amount: u64) -> Result<()> {
        instructions::make_offer(ctx, id, token_a_giving_amount, token_b_wanted_amount)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::delegate_tokens(&ctx)?;
        instructions::take_offer::exchange_tokens(&ctx)
    }
}
