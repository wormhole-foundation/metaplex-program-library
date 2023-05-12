pub mod collection;
pub mod creator;
pub mod data;
pub mod metadata;
pub mod uses;

pub use collection::*;
pub use creator::*;
pub use data::*;
pub use metadata::*;
pub use uses::*;

use crate::{
    assertions::misc::assert_owned_by,
    error::MetadataError,
    ID,
};
use borsh::{
    maybestd::io::Error as BorshError,
    BorshDeserialize,
    BorshSerialize,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
};
use std::io::ErrorKind;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, Copy, FromPrimitive)]
pub enum TokenStandard {
    NonFungible,             // This is a master edition
    FungibleAsset,           // A token with metadata that can also have attributes
    Fungible,                // A token with simple metadata
    NonFungibleEdition,      // This is a limited edition
    ProgrammableNonFungible, // NonFungible with programmable configuration
}

pub trait TokenMetadataAccount: BorshDeserialize {
    fn key() -> Key;

    fn size() -> usize;

    fn is_correct_account_type(data: &[u8], data_type: Key, data_size: usize) -> bool {
        if data.is_empty() {
            return false;
        }

        let key: Option<Key> = Key::from_u8(data[0]);
        match key {
            Some(key) => {
                (key == data_type || key == Key::Uninitialized) && (data.len() == data_size)
            }
            None => false,
        }
    }

    fn pad_length(buf: &mut Vec<u8>) -> Result<(), MetadataError> {
        let padding_length = Self::size()
            .checked_sub(buf.len())
            .ok_or(MetadataError::NumericalOverflowError)?;
        buf.extend(vec![0; padding_length]);
        Ok(())
    }

    fn safe_deserialize(mut data: &[u8]) -> Result<Self, BorshError> {
        if !Self::is_correct_account_type(data, Self::key(), Self::size()) {
            return Err(BorshError::new(ErrorKind::Other, "DataTypeMismatch"));
        }

        let result = Self::deserialize(&mut data)?;

        Ok(result)
    }

    fn from_account_info(a: &AccountInfo) -> Result<Self, ProgramError>
where {
        let data = &a.data.borrow_mut();

        let ua = Self::safe_deserialize(data).map_err(|_| MetadataError::DataTypeMismatch)?;

        // Check that this is a `token-metadata` owned account.
        assert_owned_by(a, &ID)?;

        Ok(ua)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, Copy, FromPrimitive)]
pub enum Key {
    Uninitialized,
    EditionV1,
    MasterEditionV1,
    ReservationListV1,
    MetadataV1,
    ReservationListV2,
    MasterEditionV2,
    EditionMarker,
    UseAuthorityRecord,
    CollectionAuthorityRecord,
    TokenOwnedEscrow,
    TokenRecord,
    MetadataDelegate,
}
