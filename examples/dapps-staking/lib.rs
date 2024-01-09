#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod staking_example {
    use dapps_staking_extension::*;

    #[ink(storage)]
    pub struct Staking {}

    impl Staking {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn read_current_era(&self) -> u32 {
            DappsStaking::read_current_era()
        }

        #[ink(message)]
        pub fn read_unbonding_period(&self) -> u32 {
            DappsStaking::read_unbonding_period()
        }

        #[ink(message)]
        pub fn read_era_reward(&self, era: u32) -> Balance {
            DappsStaking::read_era_reward(era)
        }

        #[ink(message)]
        pub fn read_era_staked(&self, era: u32) -> Balance {
            DappsStaking::read_era_staked(era)
        }

        #[ink(message)]
        pub fn read_staked_amount(&self, account: AccountId) -> Balance {
            DappsStaking::read_staked_amount(account)
        }

        #[ink(message)]
        pub fn read_staked_amount_on_contract(
            &self,
            staker: AccountId,
            contract: AccountId,
        ) -> Balance {
            DappsStaking::read_staked_amount_on_contract(staker, contract)
        }

        #[ink(message)]
        pub fn read_contract_stake(&self, account: AccountId) -> Balance {
            DappsStaking::read_contract_stake(account)
        }

        #[ink(message, payable)]
        pub fn bond_and_stake(&mut self) -> Result<(), DSError> {
            // make sure the caller is recorded as staker

            let contract = self.env().account_id();
            let value = self.env().transferred_value();
            DappsStaking::bond_and_stake(contract, value)
        }

        #[ink(message)]
        pub fn unbond_and_unstake(&mut self, value: Balance) -> Result<(), DSError> {
            // make sure caller is the staker

            let contract = self.env().account_id();
            DappsStaking::unbond_and_unstake(contract, value)
        }

        #[ink(message)]
        pub fn withdraw_unbonded(&mut self) -> Result<(), DSError> {
            // make sure caller has staked what is withdrawn

            DappsStaking::withdraw_unbonded()
        }

        #[ink(message)]
        pub fn claim_dapp(&mut self, account_id: AccountId, era: u32) -> Result<(), DSError> {
            DappsStaking::claim_dapp(account_id, era)
        }

        #[ink(message)]
        pub fn claim_staker(&mut self) -> Result<(), DSError> {
            let contract = self.env().account_id();
            DappsStaking::claim_staker(contract)
        }

        #[ink(message)]
        pub fn set_reward_destination(&mut self, destination: u8) -> Result<(), DSError> {
            DappsStaking::set_reward_destination(destination)
        }

        #[ink(message)]
        pub fn nomination_transfer(
            &mut self,
            origin_contract: AccountId,
            target_contract: AccountId,
            value: Balance,
        ) -> Result<(), DSError> {
            DappsStaking::nomination_transfer(origin_contract, target_contract, value)
        }
    }
}
