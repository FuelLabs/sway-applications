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
use utils::{identities_equal, sender_identity};

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
    option::*,
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
    meta_data: StorageMap<u64, Option<MetaData>>,

    /// Stores a b256 hash of the (owner, operator) and stores whether the
    /// operator is allowed to transfer ALL tokens on the owner's behalf 
    operator_approval: StorageMap<b256, bool>,

    /// TODO: Use a Vec here to support multiple ownerships
    /// Maintains the token ids owned by the specified identity
    owners: StorageMap<Identity, u64>,

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
    /// - The NFT contract does not have access control set
    /// - The Identity is not the access control address
    fn allow_mint(minter: Identity, allow: bool) {
        require(storage.access_control, AccessError::AccessControlNotSet);
        
        // Ensure that the sender is allowed to add indetites to the approved list
        let sender: Identity = sender_identity();
        require(
            identities_equal(sender, storage.access_control_address), 
            AccessError::SenderCannotSetAccessControl
        );

        // Add the provided identity to the list of identities that are approved to mint
        storage.allowed_minters.insert(minter, allow);
    }

    /// Gives approval to the 'to' Identity to transfer the specified token
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The token does not exist
    /// - The appover is the owner
    /// - The sender is not the owner
    fn approve(to: Identity, token_id: u64, approve: bool) {
        let meta_data: Option<MetaData> = storage.meta_data.get(token_id);
        require(meta_data.is_some(), InputError::TokenDoesNotExist);

        let mut meta_data: MetaData = meta_data.unwrap();
        let owner = meta_data.owner;
        require(
            !identities_equal(meta_data.owner, to), 
            ApprovalError::ApproverCannotBeOwner
        );

        // Ensure that the sender is the owner of the token to be approved
        let sender = sender_identity();
        require(
            identities_equal(owner, sender), 
            AccessError::SenderNotOwner
        );

        match approve {
            true => {
                // Approve this identity for this token
                meta_data.approved = Option::Some(to);
                storage.meta_data.insert(token_id, Option::Some(meta_data));
            },
            false => {
                // Remove approval
                meta_data.approved = Option::None();
                storage.meta_data.insert(token_id, Option::Some(meta_data));
            }
        }

        log(ApprovalEvent{owner: sender, approved: to, token_id});
    }

    /// Returns the balance of the specified owner
    fn balance_of(owner: Identity) -> u64 {
        storage.balances.get(owner)
    }

    /// Burns the specified token
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The token id does not exist
    /// - The sender is not the owner
    fn burn(token_id: u64) {
        // Ensure this is a valid token that has already been minted and exists
        let meta_data: Option<MetaData> = storage.meta_data.get(token_id);
        require(meta_data.is_some(), InputError::TokenDoesNotExist);

        // Ensure the sender owns the token that is provided
        let sender = sender_identity();
        let meta_data: MetaData = meta_data.unwrap();
        require(
            identities_equal(meta_data.owner, sender), 
            AccessError::SenderNotOwner
        );

        // Burn this token
        storage.meta_data.insert(token_id, Option::None());

        // Reduce the balance of tokens for the owner
        storage.balances.insert(sender, storage.balances.get(sender) - 1);

        // NOTE: Until we have a vec get_tokens will now return
        //       owning nothing, even if mutliple tokens are owned
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
    fn constructor(owner: Identity, access_control: bool, token_supply: u64) {
        require(storage.token_supply == 0, InitError::CannotReinitialize);
        require(token_supply != 0, InputError::TokenSupplyCannotBeZero);

        storage.access_control_address = owner;
        storage.access_control = access_control;
        storage.token_supply = token_supply;
    }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
    /// Returns the approved address
    // fn get_approved(token_id: u64) -> Option<Identity> {
    //     let meta_data: Option<MetaData> = storage.meta_data.get(token_id);

    //     match meta_data {
    //         Option::Some(MetaData) => {
    //             let meta_data: MetaData = meta_data.unwrap();
    //             let approved = meta_data.approved;

    //             match approved {
    //                 Option::Some(Identity) => Option::Some(approved.unwrap()),
    //                 Option::None(Identity) => Option::None(),
    //             }
    //         },
    //         Option::None(MetaData) => Option::None(),
    //     }
    // }

    /// Returns the tokens owned by the address
    fn get_tokens(identity: Identity) -> u64 {
        storage.owners.get(identity)
    }

    /// Returns the total supply for the NFT contract
    fn get_total_supply() -> u64 {
        storage.token_supply
    }

    /// Returns whether the address is approved for all tokens
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool {
        storage.operator_approval.get(sha256(owner, operator))
    }

    /// Mints an NFT
    ///
    /// # Panics
    /// 
    /// The function will panic when:
    /// - The amount is set to 0
    /// - More NFTs than supply is minted
    /// - The sender is not approved to mint
    /// - The sender sent the wrong asset
    /// - The sender did not pay enough tokens
    fn mint(to: Identity, amount: u64) {
        require(
            storage.token_supply >= (storage.token_count + amount), 
            InputError::NotEnoughTokensToMint
        );

        let sender = sender_identity();

        // Ensure that the sender is on the approved mint list
        require(
            !storage.access_control || 
            storage.allowed_minters.get(sender), 
            AccessError::SenderDoesNotHaveAccessControl
        );

        // Mint as many tokens as the sender has paid for
        let mut index = 0;
        while index < amount {
            // Increment the token count
            storage.token_count = storage.token_count + 1;

            // Create the metadata for this new token with the owner 
            let meta_data = MetaData {
                owner: to, approved: Option::None()
            };
            storage.meta_data.insert(storage.token_count, Option::Some(meta_data));
            storage.owners.insert(to, storage.token_count);
            
            // Increase the balance of the new owner
            storage.balances.insert(to, storage.balances.get(to) + 1);

            // and the number of tokens minted in this transaction
            index = index + 1;

            // TODO: When Vec is available, log a Vec of tokens instead
            let token_id = storage.token_count;
            log(MintEvent{owner: to, token_id});
        }
    }

    // Uncomment when https://github.com/FuelLabs/fuels-rs/issues/375 is resolved
    /// Returns the owner of a given token id
    // fn owner_of(token_id: u64) -> Option<Identity> {
    //     let meta_data: Option<MetaData> = storage.meta_data.get(token_id);

    //     match meta_data {
    //         Option::Some(MetaData) => {
    //             let meta_data: MetaData = meta_data.unwrap();
    //             Option::Some(meta_data.owner)
    //         },
    //         Option::None(MetaData) => Option::None(),
    //     }
    // }

    /// Gives the operator identity approval to transfer any tokens owned by 
    /// the owner identity. This can be dangerous.
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The sender is not the owner
    fn set_approval_for_all(owner: Identity, operator: Identity, allow: bool) {
        let hash = sha256(owner, operator);

        let sender = sender_identity();
        require(identities_equal(owner, sender), AccessError::SenderNotOwner);

        // Set the identity to have approval on all tokens owned
        storage.operator_approval.insert(hash, allow);

        log(OperatorEvent{owner, operator});
    }

    /// Transfers ownership from one address to another
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The token does not exist
    /// - The sender is not the owner
    /// - The sender is not approved
    /// - The sender is not an operator for the owner
    fn transfer_from(from: Identity, to: Identity, token_id: u64) {
        // Make sure the token exists
        let meta_data: Option<MetaData> = storage.meta_data.get(token_id);
        require(meta_data.is_some(), InputError::TokenDoesNotExist);

        // Ensure that the sender is either the owner of the tokens, is approved
        // for transfer, or is an operator and the token is owned by the operator
        let sender = sender_identity();
        let mut meta_data: MetaData = meta_data.unwrap();
        let approved: Option<Identity> = meta_data.approved;

        match approved {
            Option::Some(Identity) => {
                // Include approved in the check for permissions
                require(
                    identities_equal(sender, meta_data.owner) ||
                    identities_equal(sender, approved.unwrap()) ||
                    (identities_equal(from, meta_data.owner) && storage.operator_approval.get(sha256(from, sender))), 
                    AccessError::SenderNotOwnerOrApproved
                );
            },
            Option::None(Identity) => {
                // Don't include approved in the check for permissions
                require(
                    identities_equal(sender, meta_data.owner) ||
                    (identities_equal(from, meta_data.owner) && storage.operator_approval.get(sha256(from, sender))), 
                    AccessError::SenderNotOwnerOrApproved
                );
            }
        }

        // Set the new owner of the token and reset the approver
        meta_data.owner = to;
        meta_data.approved = Option::None();
        storage.meta_data.insert(token_id, Option::Some(meta_data));

        // Note: Until Vec is supported, getting the tokens owned by the old owner
        //        will return nothing after transfer
        storage.owners.insert(from, 0);
        storage.owners.insert(to, token_id);

        // Decrease the previous owner's balance of tokens
        storage.balances.insert(from, storage.balances.get(from) - 1);

        // Increase the new owner's balance of tokens
        storage.balances.insert(to, storage.balances.get(to) + 1);

        log(TransferEvent{from, to, token_id});
    }
}
