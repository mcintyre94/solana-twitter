use anchor_lang::prelude::*;

declare_id!("D3YdyXTsbpeUcjp5hUU6YVgAUxq7VXADtCUqRkrsW7tG");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

// every account has an 8byte discriminator 
const DISCRIMINATOR_LENGTH: usize = 8;

// public key is pub struct Pubkey([u8; 32]);, array of 32 * 8-bit unsigned ints
// ie 32 bytes
const PUBLIC_KEY_LENGTH: usize = 32;

// i64 is 64-bit integer, 8 bytes
const TIMESTAMP_LENGTH: usize = 8;

// All strings have a 4-byte prefix containing their length
// Strings are vectors with unbounded length, so we have to store the length
const STRING_LENGTH_PREFIX: usize = 4; 

// 50 chars max for topic, each UTF-8 char is up to 4 bytes
const MAX_TOPIC_LENGTH: usize = 50 * 4;

// 280 chars max for content, each UTF-8 char is up to 4 bytes
const MAX_CONTENT_LENGTH: usize = 280 * 4;

impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
}