contract;

dep errors;
dep events;

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

use errors::{
    AccessError,
    ApprovalError,
    InitError,
    InputError
};

use events::{
    ApprovalEvent,
    BurnEvent,
    MintEvent,
    OperatorEvent,
    TransferEvent
};

abi NFT {
    fn allow_mint(minter: Identity) -> bool;
    fn approve(to: Identity, token_id: u64) -> bool;
    fn balance_of(owner: Identity) -> u64;
    fn burn(token_id: u64) -> bool ;
    fn constructor(owner: Identity, access_control: bool, token_supply: u64, token_price: u64, asset: ContractId) -> bool;
    //fn get_approved(token_id: u64) -> Option<Identity>;
    fn get_tokens(address: Identity) -> u64;
    fn get_total_supply() -> u64;
    fn is_approved_for_all(owner: Identity, operator: Identity) -> bool;
    fn mint(to: Identity, amount: u64) -> bool ;
    //fn owner_of(token_id: u64) -> Option<Identity>;
    fn set_approval_for_all(owner: Identity, operator: Identity) -> bool;
    fn transfer_from(from: Identity, to: Identity, token_id: u64) -> bool;
}

struct MetaData {
    // NFT Metadata
    owner: Identity,
    approved: Identity,
}

storage {
    access_control: bool,
    access_control_address: Identity,
    allowed_minters: StorageMap<Identity, bool>,
    balances: StorageMap<Identity, u64>,
    asset: ContractId,
    meta_data: StorageMap<u64, MetaData>,
    operator_approval: StorageMap<b256, bool>,
    // TODO: Use a Vec here to support multiple ownerships
    owners: StorageMap<Identity, u64>,
    state: u64,
    token_count: u64,
    token_price: u64,
    token_supply: u64,
}

impl NFT for Contract {

    /// Allows access to mint any NFT
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The NFT contract does not have access control set
    /// - The Identity provided is valid
    /// - The Identity has already been allowed access
    /// - The Identity is not the access control address
    fn allow_mint(minter: Identity) -> bool {
        require(storage.state != 0, InitError::NFTNotInitalized);
        require(storage.access_control, AccessError::AccessControlNotSet);

        _require_identity_is_valid(minter);

        require(storage.allowed_minters.get(minter) == false, 
            ApprovalError::AddressAlreadyGivenAccess
        );
        
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = sender.unwrap();
        require(
            _compare_identities(sender, storage.access_control_address), 
            AccessError::SenderCannotSetAccessControl
        );

        storage.allowed_minters.insert(minter, true);

        true
    }

    /// Gives approval to the to address to transfer the specified token
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The Identity provided is valid
    /// - The address has already been approved
    /// - The appover is the owner
    /// - The sender is not the owner
    fn approve(to: Identity, token_id: u64) -> bool {
        require(storage.state != 0, InitError::NFTNotInitalized);

        _require_identity_is_valid(to);

        let mut meta_data = storage.meta_data.get(token_id);
        require(
            !_compare_identities(meta_data.approved, to), 
            ApprovalError::AddressAlreadyGivenApproval
        );
        require(
            !_compare_identities(meta_data.owner, to), 
            ApprovalError::ApproverCannotBeOwner
        );

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = sender.unwrap();
        require(
            _compare_identities(meta_data.owner, sender), 
            AccessError::SenderNotOwner
        );

        meta_data.approved = to;
        storage.meta_data.insert(token_id, meta_data);

        log(ApprovalEvent{owner: sender, approved: to, token_id});
        true
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
    fn burn(token_id: u64) -> bool {
        require(storage.state != 0, InitError::NFTNotInitalized);

        let empty_identity: Identity = Identity::Address(~Address::from(NATIVE_ASSET_ID));
        let mut meta_data: MetaData = storage.meta_data.get(token_id);
        require(
            !_compare_identities(meta_data.owner, empty_identity), 
            InputError::TokenDoesNotExist
        );

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = sender.unwrap();
        require(
            _compare_identities(meta_data.owner, sender), 
            AccessError::SenderNotOwner
        );

        meta_data.owner = empty_identity;
        meta_data.approved = empty_identity;
        storage.meta_data.insert(token_id, meta_data);

        let balance = storage.balances.get(sender);
        storage.balances.insert(sender, balance - 1);

        // NOTE: Until we have a vec get_tokens will now return
        //       owning nothing, even if mutliple tokens are owned
        storage.owners.insert(sender, 0);

        log(BurnEvent{owner: sender, token_id});

        true
    }

    /// Constructor for the NFT
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The constructor has already been called
    /// - The token count is 0
    /// - The owner is not a valid identity
    fn constructor(owner: Identity, access_control: bool, token_supply: u64, token_price: u64, asset: ContractId) -> bool {
        require(storage.state == 0, InitError::CannotReinitialize);
        require(token_supply != 0, InputError::TokenSupplyCannotBeZero);

        _require_identity_is_valid(owner);

        storage.access_control_address = owner;
        storage.access_control = access_control;
        storage.token_supply = token_supply;
        storage.token_price = token_price;
        storage.asset = asset;
        storage.state = 1;

        true
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

    //     let meta_data: MetaData = storage.meta_data.get(token_id);
    //     Option::Some(meta_data.approved)
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

        let hash: b256 = _get_owner_operator_hash(owner, operator);
        storage.operator_approval.get(hash)
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
    fn mint(to: Identity, amount: u64) -> bool {
        require(storage.state != 0, InitError::NFTNotInitalized);
        require(amount != 0, InputError::MintAmountCannotBeZero);
        require(
            storage.token_supply >= (storage.token_count + amount), 
            InputError::NotEnoughTokensToMint
        );

        _require_identity_is_valid(to);

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = sender.unwrap();

        require(
            !storage.access_control || 
            (storage.access_control && storage.allowed_minters.get(sender)), 
            AccessError::SenderDoesNotHaveAccessControl
        );

        let cost: u64 = storage.token_price * amount;
        require(msg_asset_id() == storage.asset, InputError::IncorrectAssetId);
        require(msg_amount() == cost, InputError::IncorrectAssetAmount);

        let mut i = 0;
        while i < amount {
            let token_id: u64 = storage.token_count + 1;
            let empty_identity: Identity = Identity::Address(~Address::from(NATIVE_ASSET_ID));

            let meta_data: MetaData = MetaData {
                owner: to, approved: empty_identity
            };
            storage.meta_data.insert(token_id, meta_data);
            
            let mut balance = storage.balances.get(to);
            storage.balances.insert(to, balance + 1);

            storage.owners.insert(to, token_id);

            storage.token_count = storage.token_count + 1;
            i = i + 1;

            log(MintEvent{owner: to, token_id});
        }

        true
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

    //     let meta_data: MetaData = storage.meta_data.get(token_id);
    //     Option::Some(meta_data.owner)
    // }

    /// Gives operator approval to the to address to transfer
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The operator address provided not a valid identity
    /// - The address has already been approved
    /// - The sender is not the owner
    fn set_approval_for_all(owner: Identity, operator: Identity) -> bool {
        require(storage.state != 0, InitError::NFTNotInitalized);
        
        _require_identity_is_valid(operator);

        let hash: b256 = _get_owner_operator_hash(owner, operator);
        require(!storage.operator_approval.get(hash), ApprovalError::AddressAlreadyGivenApproval);

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = sender.unwrap();

        require(_compare_identities(owner, sender), AccessError::SenderNotOwner);

        storage.operator_approval.insert(hash, true);
        log(OperatorEvent{owner, operator});
        true
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
    fn transfer_from(from: Identity, to: Identity, token_id: u64) -> bool {
        require(storage.state != 0, InitError::NFTNotInitalized);
        
        _require_identity_is_valid(to);

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Identity = sender.unwrap();

        let mut meta_data: MetaData = storage.meta_data.get(token_id);
        let hash: b256 = _get_owner_operator_hash(from, sender);

        require(
            _compare_identities(sender, meta_data.owner) ||
            _compare_identities(sender, meta_data.approved) ||
            storage.operator_approval.get(hash), 
            AccessError::SenderNotOwnerOrApproved
        );

        let empty_identity: Identity = Identity::Address(~Address::from(NATIVE_ASSET_ID));

        meta_data.owner = to;
        meta_data.approved = empty_identity;
        storage.meta_data.insert(token_id, meta_data);

        let mut balance_from = storage.balances.get(from);
        storage.balances.insert(from, balance_from - 1);

        let mut balance_to = storage.balances.get(to);
        storage.balances.insert(to, balance_to + 1);

        storage.owners.insert(from, 0);
        storage.owners.insert(to, token_id);

        log(TransferEvent{from, to, token_id});

        true
    }
}

// This function will take two identities and return true if they are the same
fn _compare_identities(identity1: Identity, identity2: Identity) -> bool {
    match identity1 {
        Identity::Address(identity1) => {
            match identity2 {
                Identity::Address(identity2) => {
                    identity1.value == identity2.value
                },
                _ => {
                    false
                }
            }
        },
        Identity::ContractId(identity1) => {
            match identity2 {
                Identity::ContractId(identity2) => {
                    identity1.value == identity2.value
                },
                _ => {
                    false
                }
            }
        }
    }
}

// This function returns the hash value of the two given idenity's b256 values
fn _get_owner_operator_hash(owner: Identity, operator: Identity) -> b256 {
    match owner {
        Identity::Address(owner) => {
            match operator {
                Identity::Address(operator) => {
                    sha256(owner.value, operator.value)
                },
                Identity::ContractId(operator) => {
                    sha256(owner.value, operator.value)
                }
            }
        },
        Identity::ContractId(owner) => {
            match operator {
                Identity::Address(operator) => {
                    sha256(owner.value, operator.value)
                },
                Identity::ContractId(operator) => {
                    sha256(owner.value, operator.value)
                }
            }
        },
    }
}

// This function will panic if the given Identity points to the zero value
fn _require_identity_is_valid(entity: Identity) {
    match entity {
        Identity::Address(entity) => {
            require(entity != ~Address::from(NATIVE_ASSET_ID), 
                InputError::InputAddressCannotBeZero);
        },
        Identity::ContractId(entity) => {
            require(entity != ~ContractId::from(NATIVE_ASSET_ID), 
                InputError::InputAddressCannotBeZero);
        },
    };
}
