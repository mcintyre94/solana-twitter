use anchor_lang::prelude::*;

declare_id!("D3YdyXTsbpeUcjp5hUU6YVgAUxq7VXADtCUqRkrsW7tG");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into());
        }

        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into());
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

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

#[error_code]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}