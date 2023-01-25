#![cfg_attr(not(feature = "std"), no_std)]

use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
    StorageLayout,
};

use ink_env::{
    AccountId,
    DefaultEnvironment,
    Environment,
};
type Balance = <DefaultEnvironment as Environment>::Balance;

pub struct AssetsExtension;

impl AssetsExtension {
    pub fn create(origin: Origin, id: u128, admin: AccountId, min_balance: Balance) -> Result<(), AssetsError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x48F60001)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<()>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, admin, min_balance))
    }

    pub fn transfer(origin: Origin, id: u128, target: AccountId, min_balance: Balance) -> Result<(), AssetsError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x48F60002)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<()>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, target, min_balance))
    }

    pub fn mint(origin: Origin, id: u128, beneficiary: AccountId, amount: Balance) -> Result<(), AssetsError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x48F60003)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<()>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, beneficiary, amount))
    }

    pub fn burn(origin: Origin, id: u128, who: AccountId, amount: Balance) -> Result<(), AssetsError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x48F60004)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<()>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, who, amount))
    }

    pub fn balance_of(id: u128, who: AccountId) -> Balance {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x48F60005)
            .input::<(u128, AccountId)>()
            .output::<Balance>()
            .ignore_error_code()
            .call(&(id, who))
    }

    pub fn total_supply(id: u128) -> Balance {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x48F60006)
            .input::<u128>()
            .output::<Balance>()
            .ignore_error_code()
            .call(&id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum Origin {
    Caller,
    Address,
}

impl Default for Origin {
    fn default() -> Self {
        Self::Address
    }
}

impl ink_storage::traits::SpreadAllocate for Origin {
    fn allocate_spread(_ptr: &mut ink_primitives::KeyPtr) -> Self {
        Self::Address
    }
}

#[derive(PartialEq, Eq, Copy, Clone, scale::Encode, scale::Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AssetsError {
    /// Success
    Success = 0,
    /// Error
    IsError = 1,
}

impl ink_env::chain_extension::FromStatusCode for AssetsError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::IsError),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for AssetsError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
