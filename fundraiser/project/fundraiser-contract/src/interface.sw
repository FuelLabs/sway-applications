library interface;

dep data_structures/asset_info;
dep data_structures/campaign_info;
dep data_structures/campaign;
dep data_structures/pledge;

use asset_info::AssetInfo;
use campaign_info::CampaignInfo;
use campaign::Campaign;
use pledge::Pledge;

abi Fundraiser {
    /// Marks a campaign as cancelled preventing further pledges or a claim to be made
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created
    /// * When the user is not the author of the campaign
    /// * When the deadline has been surpassed
    /// * When the campaign has already been cancelled
    #[storage(read, write)]
    fn cancel_campaign(id: u64);

    /// Transfers the total pledge to the beneficiary
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created
    /// * When the user is not the author of the campaign
    /// * When the deadline has not been reached
    /// * When the total pledge has not reached the minimum `target_amount`
    /// * When the campaign has already been claimed
    /// * When the campaign has already been cancelled
    #[storage(read, write)]
    fn claim_pledges(id: u64);

    /// Creates a data structure representing a campaign that users can pledge to
    ///
    /// Instead of having a contract per campaign we create an internal representation for the data
    /// and manage it via mappings.
    ///
    /// # Arguments
    ///
    /// * `asset` - A coin that the campaign accepts as a pledge
    /// * `beneficiary` - The recipient to whom the pledge will be sent to upon a successful campaign
    /// * `deadline` - Block height used to dictate the end time of a campaign
    /// * `target_amount` - The amount of `asset` required to deem the campaign a success
    ///
    /// # Reverts
    ///
    /// * When the `deadline` is not ahead of the current block height
    /// * When the `target_amount` is 0
    #[storage(read, write)]
    fn create_campaign(asset: ContractId, beneficiary: Identity, deadline: u64, target_amount: u64);

    /// Allows a user to pledge any amount of the campaign asset towards the campaign goal
    ///
    /// In order to reach the campaign's target amount users must pledge some amount of asset towards
    /// that campaign.
    /// This information is recorded for the campaign and for the user so that they can unpledge.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created
    /// * When the user attempts to pledge when the deadline has been reached
    /// * When the user pledges a different asset to the one specified in the campaign
    /// * When the user pledges after the campaign has been cancelled
    #[storage(read, write)]
    fn pledge(id: u64);

    /// Allows a user to unpledge an amount of the campaign asset that they have pledged
    ///
    /// A user may have changed their mind about the amount of an asset that they have pledged
    /// therefore they may wish to unpledge some amount of that pledge.
    /// If they attempt to unpledge more than they have pledged then their total pledge will be returned
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    /// * `amount` - The amount of asset that the user wishes to unpledge
    ///
    /// # Reverts
    ///
    /// * When the `id` is either 0 or greater than the total number of campaigns created
    /// * When the user attempts to unpledge after the deadline and `target_amount` has been reached
    /// * When the user has not pledged to the campaign represented by the `id`
    #[storage(read, write)]
    fn unpledge(id: u64, amount: u64);
}

abi Info {
    /// Returns the number of unique assets that have been pledged across all campaigns
    #[storage(read)]
    fn asset_count() -> u64;

    /// Returns information about the specificed asset, specifically if it has been added and the
    /// pledged amount
    ///
    /// The user interface will not know all possible assets that the contract contains therefore
    /// this helper method allows the interface to iterate over the asset_count to discover all assets
    ///
    /// # Arguments
    ///
    /// * `index` - Number from 1...asset_count
    #[storage(read)]
    fn asset_info_by_count(index: u64) -> Option<AssetInfo>;

    /// Returns information about the specificed asset, specifically if it has been added and the
    /// pledged amount
    ///
    /// # Arguments
    ///
    /// * `asset` - Uniquie identifier that identifies the asset
    #[storage(read)]
    fn asset_info_by_id(asset: ContractId) -> Option<AssetInfo>;

    /// Returns information about the specified campaign
    ///
    /// # Arguments
    ///
    /// * `id` - Unique campaign identifier which is a number from the storage.total_campaigns range
    #[storage(read)]
    fn campaign_info(id: u64) -> Option<CampaignInfo>;

    /// Returns information about the specified campaign for the campaign author
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier which is a number starting from 1...storage.user_campaign_count
    #[storage(read)]
    fn campaign(campaign_history_index: u64, user: Identity) -> Option<Campaign>;

    /// Returns the number of campaigns that the user has pledged to
    #[storage(read)]
    fn pledge_count(user: Identity) -> u64;

    /// Returns information about the specified pledge for the user
    ///
    /// # Arguments
    ///
    /// * `pledge_history_index` - Unique identifier which is a number starting from 1...storage.pledge_count
    /// * `user` - The user that has pledged to a campaign
    #[storage(read)]
    fn pledged(pledge_history_index: u64, user: Identity) -> Option<Pledge>;

    /// Returns the total number of campaigns that have been created by all users
    #[storage(read)]
    fn total_campaigns() -> u64;

    /// Returns the number of campaigns that the user has created
    #[storage(read)]
    fn user_campaign_count(user: Identity) -> u64;
}
