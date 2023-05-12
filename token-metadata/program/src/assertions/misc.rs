use crate::error::MetadataError;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_option::COption,
    program_pack::{
        IsInitialized,
        Pack,
    },
    pubkey::Pubkey,
    rent::Rent,
};

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    //mpl_utils::assert_owned_by(account, owner, MetadataError::IncorrectOwner)
    if account.owner != owner {
        Err(MetadataError::IncorrectOwner.into())
    } else {
        Ok(())
    }
}
