#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;

use assets_chain_extension_types::{
    Origin,
    Outcome as AssetsError,
};
use ink::env::{
    DefaultEnvironment,
    Environment,
};

type Balance = <DefaultEnvironment as Environment>::Balance;
type AccountId = <DefaultEnvironment as Environment>::AccountId;

pub struct AssetsExtension;

impl AssetsExtension {
    pub fn create(origin: Origin, id: u128, admin: AccountId, min_balance: Balance) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F60001)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<(), false>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, admin, min_balance))
    }

    pub fn transfer(origin: Origin, id: u128, target: AccountId, min_balance: Balance) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F60002)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<(), false>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, target, min_balance))
    }

    pub fn mint(origin: Origin, id: u128, beneficiary: AccountId, amount: Balance) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F60003)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<(), false>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, beneficiary, amount))
    }

    pub fn burn(origin: Origin, id: u128, who: AccountId, amount: Balance) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F60004)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<(), false>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, who, amount))
    }

    pub fn balance_of(id: u128, who: AccountId) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F60005)
            .input::<(u128, AccountId)>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&(id, who))
    }

    pub fn total_supply(id: u128) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F60006)
            .input::<u128>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn allowance(id: u128, owner: AccountId, delegate: AccountId) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F60007)
            .input::<(u128, AccountId, AccountId)>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&(id, owner, delegate))
    }

    pub fn approve_transfer(origin: Origin, id: u128, delegate: AccountId, amount: Balance) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F60008)
            .input::<(Origin, u128, AccountId, Balance)>()
            .output::<(), false>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, delegate, amount))
    }

    pub fn cancel_approval(origin: Origin, id: u128, delegate: AccountId) -> Result<(), AssetsError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F60009)
            .input::<(Origin, u128, AccountId)>()
            .output::<(), false>()
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
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F6000A)
            .input::<(Origin, u128, AccountId, AccountId, Balance)>()
            .output::<(), false>()
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
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F6000B)
            .input::<(Origin, u128, Vec<u8>, Vec<u8>, u8)>()
            .output::<(), false>()
            .handle_error_code::<AssetsError>()
            .call(&(origin, id, name, symbol, decimals))
    }

    pub fn metadata_name(id: u128) -> Vec<u8> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F6000C)
            .input::<u128>()
            .output::<Vec<u8>, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn metadata_symbol(id: u128) -> Vec<u8> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F6000D)
            .input::<u128>()
            .output::<Vec<u8>, false>()
            .ignore_error_code()
            .call(&id)
    }

    pub fn metadata_decimals(id: u128) -> u8 {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0x48F6000E)
            .input::<u128>()
            .output::<u8, false>()
            .ignore_error_code()
            .call(&id)
    }
}
