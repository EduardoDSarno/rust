
use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey
};
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct State {
    pub is_initialized: bool,
    pub authority: Pubkey,
}

impl State {
    pub const LEN: usize = 33; // 1 + 32

    pub fn new(is_initialized: bool, authority: Pubkey) ->Self{
        State { is_initialized, authority}
    }
}