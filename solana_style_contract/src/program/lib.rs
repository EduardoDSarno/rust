use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize,BorshSerialize,Debug)]
pub 
struct MyAccount{
    // for account type and future upgrades
    pub version: u8,
    pub authority: [u8;32], // pubkey bytes

    pub score: u64,
}