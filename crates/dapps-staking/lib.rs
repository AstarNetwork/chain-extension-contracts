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
        ::ink_env::chain_extension::ChainExtensionMethod::build(3401u32)
            .input::<()>()
            .output::<u32>()
            .ignore_error_code()
            .call(&())
    }

    /// Fetch unbonding period
    pub fn read_unbonding_period() -> u32 {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3402u32)
            .input::<()>()
            .output::<u32>()
            .ignore_error_code()
            .call(&())
    }

    /// Fetch reward from EraRewardsAndStakes storage map
    /// Returns '0' if no rewards
    pub fn read_era_reward(era: u32) -> Balance {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3403u32)
            .input::<u32>()
            .output::<Balance>()
            .ignore_error_code()
            .call(&era)
    }

    /// Fetch total staked amount from EraRewardsAndStakes storage map
    pub fn read_era_staked(era: u32) -> Balance {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3404u32)
            .input::<u32>()
            .output::<Balance>()
            .ignore_error_code()
            .call(&era)
    }

    /// Fetch Ledger storage map for an account
    pub fn read_staked_amount(account: AccountId) -> Balance {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3405u32)
            .input::<AccountId>()
            .output::<Balance>()
            .ignore_error_code()
            .call(&account)
    }

    /// Read GeneralStakerInfo for account/contract
    pub fn read_staked_amount_on_contract(staker: AccountId, contract: AccountId) -> Balance {
        let input = DappsStakingAccountInput { staker, contract };
        ::ink_env::chain_extension::ChainExtensionMethod::build(3406u32)
            .input::<DappsStakingAccountInput>()
            .output::<Balance>()
            .ignore_error_code()
            .call(&input)
    }

    /// Read the amount staked on contract in the given era
    pub fn read_contract_stake(contract: AccountId) -> Balance {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3407u32)
            .input::<AccountId>()
            .output::<Balance>()
            .ignore_error_code()
            .call(&contract)
    }

    /// Register contract with the dapp-staking pallet
    pub fn register(contract_account_id: AccountId) -> Result<(), DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3408u32)
            .input::<AccountId>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&contract_account_id)?
    }

    /// Lock up and stake balance of the origin account.
    pub fn bond_and_stake(contract: AccountId, value: Balance) -> Result<(), DSError> {
        let input = DappsStakingValueInput { contract, value };
        ::ink_env::chain_extension::ChainExtensionMethod::build(3409u32)
            .input::<DappsStakingValueInput>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&input)?
    }

    /// Start unbonding process and unstake balance from the contract.
    pub fn unbond_and_unstake(contract: AccountId, value: Balance) -> Result<(), DSError> {
        let input = DappsStakingValueInput { contract, value };
        ::ink_env::chain_extension::ChainExtensionMethod::build(3410u32)
            .input::<DappsStakingValueInput>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&input)?
    }

    pub fn withdraw_unbonded() -> Result<(), DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3411u32)
            .input::<()>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&())?
    }

    /// Claim rewards for the contract in the dapps-staking pallet
    pub fn claim_staker(account_id: AccountId) -> Result<(), DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3412u32)
            .input::<AccountId>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&account_id)?
    }

    /// Claim rewards for the contract in the dapps-staking pallet
    pub fn claim_dapp(contract: AccountId, era: u32) -> Result<(), DSError> {
        let input = DappsStakingEraInput { contract, era };
        ::ink_env::chain_extension::ChainExtensionMethod::build(3413u32)
            .input::<DappsStakingEraInput>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&input)?
    }

    /// Set claim reward destination for the caller
    pub fn set_reward_destination(destination: u8) -> Result<(), DSError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(3414u32)
            .input::<u8>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&destination)?
    }

    /// Claim rewards for the contract in the dapps-staking pallet
    pub fn nomination_transfer(
        origin_contract: AccountId,
        target_contract: AccountId,
        value: Balance,
    ) -> Result<(), DSError> {
        let input = DappsStakingNominationInput {
            origin_contract,
            target_contract,
            value,
        };
        ::ink_env::chain_extension::ChainExtensionMethod::build(3415u32)
            .input::<DappsStakingNominationInput>()
            .output::<Result<(), DSError>>()
            .handle_error_code::<DSError>()
            .call(&input)?
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
pub struct DappsStakingEraInput {
    contract: AccountId,
    era: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
pub struct DappsStakingValueInput {
    contract: AccountId,
    value: Balance,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
pub struct DappsStakingAccountInput {
    contract: AccountId,
    staker: AccountId,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode)]
pub struct DappsStakingNominationInput {
    pub origin_contract: AccountId,
    pub target_contract: AccountId,
    pub value: Balance,
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

#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
pub enum DSError {
    /// Disabled
    Disabled,
    /// No change in maintenance mode
    NoMaintenanceModeChange,
    /// Upgrade is too heavy, reduce the weight parameter.
    UpgradeTooHeavy,
    /// Can not stake with zero value.
    StakingWithNoValue,
    /// Can not stake with value less than minimum staking value
    InsufficientValue,
    /// Number of stakers per contract exceeded.
    MaxNumberOfStakersExceeded,
    /// Targets must be operated contracts
    NotOperatedContract,
    /// Contract isn't staked.
    NotStakedContract,
    /// Contract isn't unregistered.
    NotUnregisteredContract,
    /// Unclaimed rewards should be claimed before withdrawing stake.
    UnclaimedRewardsRemaining,
    /// Unstaking a contract with zero value
    UnstakingWithNoValue,
    /// There are no previously unbonded funds that can be unstaked and withdrawn.
    NothingToWithdraw,
    /// The contract is already registered by other account
    AlreadyRegisteredContract,
    /// User attempts to register with address which is not contract
    ContractIsNotValid,
    /// This account was already used to register contract
    AlreadyUsedDeveloperAccount,
    /// Smart contract not owned by the account id.
    NotOwnedContract,
    /// Report issue on github if this is ever emitted
    UnknownEraReward,
    /// Report issue on github if this is ever emitted
    UnexpectedStakeInfoEra,
    /// Contract has too many unlocking chunks. Withdraw the existing chunks if possible
    /// or wait for current chunks to complete unlocking process to withdraw them.
    TooManyUnlockingChunks,
    /// Contract already claimed in this era and reward is distributed
    AlreadyClaimedInThisEra,
    /// Era parameter is out of bounds
    EraOutOfBounds,
    /// Too many active `EraStake` values for (staker, contract) pairing.
    /// Claim existing rewards to fix this problem.
    TooManyEraStakeValues,
    /// To register a contract, pre-approval is needed for this address
    RequiredContractPreApproval,
    /// Developer's account is already part of pre-approved list
    AlreadyPreApprovedDeveloper,
    /// Account is not actively staking
    NotActiveStaker,
    /// Transfering nomination to the same contract
    NominationTransferToSameContract,
    /// Unexpected reward destination value
    RewardDestinationValueOutOfBounds,
}

impl ink_env::chain_extension::FromStatusCode for DSError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            _ => panic!("encountered unknown status code"),
        }
    }
}

impl From<scale::Error> for DSError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}
