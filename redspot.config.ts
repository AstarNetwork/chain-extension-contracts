import { RedspotUserConfig } from 'redspot/types'
import '@redspot/patract'
import '@redspot/chai'
import '@redspot/gas-reporter'

const types = {
    minmax: [0, undefined],
    types: {
        Keys: 'AccountId',
        ShidenRuntimeSmartContract: {
            _enum: {
                Evm: 'H160',
                Wasm: 'AccountId'
            }
        },
        PalletDappsStakingEraIndex: 'u32',
        PalletDappsStakingEraStakingPoints: {
            total: 'Balance',
            stakers: 'BTreeMap<AccountId, Balance>',
            _formerStakedEra: 'EraIndex',
            claimedRewards: 'Balance'
        },
        PalletDappsStakingEraRewardAndStake: {
            rewards: 'Balance',
            staked: 'Balance'
        },
        PalletDappsStakingForcing: {
            _enum: ['NotForcing', 'ForceNew', 'ForceNone', 'ForceAlways']
        }
    }
}

export default {
    defaultNetwork: 'development',
    contract: {
        ink: {
            toolchain: 'nightly',
            // sources: ['examples/dapps-staking']
            sources: ['examples/rmrk']
        }
    },
    networks: {
        development: {
            endpoint: 'ws://127.0.0.1:9944',
            gasLimit: '400000000000',
            explorerUrl: 'https://polkadot.js.org/apps/#/explorer/query/?rpc=ws://127.0.0.1:9944/',
            types: {
                RmrkExtensionRmrkError: {
                    _enum: {
                        NoneValue: null,
                        StorageOverflow: null,
                        TooLong: null,
                        NoAvailableCollectionId: null,
                        NoAvailableResourceId: null,
                        MetadataNotSet: null,
                        RecipientNotSet: null,
                        NoAvailableNftId: null,
                        NotInRange: null,
                        RoyaltyNotSet: null,
                        CollectionUnknown: null,
                        NoPermission: null,
                        NoWitness: null,
                        CollectionNotEmpty: null,
                        CollectionFullOrLocked: null,
                        CannotSendToDescendentOrSelf: null,
                        ResourceAlreadyExists: null,
                        NftAlreadyExists: null,
                        EmptyResource: null,
                        TooManyRecursions: null,
                        NftIsLocked: null,
                        CannotAcceptNonOwnedNft: null,
                        CannotRejectNonOwnedNft: null,
                        CannotRejectNonPendingNft: null,
                        ResourceDoesntExist: null,
                        ResourceNotPending: null,
                        NonTransferable: null,
                        CannotSendEquippedItem: null,
                    },
                },
            },
        },
        substrate: {
            endpoint: 'ws://127.0.0.1:9944',
            gasLimit: '400000000000',
            accounts: ['//Alice'],
        }
    },
    mocha: {
        timeout: 600000
    }
} as RedspotUserConfig