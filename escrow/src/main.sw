contract;

// Our library dependencies
dep data_structures;
dep errors;
dep events;
dep interface;

use data_structures::{Arbiter, Asset, Buyer, EscrowInfo, Seller, State};
use errors::{CreationError, DepositError, StateError, UserError};
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
};
use interface::Escrow;
use std::{
    assert::require,
    block::height,
    chain::auth::msg_sender,
    context::{call_frames::msg_asset_id, msg_amount},
    contract_id::ContractId,
    identity::Identity,
    logging::log,
    option::Option,
    result::Result,
    storage::StorageMap,
    token::transfer,
    vec::Vec,
};

storage {
    /// Used as a temporary variable for containing a change, proposed by the seller, to the arbiter
    /// Map(ID => Info)
    arbiter_proposal: StorageMap<u64, Option<Arbiter>>,

    /// Information describing an escrow created via create_escrow()
    /// Map(ID => Info)
    escrows: StorageMap<u64, EscrowInfo>, 
    
    /// Number of created escrows
    /// Used as an identifier for O(1) look-up in mappings
    escrow_count: u64,
}

impl Escrow for Contract {

    #[storage(read, write)]
    fn accept_arbiter(identifier: u64) {
        // The assertions ensure that only the buyer can accept a proposal if the escrow has not 
        // been completed and the seller has proposed a new arbiter

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);

        let arbiter = storage.arbiter_proposal.get(identifier);

        require(arbiter.is_some(), "TODO");

        // Upon acceptance we must transfer back the previous fee the seller deposited
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        escrow.arbiter = arbiter.unwrap();

        // We must reset the proposal or the escrow contract will be drained
        storage.arbiter_proposal.insert(identifier, Option::None);
        storage.escrows.insert(identifier, escrow);

        log(AcceptedArbiterEvent { identifier });
    }

    #[storage(read, write)]fn create_escrow(arbiter: Arbiter, assets: Vec<Asset>, buyer: Identity, deadline: u64) {
        // The assertions ensure that assets are specified with a none-zero amount, the arbiter is 
        // not the buyer / seller, the arbiter has a fee that they can take upon resolving a dispute
        // and the escrow deadline is set in the future

        require(0 < assets.len(), CreationError::UnspecifiedAssets);
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);
        require(0 < arbiter.fee_amount, CreationError::ArbiterFeeCannotBeZero);
        require(arbiter.fee_amount == msg_amount(), CreationError::ArbiterFeeDoesNotMatchAmountSent);
        require(arbiter.asset == msg_asset_id(), "TODO");
        require(arbiter.address != buyer, CreationError::ArbiterCannotBeBuyer);
        require(arbiter.address != msg_sender().unwrap(), CreationError::ArbiterCannotBeSeller);

        let mut index = 0;
        while index < assets.len() {
            require(0 < assets.get(index).unwrap().amount, CreationError::DepositAmountCannotBeZero);
            index += 1;
        }

        let escrow = EscrowInfo {
            arbiter, 
            assets, 
            buyer: Buyer {
                address: buyer,
                asset: Option::None::<ContractId>(),
                deposited_amount: 0,
            },
            deadline, 
            disputed: false,
            seller: Seller {
                address: msg_sender().unwrap(),
            },
            state: State::Pending,
        };

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

        require(height() < escrow.deadline, DepositError::EscrowExpired);
        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_none(), DepositError::AlreadyDeposited);

        // TODO: https://github.com/FuelLabs/sway/issues/2014
        //       `.contains() -> bool / .position() -> u64` would clean up the loop
        let mut index = 0;
        while index < escrow.assets.len() {
            let asset = escrow.assets.get(index).unwrap();

            if asset.id == msg_asset_id() {
                require(asset.amount == msg_amount(), DepositError::IncorrectAssetAmount);
                escrow.buyer.asset = Option::Some(msg_asset_id());
                escrow.buyer.deposited_amount = msg_amount();
                break;
            }

            index += 1;
        }

        // User must deposit one of the specified assets in the correct amount
        require(escrow.buyer.asset.is_some(), DepositError::IncorrectAssetDeposited);

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
        require(!escrow.disputed, UserError::AlreadyDisputed);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_some(), UserError::CannotDisputeBeforeDesposit);

        // Lock the escrow
        escrow.disputed = true;
        storage.escrows.insert(identifier, escrow);

        log(DisputeEvent {
            identifier
        });
    }

    #[storage(read, write)]fn propose_arbiter(arbiter: Arbiter, identifier: u64) {
        // The assertions ensure that only the seller can propose a new arbiter and the arbiter
        // cannot be the buyer / seller, the arbiter will be able to take a none-zero payment

        let escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);

        let user = msg_sender().unwrap();

        require(user == escrow.seller.address, UserError::UnauthorizedUser);
        require(arbiter.address != escrow.buyer.address, CreationError::ArbiterCannotBeBuyer);
        require(arbiter.address != escrow.seller.address, CreationError::ArbiterCannotBeSeller);
        require(0 < arbiter.fee_amount, CreationError::ArbiterFeeCannotBeZero);
        require(arbiter.fee_amount == msg_amount(), "TODO");
        require(arbiter.asset == msg_asset_id(), "TODO");

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() {
            transfer(proposal.unwrap().fee_amount, proposal.unwrap().asset, escrow.seller.address);
        }

        storage.arbiter_proposal.insert(identifier, Option::Some(arbiter));

        log(ProposedArbiterEvent { arbiter, identifier });
    }

    #[storage(read, write)]fn resolve_dispute(identifier: u64, payment_amount: u64, user: Identity) {
        // The assertions ensure that a resolution can only occur during a dispute and only once 
        // by the specified arbiter. The deposit will be sent to either the buyer or seller and the
        // arbiter can choose their payment amount up to the deposit from the seller

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.disputed, UserError::NotDisputed);
        require(msg_sender().unwrap() == escrow.arbiter.address, UserError::UnauthorizedUser);
        require(user == escrow.buyer.address || user == escrow.seller.address, UserError::InvalidRecipient);
        require(escrow.buyer.asset.is_some(), UserError::CannotResolveBeforeDesposit);
        require(payment_amount <= escrow.arbiter.fee_amount, UserError::ArbiterPaymentCannotBeGreaterThanDepositFromSeller);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), user);
        transfer(payment_amount, escrow.arbiter.asset, escrow.arbiter.address);

        if payment_amount != escrow.arbiter.fee_amount {
            transfer(escrow.arbiter.fee_amount - payment_amount, escrow.arbiter.asset, escrow.seller.address);
        }

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() {
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
        require(msg_sender().unwrap() == escrow.seller.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.buyer.address);
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() {
            transfer(proposal.unwrap().fee_amount, proposal.unwrap().asset, escrow.seller.address);
            // Not needed as long as the entire contract handles state correctly but leaving it in
            // for conceptual closure at the slight expense of users
            storage.arbiter_proposal.insert(identifier, Option::None);
        }

        log(ReturnedDepositEvent {
            identifier
        });
    }

    #[storage(read, write)]fn take_payment(identifier: u64) {
        // The assertions ensure that only the seller can take payment before the escrow has been 
        // completed and after the deadline as long as there is no disupte and it contains a deposit

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.deadline < height(), UserError::CannotTakePaymentBeforeDeadline);
        require(!escrow.disputed, UserError::CannotTakePaymentDuringDispute);
        require(msg_sender().unwrap() == escrow.seller.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() {
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
        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);
        transfer(escrow.arbiter.fee_amount, escrow.arbiter.asset, escrow.seller.address);

        // If there is a previous proposal then we must transfer those funds back to the seller
        let proposal = storage.arbiter_proposal.get(identifier);
        if proposal.is_some() {
            transfer(proposal.unwrap().fee_amount, proposal.unwrap().asset, escrow.seller.address);
            // Not needed as long as the entire contract handles state correctly but leaving it in
            // for conceptual closure at the slight expense of users
            storage.arbiter_proposal.insert(identifier, Option::None);
        }

        log(TransferredToSellerEvent {
            identifier
        });
    }
}
