contract;

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
    result::*,
    revert::revert,
    storage::StorageMap,
};

use events::{
    ApprovalEvent,
    BurnEvent,
    MintEvent,
    OperatorEvent,
    TransferEvent
};

abi NFT {
    fn allow_mint(minter: Address) -> bool;
    fn approve(to: Address, token_id: u64) -> bool;
    fn balance_of(owner: Address) -> u64;
    fn burn(token_id: u64) -> bool ;
    fn constructor(owner: Address, access_control: bool, token_supply: u64, token_price: u64, asset: ContractId) -> bool;
    fn get_approved(token_id: u64) -> Address;
    fn get_tokens(address: Address) -> u64;
    fn get_total_supply() -> u64;
    fn is_approved_for_all(owner: Address, operator: Address) -> bool;
    fn mint(to: Address, amount: u64) -> bool ;
    fn owner_of(token_id: u64) -> Address;
    fn set_approval_for_all(owner: Address, operator: Address) -> bool;
    fn transfer_from(from: Address, to: Address, token_id: u64) -> bool;
}

enum Error {
    AccessControlNotSet: (),
    AddressAlreadyGivenAccess: (),
    AddressAlreadyGivenApproval: (),
    ApproverCannotBeOwner: (),
    CannotReinitialize: (),
    IncorrectAssetAmount: (),
    IncorrectAssetId: (),
    InputAddressCannotBeZero: (),
    MintAmountCannotBeZero: (),
    NFTNotInitalized: (),
    NotEnoughTokensToMint: (),
    SenderCannotSetAccessControl: (),
    SenderDoesNotHaveAccessControl: (),
    SenderNotOwner: (),
    SenderNotOwnerOrApproved: (),
    TokenDoesNotExist: (),
    TokenSupplyCannotBeZero: (),
}

struct MetaData {
    // NFT Metadata
    owner: Address,
    approved: Address,
}

storage {
    access_control: bool,
    access_control_address: Address,
    allowed_minters: StorageMap<Address, bool>,
    balances: StorageMap<Address, u64>,
    asset: ContractId,
    meta_data: StorageMap<u64, MetaData>,
    operator_approval: StorageMap<b256, bool>,
    owners: StorageMap<Address, u64>,
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
    /// - The address provided is the 0 address
    /// - The address has already been allowed access
    /// - The address is not the access control address
    fn allow_mint(minter: Address) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);
        require(storage.access_control, Error::AccessControlNotSet);
        require(minter.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);
        require(storage.allowed_minters.get(minter) == false, Error::AddressAlreadyGivenAccess);
        
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(address) => {
                address
            },
            _ => {
                revert(0);
            },
        };
        require(storage.access_control_address == sender, Error::SenderCannotSetAccessControl);

        storage.allowed_minters.insert(minter, true);

        true
    }

    /// Gives approval to the to address to transfer the specified token
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The address provided is the 0 address
    /// - The address has already been approved
    /// - The appover is the owner
    /// - The sender is not the owner
    fn approve(to: Address, token_id: u64) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);
        require(to.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);

        let mut meta_data = storage.meta_data.get(token_id);
        require(meta_data.approved != to, Error::AddressAlreadyGivenApproval);
        require(meta_data.owner != to, Error::ApproverCannotBeOwner);

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(address) => {
                address
            },
            _ => {
                revert(0);
            },
        };
        require(meta_data.owner == sender, Error::SenderNotOwner);

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
    fn balance_of(owner: Address) -> u64 {
        require(storage.state != 0, Error::NFTNotInitalized);
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
        require(storage.state != 0, Error::NFTNotInitalized);

        let mut meta_data: MetaData = storage.meta_data.get(token_id);
        require(meta_data.owner.value != NATIVE_ASSET_ID, Error::TokenDoesNotExist);

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(address) => {
                address
            },
            _ => {
                revert(0);
            },
        };
        require(meta_data.owner == sender, Error::SenderNotOwner);

        meta_data.owner = ~Address::from(NATIVE_ASSET_ID);
        meta_data.approved = ~Address::from(NATIVE_ASSET_ID);
        storage.meta_data.insert(token_id, meta_data);

        let balance = storage.balances.get(sender);
        storage.balances.insert(sender, balance - 1);

        // NOTE: Until we have a vec get_tokens will now return not
        //       owning anything, even if mutliple tokens are owned
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
    /// - The owner is the 0 address
    /// - The token count is 0
    fn constructor(owner: Address, access_control: bool, token_supply: u64, token_price: u64, asset: ContractId) -> bool {
        require(storage.state == 0, Error::CannotReinitialize);
        require(owner.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);
        require(token_supply != 0, Error::TokenSupplyCannotBeZero);

        storage.access_control_address = owner;
        storage.access_control = access_control;
        storage.token_supply = token_supply;
        storage.token_price = token_price;
        storage.asset = asset;
        storage.state = 1;

        true
    }

    /// Returns the approved address
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    fn get_approved(token_id: u64) -> Address {
        require(storage.state != 0, Error::NFTNotInitalized);

        let meta_data: MetaData = storage.meta_data.get(token_id);
        meta_data.approved
    }

    /// Returns the tokens owned by the address
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been intialized
    fn get_tokens(address: Address) -> u64 {
        require(storage.state != 0, Error::NFTNotInitalized);
        storage.owners.get(address)
    }

    /// Returns the total supply for the NFT contract
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    fn get_total_supply() -> u64 {
        require(storage.state != 0, Error::NFTNotInitalized);
        storage.token_supply
    }

    /// Returns whether the address is approved for all tokens
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    fn is_approved_for_all(owner: Address, operator: Address) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);

        let hash: b256 = sha256(owner.value, operator.value);
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
    /// - The sender is not approved to mint
    /// - The sender sent the wrong asset
    /// - The sender did not pay enough tokens
    fn mint(to: Address, amount: u64) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);
        require(amount != 0, Error::MintAmountCannotBeZero);
        require(storage.token_supply >= (storage.token_count + amount), Error::NotEnoughTokensToMint);

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(address) => {
                address
            },
            _ => {
                revert(0);
            },
        };

        require(
            !storage.access_control 
            || (storage.access_control && storage.allowed_minters.get(sender)), 
            Error::SenderDoesNotHaveAccessControl
            );

        let cost: u64 = storage.token_price * amount;
        require(msg_asset_id() == storage.asset, Error::IncorrectAssetId);
        require(msg_amount() == cost, Error::IncorrectAssetAmount);

        let mut i = 0;
        while i < amount {
            let token_id: u64 = storage.token_count + 1;

            let meta_data: MetaData = MetaData {
                owner: to, approved: ~Address::from(NATIVE_ASSET_ID)
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

    /// Returns the owner of a given token id
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    fn owner_of(token_id: u64) -> Address {
        require(storage.state != 0, Error::NFTNotInitalized);

        let meta_data: MetaData = storage.meta_data.get(token_id);
        meta_data.owner
    }

    /// Gives operator approval to the to address to transfer
    ///
    /// # Panics
    ///
    /// The function will panic when:
    /// - The NFT contract has not been initalized
    /// - The operator address provided is the 0 address
    /// - The address has already been approved
    /// - The sender is not the owner
    fn set_approval_for_all(owner: Address, operator: Address) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);
        require(operator.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);

        let hash: b256 = sha256(owner.value, operator.value);
        require(!storage.operator_approval.get(hash), Error::AddressAlreadyGivenApproval);

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(address) => {
                address
            },
            _ => {
                revert(0);
            },
        };

        require(owner == sender, Error::SenderNotOwner);

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
    /// - The to address provided is the 0 address
    /// - The sender is not the owner
    /// - The sender is not approved
    /// - The sender is not an operator for the owner
    fn transfer_from(from: Address, to: Address, token_id: u64) -> bool {
        require(storage.state != 0, Error::NFTNotInitalized);
        require(to.value != NATIVE_ASSET_ID, Error::InputAddressCannotBeZero);
        
        let mut meta_data: MetaData = storage.meta_data.get(token_id);
        let hash: b256 = sha256(from.value, address.value);

        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(address) => {
                    address
            },
            _ => {
                revert(0);
            },
        };

        require(
            (sender == meta_data.owner 
            || sender == meta_data.approved 
            || storage.operator_approval.get(hash)), 
            Error::SenderNotOwnerOrApproved
            );

        meta_data.owner = to;
        meta_data.approved = ~Address::from(NATIVE_ASSET_ID);
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
