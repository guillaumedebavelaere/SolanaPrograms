use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

declare_id!("2iyDKerqew6rBqQQBsZvmaMGPnQ8jSyRQqnUuAuMSspE");

#[program]
pub mod pyth {
    use super::*;

    pub fn sample(ctx: Context<Sample>) -> Result<()> {
        let price_update = &mut ctx.accounts.price_update;
        // get_price_no_older_than will fail if the price update is more than 30 seconds old
        let maximum_age: u64 = 30;
        // get_price_no_older_than will fail if the price update is for a different price feed.
        // This string is the id of the BTC/USD feed. See https://pyth.network/developers/price-feed-ids for all available IDs.
        let feed_id: [u8; 32] = get_feed_id_from_hex("0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43")?;
        let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;
        // Sample output:
        // The price is (8566897345520 ± 3941608267) * 10^-8'
        msg!("The price is ({} ± {}) * 10^{}", price.price, price.conf, price.exponent);
        
        // The formatted price is : (85668 +- 39)
        let display_price = u64::try_from(price.price).unwrap() / 10u64.pow(u32::try_from(-price.exponent).unwrap());
        let display_confidence = u64::try_from(price.conf).unwrap() / 10u64.pow(u32::try_from(-price.exponent).unwrap());
        msg!("The formatted price is : ({} +- {})", display_price, display_confidence);
     
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct Sample<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // Add this account to any instruction Context that needs price data.
    pub price_update: Account<'info, PriceUpdateV2>,
}
