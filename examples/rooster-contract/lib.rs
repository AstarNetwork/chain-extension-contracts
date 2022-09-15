#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use scale::{Decode, Encode};


use rmrk_extension::*;

#[derive(Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkErrorCode {
    Failed,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkError {
    ErrorCode(RmrkErrorCode),
}


impl From<RmrkErrorCode> for RmrkError {
    fn from(error_code: RmrkErrorCode) -> Self {
        Self::ErrorCode(error_code)
    }
}

impl From<scale::Error> for RmrkError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink_env::chain_extension::FromStatusCode for RmrkErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Failed),
            _ => panic!("encountered unknown status code"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

#[openbrush::contract]
pub mod governor {
    
    use ink_storage::traits::{
        SpreadAllocate,
        SpreadLayout,
        PackedLayout,
    };

    #[cfg(feature = "std")]
    use ink_storage::traits::StorageLayout;

    use ink_prelude::vec;
    use ink_prelude::vec::Vec;

    use super::{
        CollectionId,
        NftId,
        BasicResource,
    };
    
    use ink_prelude::string::{
        String,
    };

    use ink_env::hash::Blake2x256;
    
    use openbrush::{
        storage::Mapping,
        contracts::timelock_controller::*,
    };
    
    use rmrk_extension::*;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ProposalState {
        Pending,
        Active,
        Canceled,
        Defeated,
        Succeeded,
        Queued,
        Expired,
        Executed
    }

    #[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum GovernorError {
        InsufficientVotingPower,
        ProposalAlreadyExists,
        ProposalDoesNotExist,
        NotOpenForVoting,
        HasAlreadyVoted,
        VoteHasNotSucceeded,
        NotOwner,
        InsufficientAmount,
        AlreadyOwner,
        MintFailed,
        AddResourceFailed,
        InternalError,
        CreateCollectionFailed,
    }

    #[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum VoteType {
        Against,
        For,
        Abstain
    }

    #[derive(Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ProposalVote {
        pub votes_against: u32,
        pub votes_for:     u32,
        pub votes_abstain: u32,
        // Todo: nest mapping 
        pub has_voted: Vec<AccountId>,
    }


    #[derive(Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ProposalCore {
        pub vote_start: Timestamp,
        pub vote_end:   Timestamp,
        pub executed: bool,
        pub canceled: bool,
    }

    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        proposal_id: OperationId,
        #[ink(topic)]
        proposer: AccountId,
        transaction: Transaction,
        description: String,
        vote_start: Timestamp,
        vote_end: Timestamp,
    }

    #[ink(event)]
    pub struct VoteCast {
        #[ink(topic)]
        voter: AccountId,
        #[ink(topic)]
        proposal_id: OperationId,
        vote: VoteType,
    }

    #[ink(event)]
    pub struct DelegateChanged {
        #[ink(topic)]
        delegator: AccountId,
        from_delegate: AccountId,
        to_delegate: AccountId,
    }

    #[ink(event)]
    pub struct DelegateVotesChanged {
        #[ink(topic)]
        delegate: AccountId,
        votes: u32,
    }

    #[ink(event)]
    pub struct CollectionCreated {
        #[ink(topic)]
        collection_id: CollectionId,
        symbol: String,        
    }

    #[ink(storage)]
    #[derive(Default,SpreadAllocate,TimelockControllerStorage)]
    pub struct Governor {
        #[TimelockControllerStorageField]
        timelock: TimelockControllerData,
        name: Option<String>,
        // Governor
        proposals: Mapping<OperationId, ProposalCore>,
        proposal_ids: Vec<OperationId>,
        votes: Mapping<OperationId, ProposalVote>,
        voting_delay: Timestamp,
        voting_period: Timestamp,
        // NFT
        collection_id: Option<CollectionId>,
        owners: Vec<AccountId>,
        owners_nft: Mapping<AccountId, NftId>,
        owners_lvl: Mapping<AccountId, u32>,
        price: Balance,
        // Delegations (Temporary implementation)
        delegations: Mapping<BlockNumber, (AccountId, AccountId)>,
        delegation_blocks: Vec<BlockNumber>,
    }


    impl Governor {
        #[ink(constructor, payable)]
        pub fn new(
            name: Option<String>,
            voting_delay: Timestamp,
            voting_period: Timestamp,
            execution_delay: Timestamp,
            nft_price: Balance,
        ) -> Self {
            ink_lang::utils::initialize_contract(|instance: &mut Self| {
                instance.name = name;
                instance.voting_delay = voting_delay;
                instance.voting_period = voting_period;

                let caller = instance.env().caller();
                let callee = instance.env().account_id();
                let calee_vec = vec![callee];

                instance.price = nft_price;
                
                // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
                // You need to call it for each trait separately, to initialize everything for these traits.
                AccessControlInternal::_init_with_admin(instance, caller);
                TimelockControllerInternal::_init_with_admin(instance, caller, execution_delay, calee_vec.clone(), calee_vec);
                
            })
        }

        //////////////////////////////
        /// Governor internal
        /// 

        fn _emit_proposal_created(
            &self, 
            proposal_id : OperationId,
            proposer: AccountId,
            transaction: Transaction,
            description: String,
            vote_start: Timestamp,
            vote_end: Timestamp,
        ) {
            self.env()
            .emit_event(ProposalCreated { 
                proposal_id,
                proposer,
                transaction,
                description,
                vote_start,
                vote_end,
             })
        }

        fn _emit_vote_cast(
            &self,
            voter: AccountId,
            proposal_id: OperationId,
            vote: VoteType,
        ) {
            self.env()
            .emit_event( VoteCast {
                voter,
                proposal_id,
                vote
            })
        }


        fn _emit_delegate_changed(
            &self,
            delegator: AccountId,
            to_delegate: AccountId,
            from_delegate: AccountId,
        ) {            
            self.env()
            .emit_event (
                DelegateChanged {
                    delegator,
                    from_delegate,
                    to_delegate,
                })
        }

        fn _emit_delegate_votes_changed(
            &self,
            delegate: AccountId
        ) {

            let votes = self._get_votes(delegate, None);

            self.env()
            .emit_event (
                DelegateVotesChanged {
                    delegate,
                    votes
                })

        }

        fn _emit_collection_created(
            &self,
            collection_id: CollectionId,
            symbol: String,
        ) {
            self.env()
            .emit_event (
                CollectionCreated {
                    collection_id,
                    symbol,
                })
        }


        fn _get_delegate(&self, delegator: AccountId) -> AccountId {
        
            for block in self.delegation_blocks.iter().rev() {
                let (cur_delegator, cur_delegate) = self.delegations.get(&block).unwrap();
                    if cur_delegator == delegator {
                        return cur_delegate
                    }
                }
            
            return AccountId::default();

        }


        fn _hash_proposal(&self, transaction: Transaction, description_hash: [u8; 32]) -> OperationId {
            TimelockController::hash_operation(self, transaction,None, description_hash)
        }

        fn _hash_description(&self, description: String) -> [u8; 32] {
            self.env().hash_bytes::<Blake2x256>(description.as_bytes())
            
        }


        /// Verifies account owns required NFT
        /// 
        /// # Errors
        /// 
        /// 
        ///    `NotOwner` if not 
        fn _has_required_nft(
            &self, 
            caller: AccountId
        ) -> Result<(),GovernorError> {
            
            if !self.owners_nft.contains(&caller) {
               return Err(GovernorError::NotOwner)    
            }

            Ok(())
        }

        fn _get_votes(&self, account: AccountId, blocknumber_o: Option<BlockNumber>) -> u32 {
            let block_limit = match blocknumber_o {
                None => self.env().block_number(),
                Some(bn) => bn
            };

            let mut result : u32 = 0;
            let mut already_seen : Vec<AccountId> = Vec::new(); 
            for block in self.delegation_blocks.iter().rev() {
                if block > &block_limit {
                    continue;
                }

                let (cur_delegator, cur_delegate) = self.delegations.get(&block).unwrap();
                if !already_seen.contains(&cur_delegator) {
                    already_seen.push(cur_delegator);
                    if cur_delegate == account {
                        result += 1
                    }
                }
            }
            
            ink_env::debug_println!("_get_votes: blocknumber={:?} account={:?} result={:?}", block_limit, account, result);
            result
        }

        /// Verifies account has voting power
        ///
        /// # Errors
        ///
        ///     Returns with `InsufficientVotingPower` if voting power is not available
        fn _has_voting_power(&self, caller: AccountId) -> Result<(),GovernorError> {
           let voting_power = self._get_votes(caller, None);
           if voting_power < 1 {
               Err(GovernorError::InsufficientVotingPower)
           }  else {
            Ok(())
           }
        }

       


        fn _cast_vote(
            &mut self, 
            proposal_id: OperationId, 
            vote: VoteType, 
        )  -> Result<(),GovernorError> {
            let caller = self.env().caller();
            self._has_voting_power(caller)?;

            if !self.proposals.contains(&proposal_id) {
                return Err(GovernorError::ProposalDoesNotExist)
            }

            if self.state(proposal_id) != ProposalState::Active {
                return Err(GovernorError::NotOpenForVoting)
            }

            let mut vote_status = self.votes.get(&proposal_id).unwrap();
            
            
            if vote_status.has_voted.contains(&caller) {
                return Err(GovernorError::HasAlreadyVoted)
            }

            let voting_power = self._get_votes(caller, None);
            match vote {
                VoteType::Against => vote_status.votes_against += voting_power,
                VoteType::For     => vote_status.votes_for     += voting_power,
                VoteType::Abstain => vote_status.votes_abstain += voting_power,
            };

            vote_status.has_voted.push(caller);
            self.votes.insert(&proposal_id, &vote_status);
            ink_env::debug_println!("_cast_vote: caller={:?} vote_status={:?}", caller, vote_status);

            self._emit_vote_cast(caller,proposal_id,vote);

            self._evolve_from_delegate(caller)?;

            Ok(())
            
        }

        fn _execute(
            &self, 
            proposal_id: OperationId
        ) -> Result<(), GovernorError> {
            //does the proposal exist?
            if !self.proposals.contains(&proposal_id) {
                return Err(GovernorError::ProposalDoesNotExist)
            }

            if self.state(proposal_id) != ProposalState::Succeeded {
                return Err(GovernorError::VoteHasNotSucceeded)
            }

            //TODO: finish this....
            Ok(())
        }

        
        fn _create_collection_metadata(
            &mut self,
            metadata: String,
            symbol: String,
        ) -> Result<(), GovernorError> {
            if self.collection_id != None {
                return Err(GovernorError::InternalError);
            }

            self.collection_id = Some(Rmrk::collection_index());

            Rmrk::create_collection(
                metadata.into_bytes(),
                None,
                symbol.clone().into_bytes(),
            ).map_err(|_| GovernorError::CreateCollectionFailed)?;

            self._emit_collection_created(self.collection_id.unwrap(), symbol);
            Ok(())
            
        }


        fn _create_collection(&mut self) -> Result<(), GovernorError> {
              
              let metadata = "ipfs://ipfs/QmTG9ekqrdMh3dsehLYjC19fUSmPR31Ds2h6Jd7LnMZ9c7";

              let symbol = "ROO";

              self._create_collection_metadata(metadata.into(), symbol.into())
        }

        fn _evolve_owner(&mut self, account: AccountId) -> Result<(),GovernorError> {
            let cur_lvl : ResourceId = self.owners_lvl.get(&account).unwrap();
            let nft_id  : NftId = self.owners_nft.get(&account).unwrap();

            let next_lvl_metadata = match cur_lvl {
                0 => "ipfs://ipfs/QmeeCx81m6RVjmzbHjdeHABa7ksVPymwvXRWSuXSnvpoYG",
                1 => "ipfs://ipfs/QmSvdCbp8VCPcptoQfZUZ725fd3gyuc8bao1qpykba9zEm",
                2 => "ipfs://ipfs/QmXCHpDw6cPGUzksURJ4rXQsxoDKTYvjYKUzcffWmQyhBh",
                _ => "ipfs://ipfs/QmddZKVwg2jg1aqmFnqLAmpUAr8zM8asy8x6tSLuLdY1Sd",
            };

            if cur_lvl > 0 {
                let _result = Rmrk::remove_resource(
                    self.collection_id.unwrap(),
                    nft_id,
                    cur_lvl - 1
                );
            }

            let resource = BasicResource {
                metadata: next_lvl_metadata.as_bytes().to_vec(),
            };


            Rmrk::add_basic_resource(
                self.collection_id.unwrap(),
                nft_id,
                resource,
                cur_lvl,
            ).map_err(|_| GovernorError::AddResourceFailed)?;


   
            self.owners_lvl.insert(&account, &(cur_lvl + 1));

            Ok(())
        }


        fn _evolve_from_delegate(&mut self, delegate: AccountId) -> Result<(),GovernorError> {
            // evolve every owner that delegated to delegate
            let mut already_seen : Vec<AccountId> = Vec::new(); 
            let mut to_evolve: Vec<AccountId> = Vec::new(); 
            for block in self.delegation_blocks.iter().rev() {
                let (cur_delegator, cur_delegate) = self.delegations.get(&block).unwrap();
                if !already_seen.contains(&cur_delegator) {
                    already_seen.push(cur_delegator);
                    if cur_delegate == delegate {
                        to_evolve.push(cur_delegator);
                    }
                }
            }

            for account in to_evolve.iter() {
                self._evolve_owner(*account)?;
            }

            Ok(())
        }
       

        //////////////////////////////
        /// Governor read functions
        /// 
        
        /// returns whether account has voted for proposal_id 
        #[ink(message)]
        pub fn has_voted(&self, proposal_id: OperationId, account: AccountId) -> bool {
            if !self.votes.contains(&proposal_id) {
                return false;
            }
            let vote_status = self.votes.get(&proposal_id).unwrap();

            vote_status.has_voted.contains(&account)
        }

        #[ink(message)]
        pub fn name(&self) -> Option<String> {
            ink_env::debug_println!("name");
            self.name.clone()
        }

        #[ink(message)]
        pub fn proposal_deadline(&self, proposal_id: OperationId) -> Timestamp {
            assert!(self.proposals.contains(&proposal_id), "Proposal does noet exist");
            
            let proposal = self.proposals.get(&proposal_id).unwrap();

            proposal.vote_end

        }

        #[ink(message)]        
        pub fn proposal_snapshot(&self, proposal_id: OperationId) -> Timestamp {
            assert!(self.proposals.contains(&proposal_id), "Proposal does noet exist");
            
            let proposal = self.proposals.get(&proposal_id).unwrap();

            proposal.vote_start

        }

        #[ink(message)]
        pub fn proposal_votes(&self, proposal_id: OperationId) -> (u32,u32,u32) {
            assert!(self.votes.contains(&proposal_id), "Proposal does noet exist");
            
            let proposal = self.votes.get(&proposal_id).unwrap();

            (proposal.votes_against, proposal.votes_for, proposal.votes_abstain)

        }

        #[ink(message)]
        pub fn state(&self, proposal_id: OperationId) -> ProposalState {
            assert!(self.proposals.contains(&proposal_id), "Proposal does noet exist");
            let proposal = self.proposals.get(&proposal_id).unwrap();

            if proposal.executed {
                return ProposalState::Executed
            }

            if proposal.canceled {
                return ProposalState::Canceled
            }

            if proposal.vote_start > self.env().block_timestamp() {
                return ProposalState::Pending
            }

            if proposal.vote_end > self.env().block_timestamp() {
                return ProposalState::Active
            }

            let vote = self.votes.get(&proposal_id).unwrap();
            if vote.votes_for > vote.votes_against {
                return ProposalState::Succeeded
            }
            
            return ProposalState::Defeated
        }

        #[ink(message)]
        pub fn voting_delay(&self) -> Timestamp {
            ink_env::debug_println!("voting_delay()");
            self.voting_delay
        }

        #[ink(message)]
        pub fn voting_period(&self) -> Timestamp {
            ink_env::debug_println!("voting_period");
            self.voting_period
        }

        #[ink(message)]
        pub fn hash_proposal(&self, transaction: Transaction, description: String) -> OperationId {
            let description_hash = self._hash_description(description);
            self._hash_proposal(transaction, description_hash)
        }

        
        /// ERC721Votes read functions
        #[ink(message)] 
        pub fn get_past_votes(&self, account: AccountId, block: BlockNumber) -> u32 {
            self._get_votes(account, Some(block))
        }

        #[ink(message)]
        pub fn get_votes(&self, account: AccountId) -> u32 {
            self._get_votes(account, None)
        }

        #[ink(message)]
        pub fn get_nft_price(&self) -> Balance {
            self.price
        }

        #[ink(message)]
        pub fn get_nft(&self, account: AccountId) -> Result<(CollectionId, NftId), GovernorError>  {
            if !self.owners_nft.contains(&account) {
                return Err(GovernorError::NotOwner)    
             }

             let nft_id = self.owners_nft.get(&account).unwrap();
             Ok((self.collection_id.unwrap(),nft_id))
        }

        #[ink(message)]
        pub fn list_owners(&self) -> Vec<(AccountId,NftId,u32)> {
            let mut result : Vec<(AccountId,NftId,u32)> = Vec::new();

            for owner in self.owners.iter() {
                let nft_id = self.owners_nft.get(owner).unwrap();
                let votes = self._get_votes(*owner, None);

                result.push((*owner,nft_id,votes));
            }

            result
        }

        #[ink(message)]
        pub fn list_proposals(&self) -> Vec<(OperationId,ProposalVote)> {
            let mut result: Vec<(OperationId,ProposalVote)> = Vec::new();

            for proposal in self.proposal_ids.iter() {
                let proposal_vote = self.votes.get(proposal).unwrap();
                result.push((*proposal, proposal_vote));
            }

            result
        }

        //////////////////////////////
        /// Governor write functions
        /// 
        #[ink(message)]
        pub fn create_collection(&mut self) -> Result<(), GovernorError> {
            self._create_collection()
        }



        #[ink(message)]
        pub fn cast_vote(
            &mut self, 
            proposal_id: OperationId,
            vote: VoteType,
        ) -> Result<(),GovernorError> {
            self._cast_vote(proposal_id, vote)
        }

        #[ink(message)]
        pub fn execute(&mut self, proposal_id: OperationId) -> Result<(), GovernorError> {
            self._execute(proposal_id)
        }

        #[ink(message)]
        pub fn propose(
            &mut self, 
            transaction: Transaction, 
            description: String
        ) -> Result<OperationId, GovernorError>  {

            let caller = self.env().caller();
            self._has_required_nft(caller)?;


            ink_env::debug_println!("propose(caller={:?}, Transaction={:?}, description={:?})",caller,transaction,description);
            
            let description_hash = self._hash_description(description.clone());
            let proposal_id = self._hash_proposal(transaction.clone(), description_hash);

            // is this a new proposal
            if self.proposals.contains(&proposal_id) {
                return Err(GovernorError::ProposalAlreadyExists)
            }


            let proposal = ProposalCore {
                vote_start: self.env().block_timestamp() + self.voting_delay,
                vote_end: self.env().block_timestamp() + self.voting_delay + self.voting_period,
                executed: false,
                canceled: false
            };

            self.proposals.insert(&proposal_id, &proposal);
            self.votes.insert(&proposal_id, &ProposalVote::default());

            self.proposal_ids.push(proposal_id);

            self
            ._emit_proposal_created(
                proposal_id,
                self.env().caller(),
                transaction,
                description,
                proposal.vote_start,
                proposal.vote_end
            );

            Ok(proposal_id)
        }

        #[ink(message)]
        pub fn delegate(
            &mut self,
            delegate: AccountId,
        ) -> Result<(),GovernorError> {

            let caller = self.env().caller();
            self._has_required_nft(caller)?;

            let old_delegate = self._get_delegate(caller);

            let current_block = self.env().block_number();
            self.delegations.insert(&current_block, &(caller,delegate));

            if !self.delegation_blocks.contains(&current_block) {
                self.delegation_blocks.push(current_block);
            }
        
            self._emit_delegate_changed(caller, delegate, old_delegate);
            self._emit_delegate_votes_changed(old_delegate);
            self._emit_delegate_votes_changed(delegate);

           Ok(())
        }

        //////////////////////////////
        /// Governor payable functions
        /// 

        #[ink(message,payable)]
        pub fn become_member(
            &mut self
         ) -> Result<(),GovernorError> {
            let caller = self.env().caller();
            
            if self.env().transferred_value() < self.price {
                return Err(GovernorError::InsufficientAmount)
            }

            if self.owners_nft.contains(&caller) {
                return Err(GovernorError::AlreadyOwner)
            }

            let metadata = "ipfs://ipfs/QmeeCx81m6RVjmzbHjdeHABa7ksVPymwvXRWSuXSnvpoYG";

            let nft_id : u32 = self.owners.len().try_into().unwrap();

            Rmrk::mint_nft(
                caller,
                nft_id,
                self.collection_id.unwrap(),
                None,
                None,
                metadata.into(),
                false,
                None
            ).map_err(|_| GovernorError::MintFailed)?;


            self.owners.push(caller);
            self.owners_nft.insert(&caller, &nft_id);
            self.owners_lvl.insert(&caller,&0);

            self._evolve_owner(caller)?;

            Ok(())
         }
        
    }

}
