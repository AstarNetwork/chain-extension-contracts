#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod rmrk_example {
    use ink_prelude::vec::Vec;
    use rmrk_extension::*;

	#[ink(storage)]
	pub struct RmrkExample {}

	impl RmrkExample {
		#[ink(constructor)]
		pub fn new() -> Self {
			Self {}
		}

		// READ functions
		#[ink(message)]
		pub fn collection_index(&self) -> CollectionId {
            Rmrk::collection_index()
		}

		#[ink(message)]
		pub fn collections(&self, collection_id: CollectionId) -> Option<CollectionInfo> {
            Rmrk::collections(collection_id)
		}

		#[ink(message)]
		pub fn nfts(&self, collection_id: CollectionId, nft_id: NftId) -> Option<NftInfo> {
            Rmrk::nfts(collection_id, nft_id)
		}

		#[ink(message)]
		pub fn priorities(
			&self,
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		) -> Option<u32> {
            Rmrk::priorities(collection_id, nft_id, resource_id)
		}

		#[ink(message)]
		pub fn children(
			&self,
			parent: (CollectionId, NftId),
			child: (CollectionId, NftId),
		) -> Option<()> {
            Rmrk::children(parent, child)
		}

		#[ink(message)]
		pub fn resources(
			&self,
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
		) -> Option<ResourceInfo> {
			Rmrk::resources(collection_id, nft_id, resource_id)
		}

		#[ink(message)]
		pub fn equippable_bases(
			&self,
			collection_id: CollectionId,
			nft_id: NftId,
			base_id: BaseId,
		) -> Option<()> {
			Rmrk::equippable_bases(collection_id, nft_id, base_id)
		}

		#[ink(message)]
		pub fn equippable_slots(
			&self,
			collection_id: CollectionId,
			nft_id: NftId,
			resource_id: ResourceId,
			base_id: BaseId,
			slot_id: SlotId,
		) -> Option<()> {
			Rmrk::equippable_slots(
                collection_id,
                nft_id,
                resource_id,
                base_id,
                slot_id,
            )
		}

		#[ink(message)]
		pub fn properties(
		    &self,
		    collection_id: CollectionId,
		    nft_id: Option<NftId>,
		    key: Vec<u8>,
		) -> Option<Vec<u8>> {
            Rmrk::properties(
                collection_id,
                nft_id,
                key,
            )
		}

		#[ink(message)]
		pub fn lock(&self, collection_id: CollectionId, nft_id: NftId) -> bool {
            Rmrk::lock(collection_id, nft_id)
		}

		/// write functions
		#[ink(message)]
		pub fn mint_ntf(
		    &mut self,
		    owner: AccountId,
			nft_id: NftId,
		    collection_id: CollectionId,
		    royalty_recipient: Option<AccountId>,
		    royalty: Option<u32>,
		    metadata: Vec<u8>,
		    transferable: bool,
		    resources: Option<Vec<ResourceTypes>>,
		) -> Result<(), RmrkError> {
            Rmrk::mint_nft(
                owner,
				nft_id,
                collection_id,
                royalty_recipient,
                royalty,
                metadata,
                transferable,
                resources,
            )
		}

		#[ink(message)]
		pub fn mint_ntf_directly_to_nft(
		    &mut self,
		    owner: (CollectionId, NftId),
			nft_id: NftId,
		    collection_id: CollectionId,
		    royalty_recipient: Option<AccountId>,
		    royalty: Option<u32>,
		    metadata: Vec<u8>,
		    transferable: bool,
		    resources: Option<Vec<ResourceTypes>>,
		) -> Result<(), RmrkError> {
            Rmrk::mint_nft_directly_to_nft(
                owner,
				nft_id,
                collection_id,
                royalty_recipient,
                royalty,
                metadata,
                transferable,
                resources,
            )
		}

		#[ink(message, payable)]
		pub fn create_collection(
			&mut self,
			metadata: Vec<u8>,
			max: Option<u32>,
			symbol: Vec<u8>,
		) -> Result<(), RmrkError> {
			ink_env::debug_println!("create collection enter in contract {:?}", metadata);
			let result = Rmrk::create_collection(
                metadata,
                max,
                symbol,
            );
			ink_env::debug_println!("create collection result in contract {:?}", result);
			Ok(())
		}

		#[ink(message)]
		pub fn burn_nft(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    max_burns: u32,
		) -> Result<(), RmrkError> {
            Rmrk::burn_nft(
                collection_id,
                nft_id,
                max_burns,
            )
		}

		#[ink(message)]
		pub fn destroy_collection(&mut self, collection_id: CollectionId) -> Result<(), RmrkError> {
            Rmrk::destroy_collection(collection_id)
		}

		#[ink(message)]
		pub fn send(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    new_owner: AccountIdOrCollectionNftTuple,
		) -> Result<(), RmrkError> {
            Rmrk::send(
                collection_id,
                nft_id,
                new_owner,
            )
		}

		#[ink(message)]
		pub fn accept_nft(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    new_owner: AccountIdOrCollectionNftTuple,
		) -> Result<(), RmrkError> {
            Rmrk::accept_nft(
                collection_id,
                nft_id,
                new_owner,
            )
		}

		#[ink(message)]
		pub fn reject_nft(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		) -> Result<(), RmrkError> {
            Rmrk::reject_nft(collection_id, nft_id)
		}

		#[ink(message)]
		pub fn change_collection_issuer(
		    &mut self,
		    collection_id: CollectionId,
		    new_issuer: AccountId,
		) -> Result<(), RmrkError> {
            Rmrk::change_collection_issuer(collection_id, new_issuer)
		}

		#[ink(message)]
		    pub fn set_property(
		    &mut self,
		    collection_id: CollectionId,
		    maybe_nft_id: Option<NftId>,
		    key: Vec<u8>,
		    value: Vec<u8>,
		) -> Result<(), RmrkError> {
            Rmrk::set_property(
                collection_id,
                maybe_nft_id,
                key,
                value,
            )
		}

		#[ink(message)]
		pub fn lock_collection(&mut self, collection_id: CollectionId) -> Result<(), RmrkError> {
            Rmrk::lock_collection(collection_id)
		}

		#[ink(message)]
		pub fn add_basic_resource(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    resource: BasicResource,
			resource_id: ResourceId,
		) -> Result<(), RmrkError> {
            Rmrk::add_basic_resource(
                collection_id,
                nft_id,
                resource,
				resource_id
            )
		}

		#[ink(message)]
		pub fn add_composable_resource(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    resource: ComposableResource,
			resource_id: ResourceId,
		) -> Result<(), RmrkError> {
            Rmrk::add_composable_resource(
                collection_id,
                nft_id,
                resource,
				resource_id
            )
		}

		#[ink(message)]
		pub fn add_slot_resource(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    resource: SlotResource,
			resource_id: ResourceId,
		) -> Result<(), RmrkError> {
            Rmrk::add_slot_resource(
                collection_id,
                nft_id,
                resource,
				resource_id
            )
		}

		#[ink(message)]
		pub fn accept_resource(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    resource_id: ResourceId,
		) -> Result<(), RmrkError> {
            Rmrk::accept_resource(
                collection_id,
                nft_id,
                resource_id,
            )
		}

		#[ink(message)]
		pub fn remove_resource(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    resource_id: ResourceId,
		) -> Result<(), RmrkError> {
            Rmrk::remove_resource(
                collection_id,
                nft_id,
                resource_id,
            )
		}

		#[ink(message)]
		pub fn accept_resource_removal(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    resource_id: ResourceId,
		) -> Result<(), RmrkError> {
            Rmrk::accept_resource_removal(
                collection_id,
                nft_id,
                resource_id,
            )
		}

		#[ink(message)]
		pub fn set_priority(
		    &mut self,
		    collection_id: CollectionId,
		    nft_id: NftId,
		    priorities: Vec<ResourceId>,
		) -> Result<(), RmrkError> {
            Rmrk::set_priority(
                collection_id,
                nft_id,
                priorities,
            )
		}
	}

	#[cfg(test)]
	mod tests {
		/// Imports all the definitions from the outer scope so we can use them here.
		use super::*;

		/// Imports `ink_lang` so we can use `#[ink::test]`.
		use ink_lang as ink;

		#[ink::test]
        fn chain_extension_works() {
            // given
            struct MockedExtension;
            impl ink_env::test::ChainExtension for MockedExtension {
                /// The static function id of the chain extension.
                fn func_id(&self) -> u32 {
                    0x0001000F // create_collection()
                }

                /// The chain extension is called with the given input.
                ///
                /// Returns an error code and may fill the `output` buffer with a
                /// SCALE encoded result. The error code is taken from the
                /// `ink_env::chain_extension::FromStatusCode` implementation for
                /// `DappsStakingResponseError`.
                fn call(&mut self, _input: &[u8], output: &mut Vec<u8>) -> u32 {
                    let ret: u32 = 1;
                    scale::Encode::encode_to(&ret, output);
                    0
                }
            }
            ink_env::test::register_chain_extension(MockedExtension);
            _create_collection();
        }

		fn _create_collection() {
			let mut rmrk = RmrkExample::new();
			let metadata = "ipfs://ipfs/QmTG9ekqrdMh3dsehLYjC19fUSmPR31Ds2h6Jd7LnMZ9c7";
			let symbol = "ROO";
			let crete_result = rmrk.create_collection(metadata.into(), None, symbol.into());
			assert_eq!(crete_result.is_err(), true);
		}
	}
}
