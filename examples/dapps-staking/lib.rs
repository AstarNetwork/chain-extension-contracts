#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod staking_example {
    use dapps_staking_extension::*;

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum StakingError {
        DsError(DSError),
    }

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

        // #[ink(message)]
        // pub fn read_era_reward(&self, era: u32) -> Result<Balance, StakingError> {
        //     DappsStaking::read_era_reward(era).map_err(|e| return StakingError::DsError(e))
        // }

        #[ink(message)]
        pub fn read_era_staked(&self, era: u32) -> Balance {
            DappsStaking::read_era_staked(era)
        }

        // #[ink(message)]
        // pub fn read_era_staked(&self, era: u32) -> Result<Balance, StakingError> {
        //     DappsStaking::read_era_staked(era).map_err(|e| return StakingError::DsError(e))
        // }

        #[ink(message)]
        pub fn read_staked_amount(&self, account: AccountId) -> Balance {
            DappsStaking::read_staked_amount(account)
        }

        // #[ink(message)]
        // pub fn read_staked_amount(&self, account: AccountId) -> Result<Balance, StakingError> {
        //     DappsStaking::read_staked_amount(account).map_err(|e| return StakingError::DsError(e))
        // }

        #[ink(message)]
        pub fn read_staked_amount_on_contract(&self, account: AccountId) -> Balance {
            DappsStaking::read_staked_amount_on_contract(account)
        }

        // #[ink(message)]
        // pub fn read_staked_amount_on_contract(&self, account: AccountId) -> Result<Balance, StakingError> {
        //     DappsStaking::read_staked_amount_on_contract(account).map_err(|e| return StakingError::DsError(e))
        // }

        #[ink(message)]
        pub fn read_contract_stake(&self, account: AccountId) -> Result<Balance, StakingError> {
            DappsStaking::read_contract_stake(account).map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn register(&mut self, contract: AccountId) -> Result<(), StakingError> {
            DappsStaking::register(contract).map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn bond_and_stake(&mut self, contract: AccountId, value: Balance) -> Result<(), StakingError> {
            DappsStaking::bond_and_stake(contract, value).map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn unbond_and_stake(&mut self, contract: AccountId, value: Balance) -> Result<(), StakingError> {
            DappsStaking::unbond_and_stake(contract, value).map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn withdraw_unbonded(&mut self) -> Result<(), StakingError> {
            DappsStaking::withdraw_unbonded().map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn claim_dapp(&mut self, account_id: AccountId, value: Balance) -> Result<(), StakingError> {
            DappsStaking::claim_dapp(account_id, value).map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn claim_staker(&mut self, account_id: AccountId) -> Result<(), StakingError> {
            DappsStaking::claim_staker(account_id).map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn set_reward_destination(&mut self, destination: u8) -> Result<(), StakingError> {
            DappsStaking::set_reward_destination(destination).map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn withdraw_from_unregistered(&mut self, account_id: AccountId) -> Result<(), StakingError> {
            DappsStaking::withdraw_from_unregistered(account_id).map_err(|e| return StakingError::DsError(e))
        }

        #[ink(message)]
        pub fn nomination_transfer(&mut self, account_id: AccountId, value: Balance) -> Result<(), StakingError> {
            DappsStaking::nomination_transfer(account_id, value).map_err(|e| return StakingError::DsError(e))
        }
    }
}
