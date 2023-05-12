use borsh::{
    BorshDeserialize,
    BorshSerialize,
};
use num_derive::FromPrimitive;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, FromPrimitive)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct Uses {
    // 17 bytes + Option byte
    pub use_method: UseMethod, //1
    pub remaining: u64,        //8
    pub total: u64,            //8
}
