use borsh::{BorshSerialize, BorshDeserialize};
use super::state::State;
use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey
};

use crate::program::instructions::verificaiton::verify;




// Handler functions
pub fn initialize<'a>(accounts: &'a [AccountInfo<'a>], program_id: &Pubkey) -> ProgramResult 
{
    // using verified to make code clean and simpler
    let verified = verify(accounts, program_id)?;

    if verified.state_data.is_initialized{
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Create new initialized state
    let new_state = State {
        is_initialized: true,
        authority: *verified.signer.key,
    };
    
    // Serialize to account
    new_state.serialize(&mut &mut verified.state_account.data.borrow_mut()[..])?;
    
    Ok(())
}

pub fn add(_accounts: &[AccountInfo]) -> ProgramResult {
    // TODO: Implement add logic
    Ok(())
}

pub fn subtract(_accounts: &[AccountInfo]) -> ProgramResult {
    // TODO: Implement subtract logic
    Ok(())
}

pub fn approve(_accounts: &[AccountInfo]) -> ProgramResult {
    // TODO: Implement approve logic
    Ok(())
}
