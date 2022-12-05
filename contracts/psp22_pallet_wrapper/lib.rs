#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod psp22_pallet_wrapper {
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };
    pub use pallet_assets_chain_extension::{
        ink::*,
        traits::*,
    };

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl psp22::Internal for PSP22WrapperContract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, amount: Balance) {
            self.env().emit_event(Transfer {
                from,
                to,
                value: amount,
            });
        }

        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value: amount,
            });
        }
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PSP22WrapperContract {
        #[storage_field]
        psp22: psp22::Data,
        asset_id: u128,
        origin: Origin,
        pallet_assets: AssetsExtension,
    }

    impl PSP22 for PSP22WrapperContract {}

    impl PSP22WrapperContract {
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

        #[ink(message)]
        pub fn deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            let contract = self.env().account_id();
            self.pallet_assets
                .transfer(Origin::Caller, self.asset_id, contract, amount)
                .map_err(|_| PSP22Error::Custom("transfer failed".into()))?;
            self._mint_to(caller, amount)
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            self._burn_from(caller, amount)?;
            self.pallet_assets
                .transfer(Origin::Address, self.asset_id, caller, amount)
                .map_err(|_| PSP22Error::Custom("transfer failed".into()))
        }
    }
}
