use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct State {
    pub is_initialized: bool,
    pub authority: Pubkey,
}

impl State {
    pub const LEN: usize = 33; // 1 + 32
}

// Handler functions
pub fn initialize(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult 
{

    let accounts_iter = &mut accounts.iter();

    let signer = next_account_info(accounts_iter)?; // first account 
    let state = next_account_info(accounts_iter)?; // second account (iterate over it)
    let token_program = next_account_info(accounts_iter)?; // third account toke nprogram
    
    // check if the transaction was signed by the account public key using is signer built in method
    if !signer.is_signer {
        return Err(solana_program::program_error::ProgramError::MissingRequiredSignature);
    }

    //only owner of the program is allowed to modify that accounts data
    if state.owner != program_id {
        return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
    }

    // account must be writable
    if !state.is_writable {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    // must verify account data size
    if state.data_len() != State::LEN {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }


    // create a initliazed state data
    let mut state_data = State {
        is_initialized: true,
        authority: *signer.key,
    };
    
        
    // Check if it is already inilizatizled
    let existing_data = State::try_from_slice(&state.data.borrow())?;

    if existing_data.is_initialized {
        return Err(solana_program::program_error::ProgramError::AccountAlreadyInitialized);
    }
    state_data.serialize(&mut &mut state.data.borrow_mut()[..])?; // this means “Take my Rust struct and write its bytes into the account.”

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
