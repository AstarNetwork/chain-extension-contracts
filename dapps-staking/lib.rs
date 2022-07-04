#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{
    AccountId,
    DefaultEnvironment,
    Environment,
};
use scale::{
    Decode,
    Encode,
    HasCompact,
};

type Balance = <DefaultEnvironment as Environment>::Balance;

pub struct DappsStaking;

impl DappsStaking {
    /// Fetch current era from CurrentEra storage map
    pub fn read_current_era() -> u32 {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<()>()
            .output::<u32>()
            .ignore_error_code()
            .call(&())
    }

    /// Fetch unbonding period
    pub fn read_unbonding_period() -> u32 {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<()>()
            .output::<u32>()
            .ignore_error_code()
            .call(&())
    }

    /// Fetch reward from EraRewardsAndStakes storage map
    /// Returns '0' if no rewards
    pub fn read_era_reward(era: u32) -> Result<Balance, DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<u32>()
            .output::<Result<Balance, DSError>>()
            .handle_error_code::<DSError>()
            .call(&era)?
    }

    /// Fetch total staked amount from EraRewardsAndStakes storage map
    pub fn read_era_staked(era: u32) -> Result<Balance, DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<u32>()
            .output::<Result<Balance, DSError>>()
            .handle_error_code::<DSError>()
            .call(&era)?
    }

    /// Fetch Ledger storage map for an account
    pub fn read_staked_amount(account: AccountId) -> Result<Balance, DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<AccountId>()
            .output::<Result<Balance, DSError>>()
            .handle_error_code::<DSError>()
            .call(&account)?
    }

    /// Read GeneralStakerInfo for account/contract
    pub fn read_staked_amount_on_contract(contract_account_id: AccountId) -> Result<Balance, DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<AccountId>()
            .output::<Result<Balance, DSError>>()
            .handle_error_code::<DSError>()
            .call(&contract_account_id)?
    }

    /// Read the amount staked on contract in the given era
    pub fn read_contract_stake(contract_account_id: AccountId) -> Result<Balance, DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<AccountId>()
            .output::<Result<Balance, DSError>>()
            .handle_error_code::<DSError>()
            .call(&contract_account_id)?
    }

    /// Register contract with the dapp-staking pallet
    pub fn register(contract_account_id: AccountId) -> Result<(), DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<AccountId>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&contract_account_id)?
    }

    /// Lock up and stake balance of the origin account.
    pub fn bond_and_stake(account_id: AccountId, value: Balance) -> Result<(), DSError> {
        let input = DappsStakingInput { account_id, value };
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<DappsStakingInput>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&input)?
    }

    /// Start unbonding process and unstake balance from the contract.
    pub fn unbond_and_stake(account_id: AccountId, value: Balance) -> Result<(), DSError> {
        let input = DappsStakingInput { account_id, value };
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<DappsStakingInput>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&input)?
    }

    pub fn withdraw_unbonded() -> Result<(), DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<()>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&())?
    }

    /// Claim rewards for the contract in the dapps-staking pallet
    pub fn claim_dapp(account_id: AccountId, value: Balance) -> Result<(), DSError> {
        let input = DappsStakingInput { account_id, value };
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<DappsStakingInput>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&input)?
    }

    /// Claim rewards for the contract in the dapps-staking pallet
    pub fn claim_staker(account_id: AccountId) -> Result<(), DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<AccountId>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&account_id)?
    }

    /// Set claim reward destination for the caller
    pub fn set_reward_destination(destination: u8) -> Result<(), DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<u8>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&destination)?
    }

    /// Withdraw staked funds from the unregistered contract
    pub fn withdraw_from_unregistered(account_id: AccountId) -> Result<(), DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<AccountId>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&account_id)?
    }

    /// Claim rewards for the contract in the dapps-staking pallet
    pub fn nomination_transfer(account_id: AccountId, value: Balance) -> Result<(), DSError> {
        let input = DappsStakingInput { account_id, value };
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<DappsStakingInput>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&input)?
    }

    /// Calls general_era_info() in the pallet-dapps-staking
    pub fn read_era_info(era: u32) -> Result<EraInfo<Balance>, DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(00u32)
            .input::<u32>()
            .output::<Result<EraInfo<Balance>, DSError>>()
            .handle_error_code::<DSError>()
            .call(&era)?
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
pub struct DappsStakingInput {
    account_id: AccountId,
    value: Balance,
}

/// A record of rewards allocated for stakers and dapps
#[derive(PartialEq, Debug, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RewardInfo<Balance: HasCompact> {
    /// Total amount of rewards for stakers in an era
    #[codec(compact)]
    pub stakers: Balance,
    /// Total amount of rewards for dapps in an era
    #[codec(compact)]
    pub dapps: Balance,
}

/// A record for total rewards and total amount staked for an era
#[derive(PartialEq, Debug, Eq, Clone, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct EraInfo<Balance: HasCompact> {
    /// Total amount of earned rewards for an era
    pub rewards: RewardInfo<Balance>,
    /// Total staked amount in an era
    #[codec(compact)]
    pub staked: Balance,
    /// Total locked amount in an era
    #[codec(compact)]
    pub locked: Balance,
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum DSError {
    Failed,
}

impl ink_env::chain_extension::FromStatusCode for DSError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Failed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for DSError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
