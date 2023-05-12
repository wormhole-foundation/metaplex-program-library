use crate::state::{
    collection::{
        Collection,
        CollectionDetails,
    },
    data::Data,
    metadata::{
        Metadata,
        ProgrammableConfig,
    },
    uses::Uses,
    Key,
    TokenStandard,
};
use borsh::{
    maybestd::io::Error as BorshError,
    BorshDeserialize,
};
use solana_program::pubkey::Pubkey;

// Custom deserialization function to handle NFTs with corrupted data.
// This function is used in a custom deserialization implementation for the
// `Metadata` struct, so should never have `msg` macros used in it as it may be used client side
// either in tests or client code.
//
// It does not check `Key` type or account length and should only be used through the custom functions
// `from_account_info` and `deserialize` implemented on the Metadata struct.
pub fn meta_deser_unchecked(buf: &mut &[u8]) -> Result<Metadata, BorshError> {
    // Metadata corruption shouldn't appear until after edition_nonce.
    let key: Key = BorshDeserialize::deserialize(buf)?;
    let update_authority: Pubkey = BorshDeserialize::deserialize(buf)?;
    let mint: Pubkey = BorshDeserialize::deserialize(buf)?;
    let data: Data = BorshDeserialize::deserialize(buf)?;
    let primary_sale_happened: bool = BorshDeserialize::deserialize(buf)?;
    let is_mutable: bool = BorshDeserialize::deserialize(buf)?;
    let edition_nonce: Option<u8> = BorshDeserialize::deserialize(buf)?;

    // V1.2
    let token_standard_res: Result<Option<TokenStandard>, BorshError> =
        BorshDeserialize::deserialize(buf);
    let collection_res: Result<Option<Collection>, BorshError> = BorshDeserialize::deserialize(buf);
    let uses_res: Result<Option<Uses>, BorshError> = BorshDeserialize::deserialize(buf);

    // V1.3
    let collection_details_res: Result<Option<CollectionDetails>, BorshError> =
        BorshDeserialize::deserialize(buf);

    // pNFT - Programmable Config
    let programmable_config_res: Result<Option<ProgrammableConfig>, BorshError> =
        BorshDeserialize::deserialize(buf);

    // We can have accidentally valid, but corrupted data, particularly on the Collection struct,
    // so to increase probability of catching errors. If any of these deserializations fail, set
    // all values to None.
    let (token_standard, collection, uses) = match (token_standard_res, collection_res, uses_res) {
        (Ok(token_standard_res), Ok(collection_res), Ok(uses_res)) => {
            (token_standard_res, collection_res, uses_res)
        }
        _ => (None, None, None),
    };

    // V1.3
    let collection_details = match collection_details_res {
        Ok(details) => details,
        Err(_) => None,
    };

    // Programmable Config
    let programmable_config = programmable_config_res.unwrap_or(None);

    let metadata = Metadata {
        key,
        update_authority,
        mint,
        data,
        primary_sale_happened,
        is_mutable,
        edition_nonce,
        token_standard,
        collection,
        uses,
        collection_details,
        programmable_config,
    };

    Ok(metadata)
}
