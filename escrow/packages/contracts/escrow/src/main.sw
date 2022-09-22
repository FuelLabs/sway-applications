contract;

// TODO: commented-out code is there because the SDK does not support Vec yet so an array is used

// Our library dependencies
dep data_structures;
dep errors;
dep events;
dep interface;

use data_structures::{Arbiter, Asset, Buyer, EscrowInfo, Seller, State};
use errors::{
    ArbiterInputError,
    AssetInputError,
    DeadlineInputError,
    DepositError,
    StateError,
    UserError,
    UserInputError,
};

use events::{
    AcceptedArbiterEvent,
    CreatedEscrowEvent,
    DepositEvent,
    DisputeEvent,
    PaymentTakenEvent,
    ProposedArbiterEvent,
    ResolvedDisputeEvent,
    ReturnedDepositEvent,
    TransferredToSellerEvent,
    WithdrawnCollateralEvent,
};

use interface::Escrow;
use std::{
    block::height,
    chain::auth::msg_sender,
    context::{call_frames::msg_asset_id, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    option::Option,
    result::Result,
    revert::require,
    storage::StorageMap,
    token::transfer,
};

storage {
    /// Stores all of the escrow ids that the Identity is a part of as an arbiter
    // TODO figure out how to handle when arbiters are changed
    arbiter_escrows: StorageMap<Identity, [u64;1]> = StorageMap {},

    /// Used as a temporary variable for containing a change, proposed by the seller, to the arbiter
    /// Map(ID => Info)
    arbiter_proposal: StorageMap<u64,
    Option<Arbiter>> = StorageMap {
    },

    /// Stores all of the escrow ids that the Identity is a part of as a buyer
    buyer_escrows: StorageMap<Identity, [u64;1]> = StorageMap {},

    /// Information describing an escrow created via create_escrow()
    /// Map(ID => Info)
    escrows: StorageMap<u64,
    EscrowInfo> = StorageMap {
    },

    /// Number of created escrows
    /// Used as an identifier for O(1) look-up in mappings
    escrow_count: u64 = 0,

    /// Stores all of the escrow ids that the Identity is a part of as a seller
    seller_escrows: StorageMap<Identity, [u64;1]> = StorageMap {},
}

impl Escrow for Contract {
    #[storage(read, write)]fn accept_arbiter(identifier: u64) {
        // The assertions ensure that only the buyer can accept a proposal if the escrow has not
        // been completed and the seller has proposed a new arbiter

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::Unauthorized);

        let arbiter = storage.arbiter_proposal.get(identifier);

        // TODO: incomplete compiler defaults the Option<Arbiter> to not be None therefore fee check
        // https://github.com/FuelLabs/sway/issues/2326
        require(arbiter.is_some() && 0 < arbiter.unwrap().fee_amount, StateError::ArbiterHasNotBeenProposed);

        // Upon acceptance we must transfer back the previous fee the seller deposited
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        escrow.arbiter = arbiter.unwrap();

        // We must reset the proposal or the escrow contract will be drained
        storage.arbiter_proposal.insert(identifier, Option::None);
        storage.escrows.insert(identifier, escrow);

        log(AcceptedArbiterEvent {
            identifier
        });
    }

    #[storage(read)]fn arbiter_escrows(arbiter: Identity) -> [u64;1] {
        storage.arbiter_escrows.get(arbiter)
    }

    #[storage(read)]fn arbiter_proposals(identifier: u64) -> Option<Arbiter> {
        storage.arbiter_proposals.get(identifier)
    }

    #[storage(read)]fn buyer_escrows(buyer: Identity) -> [u64;1] {
        storage.buyer_escrows.get(buyer)
    }

    #[storage(read, write)]fn create_escrow(arbiter: Arbiter, assets: [Asset;
    2], buyer: Identity, deadline: u64) {
        // The assertions ensure that assets are specified with a none-zero amount, the arbiter is
        // not the buyer / seller, the arbiter has a fee that they can take upon resolving a dispute
        // and the escrow deadline is set in the future

        // require(0 < assets.len(), AssetInputError::UnspecifiedAssets);
        require(height() < deadline, DeadlineInputError::MustBeInTheFuture);
        require(0 < arbiter.fee_amount, ArbiterInputError::FeeCannotBeZero);
        require(arbiter.fee_amount == msg_amount(), ArbiterInputError::FeeDoesNotMatchAmountSent);
        require(arbiter.asset == msg_asset_id(), ArbiterInputError::AssetDoesNotMatch);
        require(arbiter.address != buyer, ArbiterInputError::CannotBeBuyer);
        require(arbiter.address != msg_sender().unwrap(), ArbiterInputError::CannotBeSeller);

        let mut index = 0;
        // while index < assets.len() {
        // require(0 < assets.get(index).unwrap().amount, AssetInputError::AssetAmountCannotBeZero);
        while index < 2 {
            require(0 < assets[index].amount, AssetInputError::AssetAmountCannotBeZero);
            index += 1;
        }

        let escrow = ~EscrowInfo::new(arbiter, assets, buyer, deadline, msg_sender().unwrap());

        // TODO once the sdk implements vecs this will become
        // storage.arbiter_escrows.insert(arbiter.address, storage.arbiter_escrows.get(arbiter).push(storage.escrow_count));
        storage.arbiter_escrows.insert(arbiter.address, [storage.escrow_count]);
        storage.buyer_escrows.insert(buyer, [storage.escrow_count]);
        storage.seller_escrows.insert(msg_sender().unwrap(), [storage.escrow_count]);
        storage.escrows.insert(storage.escrow_count, escrow);
        storage.escrow_count += 1;

        log(CreatedEscrowEvent {
            escrow, identifier: storage.escrow_count - 1
        });
    }

    #[storage(read, write)]fn deposit(identifier: u64) {
        // The assertions ensure that only the buyer can deposit (only once) prior to the deadline
        // and escrow completion

        let mut escrow = storage.escrows.get(identifier);

        require(height() < escrow.deadline, StateError::EscrowExpired);
        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::Unauthorized);
        require(escrow.buyer.asset.is_none(), StateError::AlreadyDeposited);

        // TODO: https://github.com/FuelLabs/sway/issues/2014
        //       `.contains() -> bool / .position() -> u64` would clean up the loop
        let mut index = 0;
        // while index < escrow.assets.len() {
        // let asset = escrow.assets.get(index).unwrap();
        while index < 2 {
            let asset = escrow.assets[index];

            if asset.id == msg_asset_id() {
                require(asset.amount == msg_amount(), DepositError::IncorrectAssetAmount);
                escrow.buyer.asset = Option::Some(msg_asset_id());
                escrow.buyer.deposited_amount = msg_amount();
                break;
            }

            index += 1;
        }

        // User must deposit one of the specified assets in the correct amount
        require(escrow.buyer.asset.is_some() && 0 < escrow.buyer.deposited_amount, DepositError::IncorrectAssetSent);

        storage.escrows.insert(identifier, escrow);

        log(DepositEvent {
            asset: escrow.buyer.asset.unwrap(), identifier
        });
    }

    #[storage(read, write)]fn dispute(identifier: u64) {
        // The assertions ensure that a dispute can only be raised once by the buyer as long as the
        // escrow is not completed and the buyer has deposited

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(!escrow.disputed, StateError::AlreadyDisputed);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::Unauthorized);
        require(escrow.buyer.asset.is_some() && 0 < escrow.buyer.deposited_amount, StateError::CannotDisputeBeforeDesposit);

        // Lock the escrow
        escrow.disputed = true;
        storage.escrows.insert(identifier, escrow);

        log(DisputeEvent {
            identifier
        });
    }

    #[storage(read)]fn escrows(identifier: u64) -> EscrowInfo {
        storage.escrows.get(identifier)
    }

    #[storage(read, write)]fn propose_arbiter(arbiter: Arbiter, identifier: u64) {
        // The assertions ensure that only the seller can propose a new arbiter and the arbiter
        // cannot be the buyer / seller, the arbiter will be able to take a none-zero payment

        let escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);

        let user = msg_sender().unwrap();

        require(user == escrow.seller.address, UserError::Unauthorized);
        require(arbiter.address != escrow.buyer.address, ArbiterInputError::CannotBeBuyer);
        require(arbiter.address != escrow.seller.address, ArbiterInputError::CannotBeSeller);
        require(0 < arbiter.fee_amount, ArbiterInputError::FeeCannotBeZero);
        require(arbiter.fee_amount == msg_amount(), ArbiterInputError::FeeDoesNotMatchAmountSent);
        require(arbiter.asset == msg_asset_id(), ArbiterInputError::AssetDoesNotMatch);

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() && 0 < proposal.unwrap().fee_amount {
            transfer(proposal.unwrap().fee_amount, proposal.unwrap().asset, escrow.seller.address);
        }

        storage.arbiter_proposal.insert(identifier, Option::Some(arbiter));

        log(ProposedArbiterEvent {
            arbiter, identifier
        });
    }

    #[storage(read, write)]fn resolve_dispute(identifier: u64, payment_amount: u64, user: Identity) {
        // The assertions ensure that a resolution can only occur during a dispute and only once
        // by the specified arbiter. The deposit will be sent to either the buyer or seller and the
        // arbiter can choose their payment amount up to the deposit from the seller

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.disputed, StateError::NotDisputed);
        require(msg_sender().unwrap() == escrow.arbiter.address, UserError::Unauthorized);
        require(user == escrow.buyer.address || user == escrow.seller.address, UserInputError::InvalidRecipient);
        require(escrow.buyer.asset.is_some() && 0 < escrow.buyer.deposited_amount, StateError::CannotResolveBeforeDesposit);
        require(payment_amount <= escrow.arbiter.fee_amount, ArbiterInputError::PaymentTooLarge);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), user);
        transfer(payment_amount, escrow.arbiter.asset, escrow.arbiter.address);

        if payment_amount != escrow.arbiter.fee_amount {
            transfer(escrow.arbiter.fee_amount - payment_amount, escrow.arbiter.asset, escrow.seller.address);
        }

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() && 0 < proposal.unwrap().fee_amount {
            transfer(proposal.unwrap().fee_amount, proposal.unwrap().asset, escrow.seller.address);
            // Not needed as long as the entire contract handles state correctly but leaving it in
            // for conceptual closure at the slight expense of users
            storage.arbiter_proposal.insert(identifier, Option::None);
        }

        log(ResolvedDisputeEvent {
            identifier, user
        });
    }

    #[storage(read, write)]fn return_deposit(identifier: u64) {
        // The assertions ensure that only the seller can return the deposit as long as the escrow
        // contains a deposit and the escrow has not been completed

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.seller.address, UserError::Unauthorized);
        require(escrow.buyer.asset.is_some() && 0 < escrow.buyer.deposited_amount, StateError::CannotTransferBeforeDesposit);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.buyer.address);
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() && 0 < proposal.unwrap().fee_amount {
            transfer(proposal.unwrap().fee_amount, proposal.unwrap().asset, escrow.seller.address);
            // Not needed as long as the entire contract handles state correctly but leaving it in
            // for conceptual closure at the slight expense of users
            storage.arbiter_proposal.insert(identifier, Option::None);
        }

        log(ReturnedDepositEvent {
            identifier
        });
    }

    #[storage(read)]fn seller_escrows(seller: Identity) -> [u64;1] {
        storage.seller_escrows.get(seller)
    }

    #[storage(read, write)]fn take_payment(identifier: u64) {
        // The assertions ensure that only the seller can take payment before the escrow has been
        // completed and after the deadline as long as there is no disupte and it contains a deposit

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.deadline < height(), StateError::CannotTakePaymentBeforeDeadline);
        require(!escrow.disputed, StateError::CannotTakePaymentDuringDispute);
        require(msg_sender().unwrap() == escrow.seller.address, UserError::Unauthorized);
        require(escrow.buyer.asset.is_some() && 0 < escrow.buyer.deposited_amount, StateError::CannotTransferBeforeDesposit);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() && 0 < proposal.unwrap().fee_amount {
            transfer(proposal.unwrap().fee_amount, proposal.unwrap().asset, escrow.seller.address);
            // Not needed as long as the entire contract handles state correctly but leaving it in
            // for conceptual closure at the slight expense of users
            storage.arbiter_proposal.insert(identifier, Option::None);
        }

        log(PaymentTakenEvent {
            identifier
        });
    }

    #[storage(read, write)]fn transfer_to_seller(identifier: u64) {
        // The assertions ensure that only the buyer can transfer their deposit once

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.buyer.asset.is_some() && 0 < escrow.buyer.deposited_amount, StateError::CannotTransferBeforeDesposit);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::Unauthorized);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() && 0 < proposal.unwrap().fee_amount {
            transfer(proposal.unwrap().fee_amount, proposal.unwrap().asset, escrow.seller.address);
            // Not needed as long as the entire contract handles state correctly but leaving it in
            // for conceptual closure at the slight expense of users
            storage.arbiter_proposal.insert(identifier, Option::None);
        }

        log(TransferredToSellerEvent {
            identifier
        });
    }

    #[storage(read, write)]fn withdraw_collateral(identifier: u64) {
        // The assertions ensure that only the seller can withdraw their initial deposit when
        // creating the escrow and additional collateral for a proposed arbiter change

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.deadline < height(), StateError::CannotWithdrawBeforeDeadline);
        require(msg_sender().unwrap() == escrow.seller.address, UserError::Unauthorized);
        require(escrow.buyer.asset.is_none(), StateError::CannotWithdrawAfterDesposit);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() && 0 < proposal.unwrap().fee_amount {
            transfer(proposal.unwrap().fee_amount, proposal.unwrap().asset, escrow.seller.address);
            // Not needed as long as the entire contract handles state correctly but leaving it in
            // for conceptual closure at the slight expense of users
            storage.arbiter_proposal.insert(identifier, Option::None);
        }

        log(WithdrawnCollateralEvent {
            identifier
        });
    }
}
