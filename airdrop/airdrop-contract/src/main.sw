contract;

mod data_structures;
mod errors;
mod events;
mod interface;

use ::data_structures::ClaimState;
use ::errors::{AccessError, InitError, StateError, VerificationError};
use ::events::{ClaimEvent, ClawbackEvent, CreateAirdropEvent};
use ::interface::{AirdropDistributor, Info};
use merkle_proof::binary_merkle_proof::{leaf_digest, verify_proof};
use std::{
    asset::transfer,
    auth::msg_sender,
    block::height,
    call_frames::msg_asset_id,
    context::{
        msg_amount,
        this_balance,
    },
    hash::{
        Hash,
        sha256,
    },
    intrinsics::size_of_val,
};

storage {
    /// The Identity which has the ability to clawback unclaimed coins of an asset.
    admin: Option<Identity> = Option::None,
    /// The asset which is to be distributed.
    asset: Option<AssetId> = Option::None,
    /// Stores the ClaimState of users that have interacted with the Airdrop Distributor contract.
    /// Maps (user => claim)
    claims: StorageMap<Identity, ClaimState> = StorageMap {},
    /// The block at which the claiming period will end.
    end_block: u32 = 0,
    /// The computed merkle root which is to be verified against.
    merkle_root: Option<b256> = Option::None,
    /// The number of leaves in the merkle tree
    number_of_leaves: u64 = 0,
}

impl AirdropDistributor for Contract {
    #[storage(read, write)]
    fn claim(amount: u64, key: u64, proof: Vec<b256>, to: Identity) {
        // The claiming period must be open
        require(
            storage
                .end_block
                .read() > height(),
            StateError::ClaimPeriodNotActive,
        );

        // Users cannot claim twice
        let sender = msg_sender().unwrap();
        require(
            storage
                .claims
                .get(sender)
                .try_read()
                .unwrap_or(ClaimState::Unclaimed) == ClaimState::Unclaimed,
            AccessError::UserAlreadyClaimed,
        );

        // There must be enough coins left in the contract
        let asset = storage.asset.read().unwrap();
        require(this_balance(asset) >= amount, AccessError::NotEnoughCoins);

        // Verify the merkle proof against the user and amount
        // TODO: Remove assembly when https://github.com/FuelLabs/sway-libs/issues/186 is resolved
        let param = (sender, amount);
        let mut result_buffer: b256 = b256::min();
        let leaf = asm(hash: result_buffer, ptr: param, bytes: size_of_val(param)) {
            s256 hash ptr bytes;
            hash: b256
        };
        require(
            verify_proof(
                key,
                leaf_digest(leaf),
                storage
                    .merkle_root
                    .read()
                    .unwrap(),
                storage
                    .number_of_leaves
                    .read(),
                proof,
            ),
            VerificationError::MerkleProofFailed,
        );

        storage.claims.insert(sender, ClaimState::Claimed(amount));

        // Transfer coins
        transfer(to, asset, amount);

        log(ClaimEvent {
            amount,
            claimer: sender,
            to,
        });
    }

    #[storage(read)]
    fn clawback() {
        let admin = storage.admin.read();
        require(
            admin
                .is_some() && admin
                .unwrap() == msg_sender()
                .unwrap(),
            AccessError::CallerNotAdmin,
        );
        require(
            storage
                .end_block
                .read() <= height(),
            StateError::ClaimPeriodActive,
        );

        let asset = storage.asset.read().unwrap();
        let balance = this_balance(asset);
        require(balance > 0, AccessError::NotEnoughCoins);

        // Send the remaining balance of coins to the admin
        transfer(admin.unwrap(), asset, balance);

        log(ClawbackEvent {
            amount: balance,
            to: admin.unwrap(),
        })
    }

    #[payable]
    #[storage(read, write)]
    fn constructor(
        admin: Identity,
        claim_time: u32,
        merkle_root: b256,
        number_of_leaves: u64,
    ) {
        // If `end_block` is set to a value other than 0, we know that the constructor has already
        // been called.
        require(storage.end_block.read() == 0, InitError::AlreadyInitialized);
        require(msg_amount() > 0, InitError::CannotAirdropZeroCoins);

        let asset = msg_asset_id();
        storage.end_block.write(claim_time + height());
        storage.merkle_root.write(Option::Some(merkle_root));
        storage.asset.write(Option::Some(asset));
        storage.number_of_leaves.write(number_of_leaves);
        storage.admin.write(Option::Some(admin));

        log(CreateAirdropEvent {
            admin,
            asset: asset,
            end_block: claim_time,
            merkle_root,
            number_of_leaves,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn admin() -> Option<Identity> {
        storage.admin.read()
    }

    #[storage(read)]
    fn claim_data(identity: Identity) -> ClaimState {
        storage.claims.get(identity).try_read().unwrap_or(ClaimState::Unclaimed)
    }

    #[storage(read)]
    fn end_block() -> u32 {
        storage.end_block.read()
    }

    #[storage(read)]
    fn is_active() -> bool {
        storage.end_block.read() > height()
    }

    #[storage(read)]
    fn merkle_root() -> Option<b256> {
        storage.merkle_root.read()
    }

    #[storage(read)]
    fn number_of_leaves() -> u64 {
        storage.number_of_leaves.read()
    }
}
