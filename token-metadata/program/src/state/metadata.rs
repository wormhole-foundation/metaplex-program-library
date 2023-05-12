use crate::{
    state::{
        collection::{
            Collection,
            CollectionDetails,
        },
        creator::{
            MAX_CREATOR_LEN,
            MAX_CREATOR_LIMIT,
        },
        data::Data,
        uses::Uses,
        Key,
        TokenMetadataAccount,
        TokenStandard,
    },
    utils::metadata::meta_deser_unchecked,
};
use borsh::{
    maybestd::io::Error as BorshError,
    BorshDeserialize,
    BorshSerialize,
};
use solana_program::pubkey::Pubkey;

pub const MAX_NAME_LENGTH: usize = 32;

pub const MAX_SYMBOL_LENGTH: usize = 10;

pub const MAX_URI_LENGTH: usize = 200;

pub const MAX_METADATA_LEN: usize = 1 // key
+ 32             // update auth pubkey
+ 32             // mint pubkey
+ MAX_DATA_SIZE
+ 1              // primary sale
+ 1              // mutable
+ 9              // nonce (pretty sure this only needs to be 2)
+ 2              // token standard
+ 34             // collection
+ 18             // uses
+ 10             // collection details
+ 33             // programmable config
+ 75; // Padding

pub const MAX_DATA_SIZE: usize = 4
    + MAX_NAME_LENGTH
    + 4
    + MAX_SYMBOL_LENGTH
    + 4
    + MAX_URI_LENGTH
    + 2
    + 1
    + 4
    + MAX_CREATOR_LIMIT * MAX_CREATOR_LEN;

#[repr(C)]
#[derive(Clone, BorshSerialize, Debug, PartialEq, Eq)]
pub struct Metadata {
    /// Account discriminator.
    pub key: Key,
    /// Address of the update authority.
    pub update_authority: Pubkey,
    /// Address of the mint.
    pub mint: Pubkey,
    /// Asset data.
    pub data: Data,
    // Immutable, once flipped, all sales of this metadata are considered secondary.
    pub primary_sale_happened: bool,
    // Whether or not the data struct is mutable, default is not
    pub is_mutable: bool,
    /// nonce for easy calculation of editions, if present
    pub edition_nonce: Option<u8>,
    /// Since we cannot easily change Metadata, we add the new DataV2 fields here at the end.
    pub token_standard: Option<TokenStandard>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
    /// Collection Details
    pub collection_details: Option<CollectionDetails>,
    /// Programmable Config
    pub programmable_config: Option<ProgrammableConfig>,
}

/// Configuration for programmable assets.
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum ProgrammableConfig {
    V1 { rule_set: Option<Pubkey> },
}

impl TokenMetadataAccount for Metadata {
    fn key() -> Key {
        Key::MetadataV1
    }

    fn size() -> usize {
        MAX_METADATA_LEN
    }
}

// We have a custom implementation of BorshDeserialize for Metadata because of corrupted metadata issues
// caused by resizing of the Creators array. We use a custom `meta_deser_unchecked` function
// that has fallback values for corrupted fields.
impl borsh::de::BorshDeserialize for Metadata {
    fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, BorshError> {
        let md = meta_deser_unchecked(buf)?;
        Ok(md)
    }
}
