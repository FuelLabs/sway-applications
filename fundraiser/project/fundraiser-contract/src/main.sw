contract;

dep data_structures/asset_info;
dep data_structures/campaign_info;
dep data_structures/campaign;
dep data_structures/pledge;
dep data_structures/campaign_state;
dep errors;
dep events;
dep interface;
dep utils;

use asset_info::AssetInfo;
use campaign_info::CampaignInfo;
use campaign::Campaign;
use errors::{CampaignError, CreationError, UserError};
use events::{
    CancelledCampaignEvent,
    ClaimedEvent,
    CreatedCampaignEvent,
    PledgedEvent,
    UnpledgedEvent,
};
use pledge::Pledge;
use campaign_state::CampaignState;
use std::{
    auth::msg_sender,
    block::height,
    call_frames::msg_asset_id,
    context::msg_amount,
    logging::log,
    token::transfer,
};
use interface::{Fundraiser, Info};
use utils::validate_campaign_id;

storage {
    /// Total number of unique assets used across all campaigns
    asset_count: u64 = 0,
    /// Direct look-up for asset data if the user wants to check via a known ID
    asset_info: StorageMap<ContractId, Option<AssetInfo>> = StorageMap {},
    /// O(1) look-up to allow searching via asset_count
    /// Map(1...asset_count => asset)
    asset_index: StorageMap<u64, ContractId> = StorageMap {},
    /// The total number of unique campaigns that a user has created
    /// This should only be incremented
    /// Cancelling / Claiming should not affect this number
    user_campaign_count: StorageMap<Identity, u64> = StorageMap {},
    /// Campaigns that have been created by a user
    /// Map(Identity => Map(1...user_campaign_count => Campaign)
    campaign_history: StorageMap<(Identity, u64), Option<Campaign>> = StorageMap {},
    /// Data describing the content of a campaign
    /// Map(Campaign ID => CampaignInfo)
    campaign_info: StorageMap<u64, Option<CampaignInfo>> = StorageMap {},
    /// The total number of unique campaigns that a user has pledged to
    /// This should only be incremented.
    /// Unpledging should not affect this number
    pledge_count: StorageMap<Identity, u64> = StorageMap {},
    /// Record of how much a user has pledged to a specific campaign
    /// Locked after the deadline
    /// Map(Identity => Map(1...pledge_count => Pledge))
    pledge_history: StorageMap<(Identity, u64), Option<Pledge>> = StorageMap {},
    /// O(1) look-up to prevent iterating over pledge_history
    /// Map(Identity => Map(Campaign ID => Pledge History Index))
    pledge_history_index: StorageMap<(Identity, u64), u64> = StorageMap {},
    /// The number of campaigns created by all users
    total_campaigns: u64 = 0,
}

impl Fundraiser for Contract {
    #[storage(read, write)]
    fn cancel_campaign(campaign_id: u64) {
        // User cannot interact with a non-existent campaign
        validate_campaign_id(campaign_id, storage.total_campaigns);

        // Retrieve the campaign in order to check its data / update it
        let mut campaign_info = storage.campaign_info.get(campaign_id).unwrap();

        // Only the creator (author) of the campaign can cancel it
        require(campaign_info.author == msg_sender().unwrap(), UserError::UnauthorizedUser);

        // The campaign can only be cancelled before it has reached its deadline (ended)
        require(height() < campaign_info.deadline, CampaignError::CampaignEnded);

        // User cannot cancel a campaign that has already been cancelled
        // Given the logic below this is unnecessary aside from ignoring event spam
        require(campaign_info.state != CampaignState::Cancelled, CampaignError::CampaignHasBeenCancelled);

        // Mark the campaign as cancelled
        campaign_info.state = CampaignState::Cancelled;

        // Overwrite the previous campaign (which has not been cancelled) with the updated version
        storage.campaign_info.insert(campaign_id, Option::Some(campaign_info));

        // We have updated the state of a campaign therefore we must log it
        log(CancelledCampaignEvent { campaign_id });
    }

    #[storage(read, write)]
    fn claim_pledges(campaign_id: u64) {
        // User cannot interact with a non-existent campaign
        validate_campaign_id(campaign_id, storage.total_campaigns);

        // Retrieve the campaign in order to check its data / update it
        let mut campaign_info = storage.campaign_info.get(campaign_id).unwrap();

        // Only the creator (author) of the campaign can initiate the claiming process
        require(campaign_info.author == msg_sender().unwrap(), UserError::UnauthorizedUser);

        // The author should only have the ability to claim after the deadline has been reached
        // (campaign has naturally ended i.e. has not been cancelled)
        require(campaign_info.deadline <= height(), CampaignError::DeadlineNotReached);

        // The author can only claim the pledges once the target amount has been reached otherwise
        // users should be able to withdraw
        require(campaign_info.target_amount <= campaign_info.total_pledge, CampaignError::TargetNotReached);

        // The author can only claim once to prevent the entire contract from being drained
        require(campaign_info.state != CampaignState::Claimed, UserError::AlreadyClaimed);

        // The author cannot claim after they have cancelled the campaign regardless of any other
        // checks
        require(campaign_info.state != CampaignState::Cancelled, CampaignError::CampaignHasBeenCancelled);

        // Mark the campaign as claimed and overwrite the previous state with the updated version
        campaign_info.state = CampaignState::Claimed;
        storage.campaign_info.insert(campaign_id, Option::Some(campaign_info));

        // Transfer the total pledged to this campaign to the beneficiary
        transfer(campaign_info.total_pledge, campaign_info.asset, campaign_info.beneficiary);

        // We have updated the state of a campaign therefore we must log it
        log(ClaimedEvent { campaign_id });
    }

    #[storage(read, write)]
    fn create_campaign(
        asset: ContractId,
        beneficiary: Identity,
        deadline: u64,
        target_amount: u64,
    ) {
        // Users cannot interact with a campaign that has already ended (is in the past)
        require(height() < deadline, CreationError::DeadlineMustBeInTheFuture);

        // A campaign must have a target to reach and therefore 0 is an invalid amount
        require(0 < target_amount, CreationError::TargetAmountCannotBeZero);

        let author = msg_sender().unwrap();

        // Create an internal representation of a campaign
        let campaign_info = CampaignInfo::new(asset, author, beneficiary, deadline, target_amount);

        // Keep track of new assets
        let mut asset_info = storage.asset_info.get(asset);
        if asset_info.is_none() {
            // Update storage for new asset
            storage.asset_info.insert(asset, Option::Some(AssetInfo::new()));

            // Increment asset count to keep track of new total
            storage.asset_count += 1;

            // Store in index to allow for asset discovery via iteration over numbers
            storage.asset_index.insert(storage.asset_count, asset);
        }

        // Use the user's number of created campaigns as an ID / way to index this new campaign
        let user_campaign_count = storage.user_campaign_count.get(author);

        // We've just created a new campaign so increment the number of created campaigns across all
        // users and store the new campaign
        storage.total_campaigns += 1;
        storage.campaign_info.insert(storage.total_campaigns, Option::Some(campaign_info));

        // Increment the number of campaigns this user has created and track the ID for the campaign
        // they have just created so that data can be easily retrieved without duplicating data
        storage.user_campaign_count.insert(author, user_campaign_count + 1);
        storage.campaign_history.insert((author, user_campaign_count + 1), Option::Some(Campaign::new(storage.total_campaigns)));

        // We have changed the state by adding a new data structure therefore we log it
        log(CreatedCampaignEvent {
            author,
            campaign_info,
            campaign_id: storage.total_campaigns,
        });
    }

    #[storage(read, write)]
    fn pledge(campaign_id: u64) {
        // User cannot interact with a non-existent campaign
        validate_campaign_id(campaign_id, storage.total_campaigns);

        // Retrieve the campaign in order to check its data / update it
        let mut campaign_info = storage.campaign_info.get(campaign_id).unwrap();

        // The users should only have the ability to pledge to campaigns that have not reached their
        // deadline (ended naturally - not been cancelled)
        require(height() < campaign_info.deadline, CampaignError::CampaignEnded);

        // The campaign specifies an asset that it accepts therefore the user must pledge the correct
        // asset in order to update the state of the campaign
        require(campaign_info.asset == msg_asset_id(), UserError::IncorrectAssetSent);

        // A user cannot pledge zero since it does not make sense to do so
        require(0 < msg_amount(), UserError::AmountCannotBeZero);

        // The user should not be able to continue to pledge if the campaign has been cancelled
        // Given the logic below it's unnecessary but it makes sense to stop them
        require(campaign_info.state != CampaignState::Cancelled, CampaignError::CampaignHasBeenCancelled);

        // Use the user's pledges as an ID / way to index this new pledge
        let user = msg_sender().unwrap();
        let pledge_count = storage.pledge_count.get(user);

        // Fetch the index to see if the user has pledged to this campaign before or if this is a
        // pledge to a new campaign
        let pledge_history_index = storage.pledge_history_index.get((user, campaign_id));

        // Pledging to a campaign that they have already pledged to
        if pledge_history_index != 0 {
            // 0 is the sentinel therefore they have pledged to this ID (campaign)
            // increment their previous amount with the current pledge and update their pledge
            let mut pledge = storage.pledge_history.get((user, pledge_history_index)).unwrap();
            pledge.amount += msg_amount();

            storage.pledge_history.insert((user, pledge_history_index), Option::Some(pledge));
        } else {
            // Pledging to a new campaign
            
            // First time pledge to this campaign therefore increment everything by 1
            storage.pledge_count.insert(user, pledge_count + 1);

            // Store the data structure required to look up the campaign they have pledged to, also
            // track how much they have pledged so that they can withdraw the correct amount.
            // Moreover, this can be used to show the user how much they have pledged to any campaign
            storage.pledge_history.insert((user, pledge_count + 1), Option::Some(Pledge::new(msg_amount(), campaign_id)));

            // Since we use the campaign ID to interact with the contract use the ID as a key for
            // a reverse look-up. Value is the 1st pledge (count)
            storage.pledge_history_index.insert((user, campaign_id), pledge_count + 1);
        }

        // The user has pledged therefore we increment the total amount that this campaign has
        // received.
        campaign_info.total_pledge += msg_amount();

        // Campaign state has been updated therefore overwrite the previous version with the new
        storage.campaign_info.insert(campaign_id, Option::Some(campaign_info));

        // Update the asset amount to track the addition of the new pledge
        let mut asset_info = storage.asset_info.get(campaign_info.asset).unwrap();
        asset_info.amount += msg_amount();

        // Update asset state
        storage.asset_info.insert(campaign_info.asset, Option::Some(asset_info));

        // We have updated the state of a campaign therefore we must log it
        log(PledgedEvent {
            amount: msg_amount(),
            campaign_id,
            user,
        });
    }

    #[storage(read, write)]
    fn unpledge(campaign_id: u64, amount: u64) {
        // User cannot interact with a non-existent campaign
        validate_campaign_id(campaign_id, storage.total_campaigns);

        // Prevent a user from unpledging 0 since it does not make sense to do so
        require(amount != 0, UserError::AmountCannotBeZero);

        // Retrieve the campaign in order to check its data / update it
        let mut campaign_info = storage.campaign_info.get(campaign_id).unwrap();

        // A user should be able to unpledge at any point except if the deadline has been reached
        // and the author has claimed
        if campaign_info.deadline <= height() {
            require(campaign_info.state != CampaignState::Claimed, UserError::AlreadyClaimed);
        }

        // Check if the user has pledged to the campaign they are attempting to unpledge from
        let user = msg_sender().unwrap();
        let pledge_history_index = storage.pledge_history_index.get((user, campaign_id));

        require(pledge_history_index != 0, UserError::UserHasNotPledged);

        // User has pledged therefore retrieve the total that they have pledged
        let mut pledge = storage.pledge_history.get((user, pledge_history_index)).unwrap();
        let mut amount = amount; // https://github.com/FuelLabs/sway/issues/3570
        // If the user is attempting to unpledge more than they have pledged then reset the amount
        // they are withdrawing to the maximum that they have pledged to this campaign
        if pledge.amount < amount {
            amount = pledge.amount;
        }

        // Update the amount that they have pledged
        pledge.amount -= amount;

        // Lower the campaign total pledge by the amount the user has unpledged
        campaign_info.total_pledge -= amount;

        // Update the state of their pledge with the new version
        storage.pledge_history.insert((user, pledge_history_index), Option::Some(pledge));

        // Update the campaign state with the updated version as well
        storage.campaign_info.insert(campaign_id, Option::Some(campaign_info));

        // Update the asset amount to track the removal of the amount
        let mut asset_info = storage.asset_info.get(campaign_info.asset).unwrap();
        asset_info.amount -= amount;

        // Update asset state
        storage.asset_info.insert(campaign_info.asset, Option::Some(asset_info));

        // Transfer back the amount the user has unpledged
        transfer(amount, campaign_info.asset, user);

        // We have updated the state of a campaign therefore we must log it
        log(UnpledgedEvent {
            amount,
            campaign_id,
            user,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn asset_count() -> u64 {
        storage.asset_count
    }

    #[storage(read)]
    fn asset_info_by_count(index: u64) -> Option<AssetInfo> {
        storage.asset_info.get(storage.asset_index.get(index))
    }

    #[storage(read)]
    fn asset_info_by_id(asset: ContractId) -> Option<AssetInfo> {
        storage.asset_info.get(asset)
    }

    #[storage(read)]
    fn campaign_info(campaign_id: u64) -> Option<CampaignInfo> {
        storage.campaign_info.get(campaign_id)
    }

    #[storage(read)]
    fn campaign(campaign_id: u64, user: Identity) -> Option<Campaign> {
        storage.campaign_history.get((user, campaign_id))
    }

    #[storage(read)]
    fn pledge_count(user: Identity) -> u64 {
        storage.pledge_count.get(user)
    }

    #[storage(read)]
    fn pledged(pledge_history_index: u64, user: Identity) -> Option<Pledge> {
        storage.pledge_history.get((user, pledge_history_index))
    }

    #[storage(read)]
    fn total_campaigns() -> u64 {
        storage.total_campaigns
    }

    #[storage(read)]
    fn user_campaign_count(user: Identity) -> u64 {
        storage.user_campaign_count.get(user)
    }
}
