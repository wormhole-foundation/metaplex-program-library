pub mod metadata;

pub use metadata::*;

use crate::instruction::metadata::{
    CreateMetadataAccountArgsV3,
    UpdateMetadataAccountArgsV2,
};
use borsh::{
    BorshDeserialize,
    BorshSerialize,
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
/// Instructions supported by the Metadata program.
pub enum MetadataInstruction {
    CreateMetadataAccount,
    UpdateMetadataAccount,
    DeprecatedCreateMasterEdition,
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
    UpdatePrimarySaleHappenedViaToken,
    DeprecatedSetReservationList,
    DeprecatedCreateReservationList,
    SignMetadata,
    DeprecatedMintPrintingTokensViaToken,
    DeprecatedMintPrintingTokens,
    CreateMasterEdition,
    MintNewEditionFromMasterEditionViaToken,
    ConvertMasterEditionV1ToV2,
    MintNewEditionFromMasterEditionViaVaultProxy,
    PuffMetadata,
    /// Update Metadata object. Token Bridge uses this instruction to update metadata.
    UpdateMetadataAccountV2(UpdateMetadataAccountArgsV2),
    CreateMetadataAccountV2,
    CreateMasterEditionV3,
    VerifyCollection,
    Utilize,
    ApproveUseAuthority,
    RevokeUseAuthority,
    UnverifyCollection,
    ApproveCollectionAuthority,
    RevokeCollectionAuthority,
    SetAndVerifyCollection,
    FreezeDelegatedAccount,
    ThawDelegatedAccount,
    RemoveCreatorVerification,
    BurnNft,
    VerifySizedCollectionItem,
    UnverifySizedCollectionItem,
    SetAndVerifySizedCollectionItem,
    /// Create Metadata object. Token Bridge and NFT Bridge use this instruction to create metadata.
    CreateMetadataAccountV3(CreateMetadataAccountArgsV3),
    SetCollectionSize,
    SetTokenStandard,
    BubblegumSetCollectionSize,
    BurnEditionNft,
    CreateEscrowAccount,
    CloseEscrowAccount,
    TransferOutOfEscrow,

    //---- New API
    Burn,
    Create,
    Mint,
    Delegate,
    Revoke,
    Lock,
    Unlock,
    Migrate,
    Transfer,
    Update,
    Use,
    Verify,
    Unverify,
}
