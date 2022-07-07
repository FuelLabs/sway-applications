contract;

// TODO: arbiter fees
//       make the seller deposit as well so that the user can get refunded some portion of the dispute?
//
//      deadline for withdrawal?
//
//       over collateralize (additional fee / penalty) in case process is not smooth, set base price and fee price
//
//       In the distant future it *should* be cheaper to interact with each escrow therefore this would
//       need to be split into a factory script and a single escrow contract each time

/*
As a buyer I should be able to
    1. Deposit
    2. Transfer my deposit to the seller
    3. Retrieve my deposit if the conditions of sale have not been met
    4. File a dispute

Buyer ->
    deposit() -> transfer_to_seller()
    deposit() -> "retrieve deposit"
    deposit() -> dispute()

----

As a seller I should be able to
    1. Specify payment conditions (create escrow)
    2. Receive payment

Seller ->
    create_escrow()
    ... -> "receive payment"

---- 

1. USER deposit() -> USER transfer_to_seller()
2. USER deposit() -> escrow expires -> SELLER take_payment()
3. USER deposit() -> USER dispute() -> locks escrow -> Arbiter resolve_dispute(user)
4. USER deposit() -> SELLER return_deposit()

In
1. Buyer is honest and transfers themselves
2. Buyer may be busy or not have enough funds to transfer therefore seller can take payment since 
   no dispute has been initiated (assumed that no problems so seller can take)
3. Buyer may not have received advertised item or in the correct condition or they are malicious
   therefore arbiter must resolve now
4. Seller may choose to return deposit if their item is faulty or returned to them in time but there
   is no incentive for them to pay the additional Tx to release the funds so user can dispute

Only case 3 results in Arbiter getting a fee if fee != 0
*/

// Our library dependencies
dep data_structures;
dep errors;
dep events;
dep interface;
dep utils;

use data_structures::{Arbiter, ArbiterProposal, Asset, Buyer, EscrowInfo, Seller, State};
use errors::{CreationError, DepositError, StateError, UserError};
use events::{
    ChangedArbiterEvent,
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
use utils::change_arbiter;

storage {
    /// If either party want to change the arbiter that data must be stored somewhere as "temporary"
    /// data. This does not belong in EscrowInfo hence a separate variable
    arbiter_proposal: StorageMap<u64, ArbiterProposal>,

    /// Information describing an escrow created via create_escrow()
    /// Map(ID => Info)
    escrows: StorageMap<u64,
    EscrowInfo>, /// Number of created escrows
    /// Used as an identifier for O(1) look-up in mappings
    escrow_count: u64,
}

impl Escrow for Contract {
    #[storage(read, write)]fn change_arbiter(arbiter: Arbiter, identifier: u64) {
        // TODO
        // The assertions ensure that
        //  - 

        require(arbiter.fee_percentage <= 100, CreationError::ArbiterFeeCannotExceed100Percent);

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);

        let user = msg_sender().unwrap();

        require(user == escrow.buyer.address || user == escrow.seller.address, UserError::UnauthorizedUser);
        require(arbiter.address != escrow.buyer.address, CreationError::ArbiterCannotBeBuyer);
        require(arbiter.address != escrow.seller.address, CreationError::ArbiterCannotBeSeller);

        let mut proposal = storage.arbiter_proposal.get(identifier);

        let user_proposal = if user == escrow.buyer.address { proposal.seller } else { proposal.buyer };
        let (update_state, escrow, proposal) = change_arbiter(arbiter, escrow, identifier, proposal, user, user_proposal);
        if update_state {
            storage.arbiter_proposal.insert(identifier, proposal);
            storage.escrows.insert(identifier, escrow);
        }
    }

    #[storage(read, write)]fn create_escrow(assets: Vec<Asset>, arbiter: Identity, arbiter_fee_percentage: u64, buyer: Identity, deadline: u64) {
        require(0 < assets.len(), CreationError::UnspecifiedAssets);
        require(arbiter_fee_percentage <= 100, CreationError::ArbiterFeeCannotExceed100Percent);
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);
        require(arbiter != buyer, CreationError::ArbiterCannotBeBuyer);
        require(arbiter != msg_sender().unwrap(), CreationError::ArbiterCannotBeSeller);

        let mut index = 0;
        while index < assets.len() {
            require(0 < assets.get(index).unwrap().amount, CreationError::DepositAmountCannotBeZero);
            index += 1;
        }

        let escrow = EscrowInfo {
            arbiter, arbiter_fee_percentage, assets, buyer: Buyer {
                address: buyer,
                asset: Option::None::<ContractId>(),
                deposited_amount: 0,
                disputed: false,
            },
            deadline, disputed: false,
            seller: Seller {
                address: msg_sender().unwrap(),
                disputed: false,
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
        // The assertions ensure that
        //  - Escrow is active (within deadline and not completed)
        //  - Only the buyer can deposit
        //  - They can only deposit once and it must be the correct asset in its stated amount

        let mut escrow = storage.escrows.get(identifier);

        require(height() < escrow.deadline, DepositError::EscrowExpired);
        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_none(), DepositError::AlreadyDeposited);

        // TODO: https://github.com/FuelLabs/sway/issues/2014
        //       `.contains()` would clean up the loop
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

        // Update escrow state in storage
        storage.escrows.insert(identifier, escrow);

        log(DepositEvent {
            asset: escrow.buyer.asset.unwrap(), identifier
        });
    }

    #[storage(read, write)]fn dispute(identifier: u64) {
        // The assertions ensure that
        //  - The escrow has not been completed
        //  - The escrow is not already in dispute
        //  - Only the buyer can dispute
        //  - The buyer has made a deposit
        // Note that you can dispute even after the deadline

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(!escrow.disputed, UserError::AlreadyDisputed);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_some(), UserError::CannotDisputeBeforeDesposit);

        // Lock the escrow down
        escrow.disputed = true;
        escrow.buyer.disputed = true;
        storage.escrows.insert(identifier, escrow);

        log(DisputeEvent {
            identifier
        });
    }

    #[storage(read, write)]fn resolve_dispute(identifier: u64, user: Identity) {
        let mut escrow = storage.escrows.get(identifier);

        // User can only dispute when the escrow state has not reached completion
        require(escrow.state == State::Pending, StateError::StateNotPending);

        // Note that a resolution cannot occur before or after deadline therefore no check for deadline

        // Cannot resolve a dispute if not in the disputed state
        require(escrow.disputed, UserError::NotDisputed);

        // Only the arbiter can resolve a dispute
        require(msg_sender().unwrap() == escrow.arbiter, UserError::UnauthorizedUser);

        require(user == escrow.buyer.address || user == escrow.seller.address, UserError::InvalidRecipient);

        require(escrow.buyer.asset.is_some(), UserError::CannotResolveBeforeDesposit);

        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), user);
        // Add fee to arbiter
        // transfer(some fee ,some asset, escrow.arbiter);

        log(ResolvedDisputeEvent {
            identifier, user
        });
    }

    #[storage(read, write)]fn return_deposit(identifier: u64) {
        // The assertions ensure that
        //  - The escrow has not been completed
        //  - Only the seller can "return" the deposit in the escrow
        //  - The buyer has made a deposit
        // Note that you can dispute even after the deadline

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(msg_sender().unwrap() == escrow.seller.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);

        // Conditions have been cleared, lock the escrow down and transfer funds back to buyer
        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.buyer.address);

        log(ReturnedDepositEvent {
            identifier
        });
    }

    #[storage(read, write)]fn take_payment(identifier: u64) {
        // The assertions ensure that
        //  - The escrow has not been completed
        //  - The call is made after the deadline of the escrow
        //  - The buyer has not initiated a dispute
        //  - Only the seller can call to take the payment
        //  - The buyer has deposited

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.deadline < height(), UserError::CannotTakePaymentBeforeDeadline);
        require(!escrow.disputed, UserError::CannotTakePaymentDuringDispute);
        require(msg_sender().unwrap() == escrow.seller.address, UserError::UnauthorizedUser);
        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);

        // Conditions have been cleared, lock the escrow down and transfer funds to seller
        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);

        log(PaymentTakenEvent {
            identifier
        });
    }

    #[storage(read, write)]fn transfer_to_seller(identifier: u64) {
        // The assertions ensure that
        //  - The escrow has not been completed
        //  - The buyer has made a deposit
        //  - Only the buyer can transfer their deposit

        let mut escrow = storage.escrows.get(identifier);

        require(escrow.state == State::Pending, StateError::StateNotPending);
        require(escrow.buyer.asset.is_some(), UserError::CannotTransferBeforeDesposit);
        require(msg_sender().unwrap() == escrow.buyer.address, UserError::UnauthorizedUser);

        // Conditions have been cleared, lock the escrow down and transfer funds to seller
        escrow.state = State::Completed;
        storage.escrows.insert(identifier, escrow);

        transfer(escrow.buyer.deposited_amount, escrow.buyer.asset.unwrap(), escrow.seller.address);

        log(TransferredToSellerEvent {
            identifier
        });
    }
}
