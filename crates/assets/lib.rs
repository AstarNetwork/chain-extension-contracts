#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;

use ink::env::{DefaultEnvironment, Environment};
use scale::{Decode, Encode};

type Balance = <DefaultEnvironment as Environment>::Balance;
type AccountId = <DefaultEnvironment as Environment>::AccountId;

pub struct AssetsExtension;

impl AssetsExtension {
    pub fn create(
        origin: Origin,
        id: u128,
        admin: AccountId,
        min_balance: Balance,
    ) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x20001)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, admin, min_balance))
    }

    pub fn transfer(
        origin: Origin,
        id: u128,
        target: AccountId,
        min_balance: Balance,
    ) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x20002)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, target, min_balance))
    }

    pub fn mint(
        origin: Origin,
        id: u128,
        beneficiary: AccountId,
        amount: Balance,
    ) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x20003)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, beneficiary, amount))
    }

    pub fn burn(
        origin: Origin,
        id: u128,
        who: AccountId,
        amount: Balance,
    ) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x20004)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, who, amount))
    }

    pub fn balance_of(id: u128, who: AccountId) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x20005)
            .input::<(u128, AccountId)>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&(id, who))
    }

    pub fn total_supply(id: u128) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x20006)
            .input::<u128>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn allowance(id: u128, owner: AccountId, delegate: AccountId) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x20007)
            .input::<(u128, AccountId, AccountId)>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&(id, owner, delegate))
    }

    pub fn approve_transfer(
        origin: Origin,
        id: u128,
        delegate: AccountId,
        amount: Balance,
    ) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x20008)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, delegate, amount))
    }

    pub fn cancel_approval(
        origin: Origin,
        id: u128,
        delegate: AccountId,
    ) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x20009)
            .input::<(Origin, u128, AccountId)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, delegate))
    }

    pub fn transfer_approved(
        origin: Origin,
        id: u128,
        owner: AccountId,
        destination: AccountId,
        amount: Balance,
    ) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x2000A)
            .input::<(Origin, u128, AccountId, AccountId, Balance)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, owner, destination, amount))
    }

    pub fn set_metadata(
        origin: Origin,
        id: u128,
        name: Vec<u8>,
        symbol: Vec<u8>,
        decimals: u8,
    ) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x2000B)
            .input::<(Origin, u128, Vec<u8>, Vec<u8>, u8)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, name, symbol, decimals))
    }

    pub fn metadata_name(id: u128) -> Vec<u8> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x2000C)
            .input::<u128>()
            .output::<Vec<u8>, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn metadata_symbol(id: u128) -> Vec<u8> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x2000D)
            .input::<u128>()
            .output::<Vec<u8>, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn metadata_decimals(id: u128) -> u8 {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x2000E)
            .input::<u128>()
            .output::<u8, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn transfer_ownership(
        origin: Origin,
        id: u128,
        owner: AccountId,
    ) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x2000F)
            .input::<(Origin, u128, AccountId)>()
            .output::<Result<(), AssetsError>, true>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, owner))
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AssetsError {
    /// Account balance must be greater than or equal to the transfer amount.
    BalanceLow = 1,
    /// The account to alter does not exist.
    NoAccount = 2,
    /// The signing account has no permission to do the operation.
    NoPermission = 3,
    /// The given asset ID is unknown.
    Unknown = 4,
    /// The origin account is frozen.
    Frozen = 5,
    /// The asset ID is already taken.
    InUse = 6,
    /// Invalid witness data given.
    BadWitness = 7,
    /// Minimum balance should be non-zero.
    MinBalanceZero = 8,
    /// Unable to increment the consumer reference counters on the account. Either no provider
    /// reference exists to allow a non-zero balance of a non-self-sufficient asset, or the
    /// maximum number of consumers has been reached.
    NoProvider = 9,
    /// Invalid metadata given.
    BadMetadata = 10,
    /// No approval exists that would allow the transfer.
    Unapproved = 11,
    /// The source account would not survive the transfer and it needs to stay alive.
    WouldDie = 12,
    /// The asset-account already exists.
    AlreadyExists = 13,
    /// The asset-account doesn't have an associated deposit.
    NoDeposit = 14,
    /// The operation would result in funds being burned.
    WouldBurn = 15,
    /// The asset is a live asset and is actively being used. Usually emit for operations such
    /// as `start_destroy` which require the asset to be in a destroying state.
    LiveAsset = 16,
    /// The asset is not live, and likely being destroyed.
    AssetNotLive = 17,
    /// The asset status is not the expected status.
    IncorrectStatus = 18,
    /// The asset should be frozen before the given operation.
    NotFrozen = 19,
    /// Origin Caller is not supported
    OriginCannotBeCaller = 98,
    /// Unknown error
    RuntimeError = 99,
    /// Unknow status code
    UnknownStatusCode,
    /// Encountered unexpected invalid SCALE encoding
    InvalidScaleEncoding,
}

impl ink::env::chain_extension::FromStatusCode for AssetsError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::BalanceLow),
            2 => Err(Self::NoAccount),
            3 => Err(Self::NoPermission),
            4 => Err(Self::Unknown),
            5 => Err(Self::Frozen),
            6 => Err(Self::InUse),
            7 => Err(Self::BadWitness),
            8 => Err(Self::MinBalanceZero),
            9 => Err(Self::NoProvider),
            10 => Err(Self::BadMetadata),
            11 => Err(Self::Unapproved),
            12 => Err(Self::WouldDie),
            13 => Err(Self::AlreadyExists),
            14 => Err(Self::NoDeposit),
            15 => Err(Self::WouldBurn),
            16 => Err(Self::LiveAsset),
            17 => Err(Self::AssetNotLive),
            18 => Err(Self::IncorrectStatus),
            19 => Err(Self::NotFrozen),
            98 => Err(Self::OriginCannotBeCaller),
            99 => Err(Self::RuntimeError),
            _ => Err(Self::UnknownStatusCode),
        }
    }
}
impl From<scale::Error> for AssetsError {
    fn from(_: scale::Error) -> Self {
        AssetsError::InvalidScaleEncoding
    }
}

#[derive(Clone, Copy, Decode, Encode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub enum Origin {
    Caller,
    Address,
}

impl Default for Origin {
    fn default() -> Self {
        Self::Address
    }
}
