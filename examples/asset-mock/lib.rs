#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use ink_lang as ink;

#[ink::contract]
pub mod contract {
    use assets_extension::*;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct Mock {
        asset_id: u128,
        origin: Origin,
    }

    impl Mock {
        #[ink(constructor)]
        pub fn new(asset_id: u128) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.asset_id = asset_id;
            })
        }

        #[ink(message)]
        pub fn asset_id(&self) -> u128 {
            self.asset_id
        }

        #[ink(message, payable)]
        pub fn create(&mut self, asset_id: u128, min_balance: Balance) -> Result<(), AssetsError> {
            let caller = self.env().caller();
            AssetsExtension::create(Origin::Caller, asset_id, caller, min_balance)?;
            Ok(())
        }

        #[ink(message, payable)]
        pub fn mint(&mut self, asset_id: u128, amount: Balance) -> Result<(), AssetsError> {
            let caller = self.env().caller();
            AssetsExtension::mint(Origin::Caller, asset_id, caller, amount)?;
            Ok(())
        }

        #[ink(message, payable)]
        pub fn burn(&mut self, asset_id: u128, amount: Balance) -> Result<(), AssetsError> {
            let caller = self.env().caller();
            AssetsExtension::burn(Origin::Caller, asset_id, caller, amount)?;
            Ok(())
        }
    }
}
