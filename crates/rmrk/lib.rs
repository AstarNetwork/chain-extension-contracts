#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
use ink_prelude::vec::Vec;
use scale::{Decode, Encode};

// use rmrk_chain_extension_types::RmrkError;

// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
// #[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
// pub enum RmrkError {}

#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
pub enum RmrkError {
    /// Error names should be descriptive.
    Success,
    /// Errors should have helpful documentation associated with them.
    StorageOverflow,
    TooLong,
    NoAvailableCollectionId,
    NoAvailableResourceId,
    MetadataNotSet,
    RecipientNotSet,
    NoAvailableNftId,
    NotInRange,
    RoyaltyNotSet,
    CollectionUnknown,
    NoPermission,
    NoWitness,
    CollectionNotEmpty,
    CollectionFullOrLocked,
    CannotSendToDescendentOrSelf,
    ResourceAlreadyExists,
    NftAlreadyExists,
    EmptyResource,
    TooManyRecursions,
    NftIsLocked,
    CannotAcceptNonOwnedNft,
    CannotRejectNonOwnedNft,
    CannotRejectNonPendingNft,
    ResourceDoesntExist,
    /// Accepting a resource that is not pending should fail
    ResourceNotPending,
    NonTransferable,
    // Must unequip an item before sending (this only applies to the
    // rmrk-equip pallet but the send operation lives in rmrk-core)
    CannotSendEquippedItem,
}

impl From<scale::Error> for RmrkError {
	fn from(_: scale::Error) -> Self {
		panic!("encountered unexpected invalid SCALE encoding")
	}
}

// impl ink_env::chain_extension::FromStatusCode for RmrkError {
// 	fn from_status_code(status_code: u32) -> Result<(), Self> {
// 		match status_code {
// 			0 => Ok(()),
// 			_ => panic!("encountered unknown status code"),
// 		}
// 	}
// }

impl ink_env::chain_extension::FromStatusCode for RmrkError {
	fn from_status_code(status_code: u32) -> Result<(), Self> {
		match status_code {
			0 => Ok(()),
            1 => Err(Self::StorageOverflow),
            2 => Err(Self::TooLong),
            3 => Err(Self::NoAvailableCollectionId),
            4 => Err(Self::NoAvailableResourceId),
            5 => Err(Self::MetadataNotSet),
            6 => Err(Self::RecipientNotSet),
            7 => Err(Self::NoAvailableNftId),
            8 => Err(Self::NotInRange),
            9 => Err(Self::RoyaltyNotSet),
            10 => Err(Self::CollectionUnknown),
            11 => Err(Self::NoPermission),
            12 => Err(Self::NoWitness),
            13 => Err(Self::CollectionNotEmpty),
            14 => Err(Self::CollectionFullOrLocked),
            15 => Err(Self::CannotSendToDescendentOrSelf),
            16 => Err(Self::ResourceAlreadyExists),
            17 => Err(Self::NftAlreadyExists),
            18 => Err(Self::EmptyResource),
            19 => Err(Self::TooManyRecursions),
            20 => Err(Self::NftIsLocked),
            21 => Err(Self::CannotAcceptNonOwnedNft),
            22 => Err(Self::CannotRejectNonOwnedNft),
            23 => Err(Self::CannotRejectNonPendingNft),
            24 => Err(Self::ResourceDoesntExist),
            25 => Err(Self::ResourceNotPending),
            26 => Err(Self::NonTransferable),
            27 => Err(Self::CannotSendEquippedItem),
			_ => panic!("RMRK contract encountered unknown status code"),
		}
	}
}

pub type CollectionId = u32;
pub type NftId = u32;
pub type ResourceId = u32;
pub type BaseId = u32;
pub type SlotId = u32;
pub type PartId = u32;

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct CollectionInfo {
	/// Current bidder and bid price.
	pub issuer: AccountId,

	pub metadata: Vec<u8>,
	pub max: Option<u32>,

	pub symbol: Vec<u8>,
	pub nfts_count: u32,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct NftInfo {
	/// The owner of the NFT, can be either an Account or a tuple (CollectionId, NftId)
	pub owner: AccountIdOrCollectionNftTuple,
	/// Royalty (optional)
	pub royalty: Option<RoyaltyInfo>,

	/// Arbitrary data about an instance, e.g. IPFS hash
	pub metadata: Vec<u8>,

	/// Equipped state
	pub equipped: bool,
	/// Pending state (if sent to NFT)
	pub pending: bool,
	/// transferability ( non-transferable is "souldbound" )
	pub transferable: bool,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AccountIdOrCollectionNftTuple {
	AccountId(AccountId),
	CollectionAndNftTuple(CollectionId, NftId),
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoyaltyInfo {
	/// Recipient (AccountId) of the royalty
	pub recipient: AccountId,
	/// Amount (Permill) of the royalty
	pub amount: u32,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ResourceInfo {
	pub id: ResourceId,

	resources: ResourceTypes,

	pub pending: bool,
	pub pending_removal: bool,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ResourceTypes {
	Basic(BasicResource),
	Composable(ComposableResource),
	Slot(SlotResource),
}
#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct BasicResource {
	/// Reference to IPFS location of metadata
	pub metadata: Vec<u8>,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ComposableResource {
	/// If a resource is composed, it will have an array of parts that compose it
	pub parts: Vec<PartId>,

	/// A Base is uniquely identified by the combination of the word `base`, its minting block
	/// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
	/// base-4477293-kanaria_superbird.
	pub base: BaseId,

	/// Reference to IPFS location of metadata
	pub metadata: Option<Vec<u8>>,

	/// If the resource has the slot property, it was designed to fit into a specific Base's slot.
	pub slot: Option<(BaseId, SlotId)>,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SlotResource {
	/// A Base is uniquely identified by the combination of the word `base`, its minting block
	/// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
	/// base-4477293-kanaria_superbird.
	pub base: BaseId,

	/// Reference to IPFS location of metadata
	pub metadata: Option<Vec<u8>>,

	/// If the resource has the slot property, it was designed to fit into a specific Base's slot.
	/// The baseslot will be composed of two dot-delimited values, like so:
	/// "base-4477293-kanaria_superbird.machine_gun_scope". This means: "This resource is
	/// compatible with the machine_gun_scope slot of base base-4477293-kanaria_superbird
	pub slot: SlotId,
}

pub struct Rmrk;

impl Rmrk {
    pub fn collection_index() -> CollectionId {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010002)
            .input::<()>()
            .output::<CollectionId>()
            .ignore_error_code()
            .call(&())
    }

    pub fn collections(collection_id: CollectionId) -> Option<CollectionInfo> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010004)
            .input::<CollectionId>()
            .output::<Option<CollectionInfo>>()
            .ignore_error_code()
            .call(&collection_id)
    }

    pub fn nfts(collection_id: CollectionId, nft_id: NftId) -> Option<NftInfo> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010005)
            .input::<(CollectionId, NftId)>()
            .output::<Option<NftInfo>>()
            .ignore_error_code()
            .call(&(collection_id, nft_id))
    }

    pub fn priorities(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
    ) -> Option<u32> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010006)
            .input::<(CollectionId, NftId, ResourceId)>()
            .output::<Option<u32>>()
            .ignore_error_code()
            .call(&(collection_id, nft_id, resource_id))
    }

    pub fn children(
        parent: (CollectionId, NftId),
        child: (CollectionId, NftId),
    ) -> Option<()> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010007)
            .input::<(
                (CollectionId, NftId),
                (CollectionId, NftId)
            )>()
            .output::<Option<()>>()
            .ignore_error_code()
            .call(&(parent, child))
    }

    pub fn resources(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
    ) -> Option<ResourceInfo> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010008)
            .input::<(CollectionId, NftId, ResourceId)>()
            .output::<Option<ResourceInfo>>()
            .ignore_error_code()
            .call(&(collection_id, nft_id, resource_id))
    }

    pub fn equippable_bases(
        collection_id: CollectionId,
        nft_id: NftId,
        base_id: BaseId,
    ) -> Option<()> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010009)
            .input::<(CollectionId, NftId, BaseId)>()
            .output::<Option<()>>()
            .ignore_error_code()
            .call(&(collection_id, nft_id, base_id))
    }

    pub fn equippable_slots(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
        base_id: BaseId,
        slot_id: SlotId,
    ) -> Option<()> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001000A)
            .input::<(
                CollectionId,
                NftId,
                ResourceId,
                BaseId,
                SlotId
            )>()
            .output::<Option<()>>()
            .ignore_error_code()
            .call(&(
                collection_id,
                nft_id,
                resource_id,
                base_id,
                slot_id
            ))
    }

    pub fn properties(
        collection_id: CollectionId,
        nft_id: Option<NftId>,
        key: Vec<u8>,
    ) -> Option<Vec<u8>> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001000B)
            .input::<(CollectionId, Option<NftId>, Vec<u8>)>()
            .output::<Option<Vec<u8>>>()
            .ignore_error_code()
            .call(&(collection_id, nft_id, key))
    }

    pub fn lock(collection_id: CollectionId, nft_id: NftId) -> bool {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001000C)
            .input::<(CollectionId, NftId)>()
            .output::<bool>()
            .ignore_error_code()
            .call(&(collection_id, nft_id))
    }

    pub fn mint_nft(
        owner: AccountId,
        nft_id: NftId,
        collection_id: CollectionId,
        royalty_recipient: Option<AccountId>,
        royalty: Option<u32>,
        metadata: Vec<u8>,
        transferable: bool,
        resources: Option<Vec<ResourceTypes>>,
    ) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001000D)
            .input::<(
                AccountId,
                NftId,
                CollectionId,
                Option<AccountId>,
                Option<u32>,
                Vec<u8>,
                bool,
                Option<Vec<ResourceTypes>>,
            )>()
            .output::<()>()
            .handle_error_code::<RmrkError>()
            .call(&(
                owner,
                nft_id,
                collection_id,
                royalty_recipient,
                royalty,
                metadata,
                transferable,
                resources,
            ))
    }

    pub fn mint_nft_directly_to_nft(
        owner: (CollectionId, NftId),
        nft_id: NftId,
        collection_id: CollectionId,
        royalty_recipient: Option<AccountId>,
        royalty: Option<u32>,
        metadata: Vec<u8>,
        transferable: bool,
        resources: Option<Vec<ResourceTypes>>,
    ) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001000E)
            .input::<(
                (CollectionId, NftId),
                NftId,
                CollectionId,
                Option<AccountId>,
                Option<u32>,
                Vec<u8>,
                bool,
                Option<Vec<ResourceTypes>>,
            )>()
            .output::<()>()
            .handle_error_code::<RmrkError>()
            .call(&(
                owner,
                nft_id,
                collection_id,
                royalty_recipient,
                royalty,
                metadata,
                transferable,
                resources,
            ))
    }

    pub fn create_collection(
        metadata: Vec<u8>,
        max: Option<u32>,
        symbol: Vec<u8>,
    ) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001000F)
            .input::<(Vec<u8>, Option<u32>, Vec<u8>)>()
            .output::<()>()
            .handle_error_code::<RmrkError>()
            .call(&(metadata, max, symbol))
    }

    pub fn burn_nft(
        collection_id: CollectionId,
        nft_id: NftId,
        max_burns: u32,
    ) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010010)
            .input::<(CollectionId, NftId, u32)>()
            .output::<()>()
            .handle_error_code::<RmrkError>()
            .call(&(collection_id, nft_id, max_burns))
    }

    pub fn destroy_collection(collection_id: CollectionId) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010011)
            .input::<CollectionId>()
            .output::<()>()
            .handle_error_code::<RmrkError>()
            .call(&collection_id)
    }

	pub fn send(
		collection_id: CollectionId,
		nft_id: NftId,
		new_owner: AccountIdOrCollectionNftTuple,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010012)
			.input::<(CollectionId, NftId, AccountIdOrCollectionNftTuple)>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(collection_id, nft_id, new_owner))
	}

	pub fn accept_nft(
		collection_id: CollectionId,
		nft_id: NftId,
		new_owner: AccountIdOrCollectionNftTuple,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010013)
			.input::<(CollectionId, NftId, AccountIdOrCollectionNftTuple)>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(collection_id, nft_id, new_owner))
	}

	pub fn reject_nft(
		collection_id: CollectionId,
		nft_id: NftId,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010014)
			.input::<(CollectionId, NftId)>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(collection_id, nft_id))
	}

	pub fn change_collection_issuer(
		collection_id: CollectionId,
		new_issuer: AccountId,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010015)
			.input::<(CollectionId, AccountId)>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(collection_id, new_issuer))
	}

	pub fn set_property(
		collection_id: CollectionId,
		maybe_nft_id: Option<NftId>,
		key: Vec<u8>,
		value: Vec<u8>,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010016)
			.input::<(
                CollectionId,
                Option<NftId>,
                Vec<u8>,
                Vec<u8>,
            )>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(
                collection_id,
                maybe_nft_id,
                key,
                value,
            ))
	}

	pub fn lock_collection(collection_id: CollectionId) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010017)
			.input::<CollectionId>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&collection_id)
	}

	pub fn add_basic_resource(
		collection_id: CollectionId,
		nft_id: NftId,
		resource: BasicResource,
        resource_id: ResourceId,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010018)
        .input::<(CollectionId, NftId, BasicResource, ResourceId)>()
        .output::<()>()
        .handle_error_code::<RmrkError>()
        .call(&(collection_id, nft_id, resource, resource_id))
	}

	pub fn add_composable_resource(
        collection_id: CollectionId,
		nft_id: NftId,
		resource: ComposableResource,
        resource_id: ResourceId,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x00010019)
        .input::<(CollectionId, NftId, ComposableResource, ResourceId)>()
        .output::<()>()
        .handle_error_code::<RmrkError>()
        .call(&(collection_id, nft_id, resource, resource_id))
	}

	pub fn add_slot_resource(
        collection_id: CollectionId,
		nft_id: NftId,
		resource: SlotResource,
        resource_id: ResourceId,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001001A)
			.input::<(CollectionId, NftId, SlotResource, ResourceId)>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(collection_id, nft_id, resource, resource_id))
	}

	pub fn accept_resource(
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001001B)
			.input::<(CollectionId, NftId, ResourceId)>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(collection_id, nft_id, resource_id))
	}

	pub fn remove_resource(
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001001C)
			.input::<(CollectionId, NftId, ResourceId)>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(collection_id, nft_id, resource_id))
	}

	pub fn accept_resource_removal(
		collection_id: CollectionId,
		nft_id: NftId,
		resource_id: ResourceId,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001001D)
			.input::<(CollectionId, NftId, ResourceId)>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(collection_id, nft_id, resource_id))
	}

	pub fn set_priority(
		collection_id: CollectionId,
		nft_id: NftId,
		priorities: Vec<ResourceId>,
	) -> Result<(), RmrkError> {
        ::ink_env::chain_extension::ChainExtensionMethod::build(0x0001001E)
			.input::<(CollectionId, NftId, Vec<ResourceId>)>()
            .output::<()>()
			.handle_error_code::<RmrkError>()
			.call(&(collection_id, nft_id, priorities))
	}
}


