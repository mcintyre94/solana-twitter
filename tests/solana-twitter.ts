import * as anchor from "@project-serum/anchor";
import { AnchorError, Program } from "@project-serum/anchor";
import { assert } from "chai";
import { SolanaTwitter } from "../target/types/solana_twitter";

describe("solana-twitter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaTwitter as Program<SolanaTwitter>;

  it('can send a new tweet', async () => {
    const tweet = anchor.web3.Keypair.generate()

    await program.methods
      .sendTweet('veganism', 'Hummus, am I right?')
      .accounts({
        tweet: tweet.publicKey,
        author: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([tweet])
      .rpc()

    const tweetAccount = await program.account.tweet.fetch(tweet.publicKey)

    assert.equal(tweetAccount.author.toBase58(), provider.wallet.publicKey.toBase58())
    assert.equal(tweetAccount.topic, 'veganism')
    assert.equal(tweetAccount.content, 'Hummus, am I right?')
    assert.ok(tweetAccount.timestamp)
  });

  it('can send a new tweet without a topic', async () => {
    const tweet = anchor.web3.Keypair.generate()

    await program.methods
      .sendTweet('', 'gm')
      .accounts({
        tweet: tweet.publicKey,
        author: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([tweet])
      .rpc()

    const tweetAccount = await program.account.tweet.fetch(tweet.publicKey)

    assert.equal(tweetAccount.author.toBase58(), provider.wallet.publicKey.toBase58())
    assert.equal(tweetAccount.topic, '')
    assert.equal(tweetAccount.content, 'gm')
    assert.ok(tweetAccount.timestamp)
  });

  it('can send a new tweet from a different author', async () => {
    const otherUser = anchor.web3.Keypair.generate()
    const sig = await provider.connection.requestAirdrop(otherUser.publicKey, anchor.web3.LAMPORTS_PER_SOL)
    await provider.connection.confirmTransaction(sig)

    const tweet = anchor.web3.Keypair.generate()
    await program.methods
      .sendTweet('veganism', 'Hummus, am I right?')
      .accounts({
        tweet: tweet.publicKey,
        author: otherUser.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([otherUser, tweet])
      .rpc()

    const tweetAccount = await program.account.tweet.fetch(tweet.publicKey)

    assert.equal(tweetAccount.author.toBase58(), otherUser.publicKey.toBase58())
    assert.equal(tweetAccount.topic, 'veganism')
    assert.equal(tweetAccount.content, 'Hummus, am I right?')
    assert.ok(tweetAccount.timestamp)
  });

  it('cannot provide a topic with more than 50 characters', async () => {
    const tweet = anchor.web3.Keypair.generate()
    const longTopic = 'x'.repeat(51)

    try {
      await program.methods
        .sendTweet(longTopic, 'gm')
        .accounts({
          tweet: tweet.publicKey,
          author: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([tweet])
        .rpc()
    } catch (error) {
      assert.equal(error.error.errorMessage, 'The provided topic should be 50 characters long maximum.')
      return
    }

    assert.fail('The instruction should have failed with a 51-character topic.');
  })

  it('cannot provide a content with more than 280 characters', async () => {
    const tweet = anchor.web3.Keypair.generate()
    const longContent = 'x'.repeat(281)

    try {
      await program.methods
        .sendTweet('gm', longContent)
        .accounts({
          tweet: tweet.publicKey,
          author: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([tweet])
        .rpc()
    } catch (error) {
      assert.equal(error.error.errorMessage, 'The provided content should be 280 characters long maximum.')
      return
    }

    assert.fail('The instruction should have failed with a 281-character content.');
  })
});
