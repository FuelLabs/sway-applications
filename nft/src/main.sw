contract;

dep abi;
dep data_structures;
dep errors;
dep events;
dep utils;

use abi::NFT;
use data_structures::MetaData;
use errors::{AccessError, ApprovalError, InitError, InputError};
use events::{ApprovalEvent, BurnEvent, MintEvent, OperatorEvent, TransferEvent};
use utils::{identities_equal, sender_identity, validate_identity};

use std::{
    address::Address,
    assert::require,
    chain::auth::{AuthError, msg_sender},
    constants::NATIVE_ASSET_ID,
    context::{call_frames::{contract_id, msg_asset_id}, msg_amount, this_balance},
    contract_id::ContractId,
    hash::sha256,
    identity::Identity,
    logging::log,
    option::Option,
    result::*,
    revert::revert,
    storage::StorageMap,
};

storage {
    /// Determines if there is a whitelist to mint. Only set on the
    /// initalization of the contract
    access_control: bool,

    /// The identity that has permission to add identities to the whitelist
    access_control_address: Identity,

    /// Stores the identities that are on the whitelist
    allowed_minters: StorageMap<Identity, bool>,

    /// Maintains the number of tokens owned by each of the identities
    balances: StorageMap<Identity, u64>,

    /// The metadata for each token based on token id
    meta_data: StorageMap<u64, MetaData>,

    /// Stores a b256 hash of the (owner, operator) and stores whether the
    /// operator is allowed to transfer ALL tokens on the owner's behalf 
    operator_approval: StorageMap<b256, bool>,

    /// TODO: Use a Vec here to support multiple ownerships
    /// Maintains the token ids owned by the specified identity
    owners: StorageMap<Identity, u64>,

    /// The state of the contract. Can either be initalized or not
    state: u64,

    /// The total number of tokens that have been minted
    token_count: u64,

    /// The total supply tokens that can be minted. Can only be set
    /// on the initalization of the contract
    token_supply: u64,
}

impl NFT for Contract {

    /// Allows or revokes permissions for an identity to mint
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The NFT contract does not have access control set
    /// - The Identity provided is invalid
    /// - The Identity has already been allowed access
    /// - The Identity is not the access control address
    fn allow_mint(minter: Identity, allow: bool) {
        require(storage.state != 0, InitError::NFTNotInitalized);
        require(storage.access_control, AccessError::AccessControlNotSet);

        validate_identity(minter);

        require(storage.allowed_minters.get(minter) == false, 
            ApprovalError::AddressAlreadyGivenAccess
        );
        
        /// Ensure that the sender is allowed to add indetites to the approved list
        let sender: Identity = sender_identity();
        require(
            identities_equal(sender, storage.access_control_address), 
            AccessError::SenderCannotSetAccessControl
        );

        /// Add the provided identity to the list of identities that are approved to mint
        storage.allowed_minters.insert(minter, allow);
    }

    /// Gives approval to the 'to' Identity to transfer the specified token
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The Identity provided is valid
    /// - The address has already been approved
    /// - The appover is the owner
    /// - The sender is not the owner
    fn approve(to: Identity, token_id: u64) {
        require(storage.state != 0, InitError::NFTNotInitalized);

        validate_identity(to);

        /// Ensure that the identity being approved is unique
        let mut meta_data = storage.meta_data.get(token_id);
        require(
            !identities_equal(meta_data.approved, to), 
            ApprovalError::AddressAlreadyGivenApproval
        );
        require(
            !identities_equal(meta_data.owner, to), 
            ApprovalError::ApproverCannotBeOwner
        );

        // Ensure that the sender is the owner of the token to be approved
        let sender: Identity = sender_identity();
        require(
            identities_equal(meta_data.owner, sender), 
            AccessError::SenderNotOwner
        );

        /// Approve this identity for this token
        meta_data.approved = to;
        storage.meta_data.insert(token_id, meta_data);

        log(ApprovalEvent{owner: sender, approved: to, token_id});
    }

    /// Returns the balance of the specified owner
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    fn balance_of(owner: Identity) -> u64 {
        require(storage.state != 0, InitError::NFTNotInitalized);
        storage.balances.get(owner)
    }

    /// Burns the specified token
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The token id does not exist
    /// - The sender is not the owner
    fn burn(token_id: u64) {
        require(storage.state != 0, InitError::NFTNotInitalized);

        /// Ensure this is a valid token that has already been minted and exists
        let empty_identity: Identity = Identity::Address(~Address::from(NATIVE_ASSET_ID));
        let mut meta_data: MetaData = storage.meta_data.get(token_id);
        require(
            !identities_equal(meta_data.owner, empty_identity), 
            InputError::TokenDoesNotExist
        );

        /// Ensure the sender owns the token that is provided
        let sender: Identity = sender_identity();
        require(
            identities_equal(meta_data.owner, sender), 
            AccessError::SenderNotOwner
        );

        /// Set the owner and approvers of the token to the 0 address
        meta_data.owner = empty_identity;
        meta_data.approved = empty_identity;
        storage.meta_data.insert(token_id, meta_data);

        /// Reduce the balance of tokens for the owner
        let balance = storage.balances.get(sender);
        storage.balances.insert(sender, balance - 1);

        /// NOTE: Until we have a vec get_tokens will now return
        ///       owning nothing, even if mutliple tokens are owned
        storage.owners.insert(sender, 0);

        log(BurnEvent{owner: sender, token_id});
    }

    /// Constructor for the NFT
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has already been called
    /// - The token count is 0
    /// - The owner is not a valid identity
    fn constructor(owner: Identity, access_control: bool, token_supply: u64) {
        require(storage.state == 0, InitError::CannotReinitialize);
        require(token_supply != 0, InputError::TokenSupplyCannotBeZero);
        validate_identity(owner);

        storage.access_control_address = owner;
        storage.access_control = access_control;
        storage.token_supply = token_supply;
        storage.state = 1;
    }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
    /// Returns the approved address
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    // fn get_approved(token_id: u64) -> Option<Identity> {
    //     require(storage.state != 0, InitError::NFTNotInitalized);

    //     let meta_data = storage.meta_data.get(token_id);
    //     let entity = meta_data.approved;

    //     let address = match entity {
    //         Identity::Address(entity) => entity.value,
    //         Identity::ContractId(entity) => entity.value,
    //     };
        
    //     if (address != NATIVE_ASSET_ID) {
    //         Option::Some(meta_data.approved)
    //     } else {
    //         Option::None()
    //     }
    // }

    /// Returns the tokens owned by the address
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been intialized
    fn get_tokens(identity: Identity) -> u64 {
        require(storage.state != 0, InitError::NFTNotInitalized);
        storage.owners.get(identity)
    }

    /// Returns the total supply for the NFT contract
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    fn get_total_supply() -> u64 {
        require(storage.state != 0, InitError::NFTNotInitalized);
        storage.token_supply
    }

    /// Returns whether the address is approved for all tokens
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool {
        require(storage.state != 0, InitError::NFTNotInitalized);
        storage.operator_approval.get(sha256(owner, operator))
    }

    /// Mints an NFT
    ///
    /// # Panics
    /// 
    /// The function will panic when:
    /// - The NFT contract has not been initalized 
    /// - The amount is set to 0
    /// - More NFTs than supply is minted
    /// - The to Identity is not valid
    /// - The sender is not approved to mint
    /// - The sender sent the wrong asset
    /// - The sender did not pay enough tokens
    fn mint(to: Identity, amount: u64) {
        require(storage.state != 0, InitError::NFTNotInitalized);
        require(amount != 0, InputError::MintAmountCannotBeZero);
        require(
            storage.token_supply >= (storage.token_count + amount), 
            InputError::NotEnoughTokensToMint
        );
        validate_identity(to);

        let sender: Identity = sender_identity();

        /// Ensure that the sender is on the approved mint list
        require(
            !storage.access_control || 
            storage.allowed_minters.get(sender), 
            AccessError::SenderDoesNotHaveAccessControl
        );

        /// Mint as many tokens as the sender has paid for
        let mut i = 0;
        while i < amount {
            /// Create a new token id in sequential order
            let token_id: u64 = storage.token_count + 1;
            let empty_identity: Identity = Identity::Address(~Address::from(NATIVE_ASSET_ID));

            /// Create the metadata for this new token with the owner 
            let meta_data: MetaData = MetaData {
                owner: to, approved: empty_identity
            };
            storage.meta_data.insert(token_id, meta_data);
            storage.owners.insert(to, token_id);
            
            /// Increase the balance of the new owner
            let mut balance = storage.balances.get(to);
            storage.balances.insert(to, balance + 1);

            /// Increment the token count and the number of tokens minted in this transaction
            storage.token_count = storage.token_count + 1;
            i = i + 1;

            log(MintEvent{owner: to, token_id});
        }
    }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
    /// Returns the owner of a given token id
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    // fn owner_of(token_id: u64) -> Option<Identity> {
    //     require(storage.state != 0, InitError::NFTNotInitalized);

    //     let meta_data = storage.meta_data.get(token_id);
    //     let entity = meta_data.owner;

    //     let address = match entity {
    //         Identity::Address(entity) => entity.value,
    //         Identity::ContractId(entity) => entity.value,
    //     };
        
    //     if (address != NATIVE_ASSET_ID) {
    //         Option::Some(meta_data.approved)
    //     } else {
    //         Option::None()
    //     }
    // }

    /// Gives the operator identity approval to transfer any tokens owned by 
    /// the owner identity. This can be dangerous.
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The operator address provided not a valid identity
    /// - The address has already been approved
    /// - The sender is not the owner
    fn set_approval_for_all(owner: Identity, operator: Identity) {
        require(storage.state != 0, InitError::NFTNotInitalized);
        validate_identity(operator);

        let hash: b256 = sha256(owner, operator);
        require(!storage.operator_approval.get(hash), ApprovalError::AddressAlreadyGivenApproval);

        let sender: Identity = sender_identity();
        require(identities_equal(owner, sender), AccessError::SenderNotOwner);

        /// Set the identity to have approval on all tokens owned
        storage.operator_approval.insert(hash, true);

        log(OperatorEvent{owner, operator});
    }

    /// Transfers ownership from one address to another
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The to identity provided is not valid
    /// - The sender is not the owner
    /// - The sender is not approved
    /// - The sender is not an operator for the owner
    fn transfer_from(from: Identity, to: Identity, token_id: u64) {
        require(storage.state != 0, InitError::NFTNotInitalized);
        validate_identity(to);

        let sender: Identity = sender_identity();
        let mut meta_data: MetaData = storage.meta_data.get(token_id);
        let empty_identity: Identity = Identity::Address(~Address::from(NATIVE_ASSET_ID));

        /// Ensure that the sender is either the owner of the tokens, is approved
        /// for transfer, or is an operator and the token is owned by the operator
        require(
            identities_equal(sender, meta_data.owner) ||
            identities_equal(sender, meta_data.approved) ||
            (identities_equal(from, meta_data.owner) && storage.operator_approval.get(sha256(from, sender))), 
            AccessError::SenderNotOwnerOrApproved
        );

        /// Set the new owner of the token and reset the approver
        meta_data.owner = to;
        meta_data.approved = empty_identity;
        storage.meta_data.insert(token_id, meta_data);
        /// Note: Until Vec is supported, getting the tokens owned by the old owner
        //        will return nothing after transfer
        storage.owners.insert(from, 0);
        storage.owners.insert(to, token_id);

        /// Decrease the previous owner's balance of tokens
        let mut balance_from = storage.balances.get(from);
        storage.balances.insert(from, balance_from - 1);

        // Increase the new owner's balance of tokens
        let mut balance_to = storage.balances.get(to);
        storage.balances.insert(to, balance_to + 1);

        log(TransferEvent{from, to, token_id});
    }
}
