import {bool, Struct, u32, Vec} from "@polkadot/types";
import {AccountId, Balance} from "@polkadot/types/interfaces";

export type EraIndex = u32;

export interface EraStake extends Struct {
    staked: Balance;
    era: number;
}

export interface GeneralStakerInfo extends Struct {
    readonly stakes: EraStake[];
}

export interface EraStakingPointsIndividualClaim extends Struct {
    total: Balance;
    numberOfStakers: u32;
    contractRewardClaimed: bool;
}

export interface RegisteredDapps extends Struct {
    readonly developer: AccountId;
    readonly state: State;
}

export interface State {
    isUnregistered: boolean;
    asUnregistered: {
        // Memo: era of unregistered
        words: number[];
    };
}

export interface EraInfo extends Struct {
    rewards: {
        stakers: Balance;
        dapps: Balance;
    };
    staked: Balance;
    locked: Balance;
}

export enum RewardDestination {
    FreeBalance = 'FreeBalance',
    StakeBalance = 'StakeBalance',
}

export interface UnlockingChunk extends Struct {
    amount: Balance;
    unlockEra: EraIndex;
}

export interface UnbondingInfo extends Struct {
    unlockingChunks: Vec<UnlockingChunk>;
}

export interface AccountLedger extends Struct {
    locked: Balance;
    unbondingInfo: UnbondingInfo;
    rewardDestination: RewardDestination;
}