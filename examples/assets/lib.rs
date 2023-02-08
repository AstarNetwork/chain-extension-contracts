#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[ink::contract]
pub mod contract {
    use assets_chain_extension_types::{
        Origin,
        Outcome as AssetsError,
    };
    use assets_extension::*;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Mock {}

    impl Mock {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message, payable)]
        pub fn create(&mut self, asset_id: u128, min_balance: Balance) -> Result<(), AssetsError> {
            let caller = self.env().caller();
            AssetsExtension::create(Origin::Caller, asset_id, caller, min_balance)?;
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, asset_id: u128, beneficiary: AccountId, amount: Balance) -> Result<(), AssetsError> {
            AssetsExtension::mint(Origin::Caller, asset_id, beneficiary, amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, asset_id: u128, who: AccountId, amount: Balance) -> Result<(), AssetsError> {
            AssetsExtension::burn(Origin::Caller, asset_id, who, amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self, asset_id: u128, target: AccountId, amount: Balance) -> Result<(), AssetsError> {
            AssetsExtension::transfer(Origin::Caller, asset_id, target, amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, asset_id: u128, who: AccountId) -> Balance {
            AssetsExtension::balance_of(asset_id, who)
        }

        #[ink(message)]
        pub fn total_supply(&self, asset_id: u128) -> Balance {
            AssetsExtension::total_supply(asset_id)
        }

        #[ink(message)]
        pub fn allowance(&self, asset_id: u128, owner: AccountId, delegate: AccountId) -> Balance {
            AssetsExtension::allowance(asset_id, owner, delegate)
        }

        #[ink(message)]
        pub fn approve_transfer(
            &mut self,
            asset_id: u128,
            delegate: AccountId,
            amount: Balance,
        ) -> Result<(), AssetsError> {
            AssetsExtension::approve_transfer(Origin::Caller, asset_id, delegate, amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn cancel_approval(&mut self, asset_id: u128, delegate: AccountId) -> Result<(), AssetsError> {
            AssetsExtension::cancel_approval(Origin::Caller, asset_id, delegate)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_approved(
            &mut self,
            asset_id: u128,
            owner: AccountId,
            destination: AccountId,
            amount: Balance,
        ) -> Result<(), AssetsError> {
            AssetsExtension::transfer_approved(Origin::Caller, asset_id, owner, destination, amount)?;
            Ok(())
        }

        #[ink(message)]
        pub fn set_metadata(
            &mut self,
            asset_id: u128,
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> Result<(), AssetsError> {
            AssetsExtension::set_metadata(Origin::Caller, asset_id, name, symbol, decimals)?;
            Ok(())
        }

        #[ink(message)]
        pub fn metadata_name(&self, asset_id: u128) -> Vec<u8> {
            AssetsExtension::metadata_name(asset_id)
        }

        #[ink(message)]
        pub fn metadata_symbol(&self, asset_id: u128) -> Vec<u8> {
            AssetsExtension::metadata_symbol(asset_id)
        }

        #[ink(message)]
        pub fn metadata_decimals(&self, asset_id: u128) -> u8 {
            AssetsExtension::metadata_decimals(asset_id)
        }
    }
}
