library utils;

dep data_structures;
dep errors;

use std::{
    auth::{
        AuthError,
        msg_sender,
    },
    block::height,
    hash::sha256,
    storage::{
        get,
        store,
    },
    token::{
        force_transfer_to_contract,
        transfer_to_address,
    },
};

use data_structures::Auction;
use errors::UserError;

/// Calculates the current price of a given auction
pub fn calculate_price(auction: Auction) -> u64 {
    // How much the price will go down by, throughout the auction
    let price_delta = auction.opening_price - auction.reserve_price;
    // How long the auction will last
    let auction_duration = auction.end_time - auction.start_time;
    // This is the amount the price will reduce by per block
    let price_shift = price_delta / auction_duration;

    // Tells us how far we are into the auction (out of the auction_duration)
    let blocks_into_auction = height() - auction.start_time;

    // Cap how far we are into the auction by the auction_duration, so price doesnt go into negative or below endprice
    let blocks_into_auction = if blocks_into_auction > auction_duration {
        auction_duration
    } else {
        blocks_into_auction
    };

    // price_shift * blocks_into_auction tells us how much the price has reduced by now
    auction.opening_price - (price_shift * blocks_into_auction)
}

/// Helper function to compare identities
pub fn eq_identity(id_1: Identity, id_2: Identity) -> bool {
    match id_1 {
        Identity::Address(address1) => {
            match id_2 {
                Identity::Address(address2) => {
                    address1 == address2
                },
                _ => false,
            }
        },
        Identity::ContractId(contract_id_1) => {
            match id_2 {
                Identity::ContractId(contract_id_2) => {
                    contract_id_1 == contract_id_2
                },
                _ => false,
            }
        },
    }
}

/// Helper function to avoid having to repeat this code
pub fn sender_indentity() -> Identity {
    let sender: Result<Identity, AuthError> = msg_sender();
    sender.unwrap()
}

/// Helper function to transfer assets to an identity
pub fn transfer_to_identity(amount: u64, asset_id: ContractId, reciever: Identity) {
    match reciever {
        Identity::Address(address) => {
            transfer_to_address(amount, asset_id, address);
        },
        Identity::ContractId(contractid) => {
            force_transfer_to_contract(amount, asset_id, contractid);
        },
    };
}

/// Validates an auction_id to make sure it corresponds to an auction
pub fn validate_id(id: u64, auction_count: u64) {
    // If the id is greater than the auction count then it's invalid
    require(id != 0, UserError::InvalidAuctionID);
    require(id <= auction_count, UserError::InvalidAuctionID);
}

/// A persistant mapping of K -> Vec<V>
pub struct StorageMapVec<K, V> {}

impl<K, V> StorageMapVec<K, V> {
    /// Appends the value to the end of the vector
    ///
    /// ### Arguments
    ///
    /// * `key` - The key to the vector
    /// * `value` - The item being added to the end of the vector
    ///
    /// ### Examples
    ///
    /// ```sway
    /// use std::storage::StorageMapVec;
    ///
    /// storage {
    ///     map_vec: StorageMapVec<u64, bool> = StorageMapVec {}
    /// }
    ///
    /// fn foo() {
    ///     let five = 5_u64;
    ///     storage.map_vec.push(five, true);
    ///     let retrieved_value = storage.map_vec.get(five).unwrap();
    ///     assert(true == retrieved_value);
    /// }
    /// ```
    #[storage(read, write)]
    pub fn push(self, key: K, value: V) {
        // The length of the vec is stored in the sha256((key, __get_storage_key())) slot
        let k = sha256((key, __get_storage_key()));
        let len = get::<u64>(k);

        // Storing the value at the current length index (if this is the first item, starts off at 0)
        let key = sha256((key, len, __get_storage_key()));
        store::<V>(k, value);

        // Incrementing the length
        store(k, len + 1);
    }

    /// Removes the last element of the vector and returns it, None if empty
    ///
    /// ### Arguments
    ///
    /// * `key` - The key to the vector
    ///
    /// ### Examples
    ///
    /// ```sway
    /// use std::storage::StorageMapVec;
    ///
    /// storage {
    ///     map_vec: StorageMapVec<u64, bool> = StorageMapVec {}
    /// }
    ///
    /// fn foo() {
    ///     let five = 5_u64;
    ///     storage.map_vec.push(five, true);
    ///     let popped_value = storage.map_vec.pop(five).unwrap();
    ///     assert(true == popped_value);
    ///     let none_value = storage.map_vec.pop(five);
    ///     assert(none_value.is_none())
    /// }
    /// ```
    #[storage(read, write)]
    pub fn pop(self, key: K) -> Option<V> {
        // The length of the vec is stored in the sha256((key, __get_storage_key())) slot
        let k = sha256((key, __get_storage_key()));
        let len = get::<u64>(k);
        // if the length is 0, there is no item to pop from the vec
        if len == 0 {
            return Option::None;
        }

        // reduces len by 1, effectively removing the last item in the vec
        store(k, len - 1);

        let key = sha256((key, len - 1, __get_storage_key()));
        Option::Some::<V>(get::<V>(key))
    }

    /// Gets the value in the given index, None if index is out of bounds
    ///
    /// ### Arguments
    ///
    /// * `key` - The key to the vector
    /// * `index` - The index of the vec to retrieve the item from
    ///
    /// ### Examples
    ///
    /// ```sway
    /// use std::storage::StorageMapVec;
    ///
    /// storage {
    ///     map_vec: StorageMapVec<u64, bool> = StorageMapVec {}
    /// }
    ///
    /// fn foo() {
    ///     let five = 5_u64;
    ///     storage.map_vec.push(five, true);
    ///     let retrieved_value = storage.map_vec.get(five, 0).unwrap();
    ///     assert(true == retrieved_value);
    ///     let none_value = storage.map_vec.get(five, 1);
    ///     assert(none_value.is_none())
    /// }
    /// ```
    #[storage(read)]
    pub fn get(self, key: K, index: u64) -> Option<V> {
        // The length of the vec is stored in the sha256((key, __get_storage_key())) slot
        let k = sha256((key, __get_storage_key()));
        let len = get::<u64>(k);
        // if the index is larger or equal to len, there is no item to return
        if len <= index {
            return Option::None;
        }

        let key = sha256((key, index, __get_storage_key()));
        Option::Some::<V>(get::<V>(key))
    }

    /// Returns the length of the vector
    ///
    /// ### Examples
    ///
    /// ```sway
    /// use std::storage::StorageMapVec;
    ///
    /// storage {
    ///     map_vec: StorageMapVec<u64, bool> = StorageMapVec {}
    /// }
    ///
    /// fn foo() {
    ///     assert(0 == storage.map_vec.len(5));
    ///     storage.map_vec.push(5, true);
    ///     assert(1 == storage.map_vec.len(5));
    ///     storage.map_vec.push(5, false);
    ///     assert(2 == storage.map_vec.len(5));
    /// }
    /// ```
    #[storage(read)]
    pub fn len(self, key: K) -> u64 {
        // The length of the vec is stored in the sha256((key, __get_storage_key())) slot
        let k = sha256((key, __get_storage_key()));
        get::<u64>(k)
    }

    /// Checks whether the len is 0 or not
    ///
    /// ### Examples
    ///
    /// ```sway
    /// use std::storage::StorageMapVec;
    ///
    /// storage {
    ///     map_vec: StorageMapVec<u64, bool> = StorageMapVec {}
    /// }
    ///
    /// fn foo() {
    ///     assert(true == storage.map_vec.is_empty(5));
    ///
    ///     storage.map_vec.push(5, true);
    ///
    ///     assert(false == storage.map_vec.is_empty(5));
    ///
    ///     storage.map_vec.clear(5);
    ///
    ///     assert(true == storage.map_vec.is_empty(5));
    /// }
    /// ```
    #[storage(read)]
    pub fn is_empty(self, key: K) -> bool {
        // The length of the vec is stored in the sha256((key, __get_storage_key())) slot
        let k = sha256((key, __get_storage_key()));
        let len = get::<u64>(k);
        len == 0
    }

    /// Sets the len to 0
    ///
    /// ### Examples
    ///
    /// ```sway
    /// use std::storage::StorageMapVec;
    ///
    /// storage {
    ///     map_vec: StorageMapVec<u64, bool> = StorageMapVec {}
    /// }
    ///
    /// fn foo() {
    ///     assert(0 == storage.map_vec.len(5));
    ///     storage.map_vec.push(5, true);
    ///     assert(1 == storage.map_vec.len(5));
    ///     storage.map_vec.clear(5);
    ///     assert(0 == storage.map_vec.len(5));
    /// }
    /// ```
    #[storage(write)]
    pub fn clear(self, key: K) {
        // The length of the vec is stored in the sha256((key, __get_storage_key())) slot
        let k = sha256((key, __get_storage_key()));
        store(k, 0);
    }
}
