use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Election {
    pub name: String,
    pub candidates: Vec<String>,
    pub votes: Vec<u64>,
    pub voters: Vec<Pubkey>,
}
