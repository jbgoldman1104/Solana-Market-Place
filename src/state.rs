use solana_program::{
    pubkey::Pubkey

};
use borsh::{BorshDeserialize, BorshSerialize};

pub const NFT_COUNT: usize = 12;
pub const NFT_RECORD_SIZE: usize = 250; // 133
pub const REPO_ACCOUNT_SEED: &str = "hallofheros";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NFTRecord{
    pub hero_id: u8,
    pub content_uri: String,
    pub key_nft: Pubkey,
    pub last_price: u64,
    pub listed_price: u64
}
