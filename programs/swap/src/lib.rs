pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("C6E82rhkqDHB3PsccYffy9A3HyU9sBnT6gkWTNcrERBT");

#[program]
pub mod swap {
    use instruction::MakeOffer;

    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault()?;
    }
}
