#![cfg_attr(not(feature = "std"), no_std)]

use ink::env::{
    DefaultEnvironment,
    Environment,
};

use scale::{
    Decode,
    Encode,
    HasCompact,
};

type Balance = <DefaultEnvironment as Environment>::Balance;
type AccountId = <DefaultEnvironment as Environment>::AccountId;

pub struct DappsStaking;

impl DappsStaking {
    /// Fetch current era from CurrentEra storage map
    pub fn read_current_era() -> u32 {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0001u32)
            .input::<()>()
            .output::<u32, false>()
            .ignore_error_code()
            .call(&())
    }

    /// Fetch unbonding period
    pub fn read_unbonding_period() -> u32 {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0002u32)
            .input::<()>()
            .output::<u32, false>()
            .ignore_error_code()
            .call(&())
    }

    /// Fetch reward from EraRewardsAndStakes storage map
    /// Returns '0' if no rewards
    pub fn read_era_reward(era: u32) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0003u32)
            .input::<u32>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&era)
    }

    /// Fetch total staked amount from EraRewardsAndStakes storage map
    pub fn read_era_staked(era: u32) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0004u32)
            .input::<u32>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&era)
    }

    /// Fetch Ledger storage map for an account
    pub fn read_staked_amount(account: AccountId) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0005u32)
            .input::<AccountId>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&account)
    }

    /// Read GeneralStakerInfo for account/contract
    pub fn read_staked_amount_on_contract(staker: AccountId, contract: AccountId) -> Balance {
        let input = DappsStakingAccountInput { staker, contract };
        ::ink::env::chain_extension::ChainExtensionMethod::build(0006u32)
            .input::<DappsStakingAccountInput>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&input)
    }

    /// Read the amount staked on contract in the given era
    pub fn read_contract_stake(contract: AccountId) -> Balance {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0007u32)
            .input::<AccountId>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&contract)
    }

    /// Lock up and stake balance of the origin account.
    pub fn bond_and_stake(contract: AccountId, value: Balance) -> Result<(), DSError> {
        let input = DappsStakingValueInput { contract, value };
        ::ink::env::chain_extension::ChainExtensionMethod::build(0008u32)
            .input::<DappsStakingValueInput>()
            .output::<(), false>()
            .handle_error_code::<DSError>()
            .call(&input)
    }

    /// Start unbonding process and unstake balance from the contract.
    pub fn unbond_and_unstake(contract: AccountId, value: Balance) -> Result<(), DSError> {
        let input = DappsStakingValueInput { contract, value };
        ::ink::env::chain_extension::ChainExtensionMethod::build(0009u32)
            .input::<DappsStakingValueInput>()
            .output::<(), false>()
            .handle_error_code::<DSError>()
            .call(&input)
    }

    pub fn withdraw_unbonded() -> Result<(), DSError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0010u32)
            .input::<()>()
            .output::<(), false>()
            .handle_error_code::<DSError>()
            .call(&())
    }

    /// Claim rewards for the contract in the dapps-staking pallet
    pub fn claim_staker(account_id: AccountId) -> Result<(), DSError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0011u32)
            .input::<AccountId>()
            .output::<(), false>()
            .handle_error_code::<DSError>()
            .call(&account_id)
    }

    /// Claim rewards for the contract in the dapps-staking pallet
    pub fn claim_dapp(contract: AccountId, era: u32) -> Result<(), DSError> {
        let input = DappsStakingEraInput { contract, era };
        ::ink::env::chain_extension::ChainExtensionMethod::build(0012u32)
            .input::<DappsStakingEraInput>()
            .output::<(), false>()
            .handle_error_code::<DSError>()
            .call(&input)
    }

    /// Set claim reward destination for the caller
    pub fn set_reward_destination(destination: u8) -> Result<(), DSError> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0013u32)
            .input::<u8>()
            .output::<(), false>()
            .handle_error_code::<DSError>()
            .call(&destination)
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
        ::ink::env::chain_extension::ChainExtensionMethod::build(0014u32)
            .input::<DappsStakingNominationInput>()
            .output::<(), false>()
            .handle_error_code::<DSError>()
            .call(&input)
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
    Disabled = 1,
    /// No change in maintenance mode
    NoMaintenanceModeChange = 2,
    /// Upgrade is too heavy, reduce the weight parameter.
    UpgradeTooHeavy = 3,
    /// Can not stake with zero value.
    StakingWithNoValue = 4,
    /// Can not stake with value less than minimum staking value
    InsufficientValue = 5,
    /// Number of stakers per contract exceeded.
    MaxNumberOfStakersExceeded = 6,
    /// Targets must be operated contracts
    NotOperatedContract = 7,
    /// Contract isn't staked.
    NotStakedContract = 8,
    /// Contract isn't unregistered.
    NotUnregisteredContract = 9,
    /// Unclaimed rewards should be claimed before withdrawing stake.
    UnclaimedRewardsRemaining = 10,
    /// Unstaking a contract with zero value
    UnstakingWithNoValue = 11,
    /// There are no previously unbonded funds that can be unstaked and withdrawn.
    NothingToWithdraw = 12,
    /// The contract is already registered by other account
    AlreadyRegisteredContract = 13,
    /// User attempts to register with address which is not contract
    ContractIsNotValid = 14,
    /// This account was already used to register contract
    AlreadyUsedDeveloperAccount = 15,
    /// Smart contract not owned by the account id.
    NotOwnedContract = 16,
    /// Report issue on github if this is ever emitted
    UnknownEraReward = 17,
    /// Report issue on github if this is ever emitted
    UnexpectedStakeInfoEra = 18,
    /// Contract has too many unlocking chunks. Withdraw the existing chunks if possible
    /// or wait for current chunks to complete unlocking process to withdraw them.
    TooManyUnlockingChunks = 19,
    /// Contract already claimed in this era and reward is distributed
    AlreadyClaimedInThisEra = 20,
    /// Era parameter is out of bounds
    EraOutOfBounds = 21,
    /// Too many active `EraStake` values for (staker, contract) pairing.
    /// Claim existing rewards to fix this problem.
    TooManyEraStakeValues = 22,
    /// To register a contract, pre-approval is needed for this address
    RequiredContractPreApproval = 23,
    /// Developer's account is already part of pre-approved list
    AlreadyPreApprovedDeveloper = 24,
    /// Account is not actively staking
    NotActiveStaker = 25,
    /// Transfering nomination to the same contract
    NominationTransferToSameContract = 26,
    /// Unexpected reward destination value
    RewardDestinationValueOutOfBounds = 27,
    /// Unknown error
    UnknownError = 99,
}

impl ink::env::chain_extension::FromStatusCode for DSError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Disabled),
            2 => Err(Self::NoMaintenanceModeChange),
            3 => Err(Self::UpgradeTooHeavy),
            4 => Err(Self::StakingWithNoValue),
            5 => Err(Self::InsufficientValue),
            6 => Err(Self::MaxNumberOfStakersExceeded),
            7 => Err(Self::NotOperatedContract),
            8 => Err(Self::NotStakedContract),
            9 => Err(Self::NotUnregisteredContract),
            10 => Err(Self::UnclaimedRewardsRemaining),
            11 => Err(Self::UnstakingWithNoValue),
            12 => Err(Self::NothingToWithdraw),
            13 => Err(Self::AlreadyRegisteredContract),
            14 => Err(Self::ContractIsNotValid),
            15 => Err(Self::AlreadyUsedDeveloperAccount),
            16 => Err(Self::NotOwnedContract),
            17 => Err(Self::UnknownEraReward),
            18 => Err(Self::UnexpectedStakeInfoEra),
            19 => Err(Self::TooManyUnlockingChunks),
            20 => Err(Self::AlreadyClaimedInThisEra),
            21 => Err(Self::EraOutOfBounds),
            22 => Err(Self::TooManyEraStakeValues),
            23 => Err(Self::RequiredContractPreApproval),
            24 => Err(Self::AlreadyPreApprovedDeveloper),
            25 => Err(Self::NotActiveStaker),
            26 => Err(Self::NominationTransferToSameContract),
            27 => Err(Self::RewardDestinationValueOutOfBounds),
            99 => Err(Self::UnknownError),
            _ => panic!("encountered unknown status code"),
        }
    }
}
