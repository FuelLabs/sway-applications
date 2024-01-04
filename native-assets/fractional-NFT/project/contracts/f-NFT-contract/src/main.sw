contract;

mod data_structures;
mod errors;

use data_structures::State;
use errors::{
    AccessError,
    BuybackError,
    DepositError,
    StateError,
    VaultCreationError,
    WithdrawError,
};
use src6::{Deposit, SRC6, Withdraw};
use src20::SRC20;
use std::{
    auth::msg_sender,
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    constants::BASE_ASSET_ID,
    context::{
        msg_amount,
        this_balance,
    },
    hash::{
        Hash,
        sha256,
    },
    string::String,
    token::{
        burn,
        mint_to,
        transfer,
    },
};

configurable {
    /// The decimals of fractionalized NFT assets minted by this contract.
    DECIMALS: u8 = 9u8,
    /// The name of fractionalized NFT assets minted by this contract.
    NAME: str[18] = __to_str_array("Fractionalized NFT"),
    /// The symbol of fractionalized NFT assets minted by this contract.
    SYMBOL: str[4] = __to_str_array("FNFT"),
}

storage {
    state: StorageMap<AssetId, State> = StorageMap {},
    vault_admin: StorageMap<(AssetId, Identity), bool> = StorageMap {},
    share_price: StorageMap<AssetId, u64> = StorageMap {},
    total_assets: u64 = 0,
    total_supply: StorageMap<AssetId, u64> = StorageMap {},
    minted_shares: StorageMap<AssetId, u64> = StorageMap {},
    proceeds: StorageMap<AssetId, u64> = StorageMap {},
}

abi FractionalNFTManager {
    #[storage(read, write)]
    fn start_buyback(nft: AssetId, token_buyback_price: u64);
    #[storage(read, write)]
    #[payable]
    fn create_vault(
        nft_contract: ContractId,
        nft_sub_id: SubId,
        vault_admin: Identity,
        initial_price: u64,
        supply: u64,
    );
    #[storage(read, write)]
    fn withdraw_nft(nft: AssetId);
    #[storage(read, write)]
    fn payout(nft: AssetId);
}

impl FractionalNFTManager for Contract {
    /// Starts the buyback process of Fractionalized NFT tokens.
    ///
    /// # Additional Information
    ///
    /// Once the buyback process has started users will no longer be able to buy F-NFT assets.
    ///
    /// # Arguments
    ///
    /// * `nft`: [AssetId] - The NFT of which the begin the buyback process.
    /// * `token_buyback_price`: [u64] - The price at which a F-NFT asset will be purchased for.
    ///
    /// # Reverts
    ///
    /// * When the sender is not the NFT admin.
    /// * When the vault is not in the distribution state.
    /// * When not enough assets are provided to cover all buybacks.
    #[storage(read, write)]
    fn start_buyback(nft: AssetId, token_buyback_price: u64) {
        let sender = msg_sender().unwrap();
        require(
            storage
                .vault_admin
                .get((nft, sender))
                .try_read()
                .unwrap_or(false),
            AccessError::NotVaultAdmin,
        );
        require(
            storage
                .state
                .get(nft)
                .read() == State::Distribution,
            StateError::InvalidState,
        );
        require(
            msg_amount() == (token_buyback_price * storage
                .minted_shares
                .get(nft)
                .read()) - storage
                .proceeds
                .get(nft)
                .read(),
            BuybackError::NotEnoughTokens,
        );
        require(msg_asset_id() == BASE_ASSET_ID, BuybackError::InvalidAsset);

        // Should acount for proceeds.
        storage.share_price.insert(nft, token_buyback_price);
        storage.state.insert(nft, State::Buyback);
        storage.proceeds.insert(nft, 0);
    }

    /// Deposits an NFT and creates a new Fractional NFT Vault.
    ///
    /// # Arguments
    ///
    /// * `nft_contract`: [ContractId] - The contract which minted the NFT and holds it's metadata.
    /// * `nft_sub_id`: [SubId] - The SubId of the NFT.
    /// * `vault_admin`: [Identity] - The user which shall be the admin of this F-NFT vault.
    /// * `price`: [u64] - The price at which a single vault share will be sold for.
    /// * `supply`: [u64] - The total supply of shares for the F-NFT vault.
    ///
    /// # Revert
    ///
    /// * When the asset sent does not match the NFT contract and SubId arguments.
    /// * When more than a single token is sent.
    /// * When the NFT does not follow the SRC-20 total supply specification.
    /// * When the NFT does not follow the SRC-20 decimals specification.
    #[storage(read, write)]
    #[payable]
    fn create_vault(
        nft_contract: ContractId,
        nft_sub_id: SubId,
        vault_admin: Identity,
        price: u64,
        supply: u64,
    ) {
        let nft = msg_asset_id();
        require(
            nft == AssetId::new(nft_contract, nft_sub_id),
            VaultCreationError::InvalidContractOrSubId,
        );
        require(msg_amount() == 1, VaultCreationError::InvalidSRC20NFT);

        // Verify that the NFT follows the SRC-20 standards
        let nft_abi = abi(SRC20, nft_contract.value);
        require(
            nft_abi
                .total_supply(nft)
                .unwrap_or(0) == 1,
            VaultCreationError::InvalidSRC20NFT,
        );
        require(
            nft_abi
                .decimals(nft)
                .unwrap() == 0u8,
            VaultCreationError::InvalidSRC20NFT,
        );

        storage.vault_admin.insert((nft, vault_admin), true);
        storage.share_price.insert(nft, price);
        storage.total_assets.write(storage.total_assets.read() + 1);
        storage.state.insert(nft, State::Distribution);

        let f_nft_asset = AssetId::from(sha256((BASE_ASSET_ID, nft)));
        storage.total_supply.insert(f_nft_asset, supply);
    }

    /// Withdraws an NFT once the buyback process has been completed.
    ///
    /// # Arguments
    ///
    /// * `nft`: [AssetId] - The NFT to withdraw.
    ///
    /// # Reverts
    ///
    /// * When the sender is not the vault admin.
    /// * When all F-NFT assets have not bee returned.
    #[storage(read, write)]
    fn withdraw_nft(nft: AssetId) {
        let sender = msg_sender().unwrap();
        require(
            storage
                .vault_admin
                .get((nft, sender))
                .try_read()
                .unwrap_or(false),
            AccessError::NotVaultAdmin,
        );

        let f_nft_asset = AssetId::from(sha256((BASE_ASSET_ID, nft)));
        require(
            this_balance(f_nft_asset) == storage
                .total_supply
                .get(f_nft_asset)
                .read(),
            WithdrawError::AllSharesNotReturned,
        );

        storage.total_supply.insert(f_nft_asset, 0);
        storage.state.insert(nft, State::Withdrawn);
        transfer(sender, nft, 1);
    }

    /// Allows a vault admin to withdraw the proceeds of vault shares.
    ///
    /// # Arguments
    ///
    /// * `nft`: [AssetId] - The NFT proceeds to withdraw.
    ///
    /// # Reverts
    ///
    /// * When the sender is not the vault admin.
    /// * When the vault is not in the distribution state.
    #[storage(read, write)]
    fn payout(nft: AssetId) {
        let sender = msg_sender().unwrap();
        require(
            storage
                .vault_admin
                .get((nft, sender))
                .try_read()
                .unwrap_or(false),
            AccessError::NotVaultAdmin,
        );
        require(
            storage
                .state
                .get(nft)
                .read() == State::Distribution,
            StateError::InvalidState,
        );

        let payout_amount = storage.proceeds.get(nft).read();
        storage.proceeds.insert(nft, 0);
        transfer(sender, BASE_ASSET_ID, payout_amount);
    }
}

impl SRC6 for Contract {
    /// Deposits assets into the contract and mints NFT shares to the receiver.
    ///
    /// # Additional Information
    ///
    /// * The Vault's SubId is the NFT's AssetId.
    ///
    /// # Arguments
    ///
    /// * `receiver`: [Identity] - The receiver of the shares.
    /// * `vault_sub_id`: [SubId] - The SubId of the vault.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of shares minted.
    ///
    /// # Reverts
    ///
    /// * When the asset sent is not the base asset.
    /// * When the vault is not in the distribution state.
    /// * When the total supply of shares has already been minted.
    #[storage(read, write)]
    fn deposit(receiver: Identity, vault_sub_id: SubId) -> u64 {
        require(msg_asset_id() == BASE_ASSET_ID, DepositError::InvalidAsset);

        let nft_asset = AssetId::from(vault_sub_id);
        require(
            storage
                .state
                .get(nft_asset)
                .try_read()
                .unwrap_or(State::Uninitialized) == State::Distribution,
            StateError::InvalidState,
        );

        let sent_amount = msg_amount();
        let f_nft_asset_sub_id = sha256((BASE_ASSET_ID, vault_sub_id));
        let f_nft_asset = AssetId::new(contract_id(), f_nft_asset_sub_id);
        let minted_shares = storage.minted_shares.get(f_nft_asset).read();
        let new_shares = storage.share_price.get(nft_asset).read() * sent_amount;

        require(
            new_shares + minted_shares <= storage
                .total_supply
                .get(f_nft_asset)
                .read(),
            DepositError::NotEnoughTokensAvailable,
        );

        storage
            .minted_shares
            .insert(f_nft_asset, minted_shares + new_shares);
        storage
            .proceeds
            .insert(
                nft_asset,
                storage
                    .proceeds
                    .get(nft_asset)
                    .read() + sent_amount,
            );
        mint_to(receiver, f_nft_asset_sub_id, new_shares);

        log(Deposit {
            caller: msg_sender().unwrap(),
            receiver: receiver,
            underlying_asset: BASE_ASSET_ID,
            vault_sub_id: vault_sub_id,
            deposited_amount: sent_amount,
            minted_shares: new_shares,
        });

        new_shares
    }

    /// Burns shares from the sender and transfers assets to the receiver.
    ///
    /// # Additional Information
    ///
    /// * Shares must be forwarded to the contract in the contract call.
    ///
    /// # Arguments
    ///
    /// * `receiver`: [Identity] - The receiver of the assets.
    /// * `underlying_asset`: [AssetId] - The asset for which the shares should be burned.
    /// * `vault_sub_id`: [SubId] - The SubId of the vault.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of assets transferred.
    ///
    /// # Reverts
    ///
    /// * If the asset is not supported by the contract.
    /// * If the amount of shares is zero.
    /// * If the transferred shares do not corresspond to the given asset.
    /// * The user crosses any global or user specific withdrawal limits.
    #[storage(read, write)]
    fn withdraw(
        receiver: Identity,
        underlying_asset: AssetId,
        vault_sub_id: SubId,
    ) -> u64 {
        let nft_asset = AssetId::from(vault_sub_id);
        require(
            storage
                .state
                .get(nft_asset)
                .try_read()
                .unwrap_or(State::Uninitialized) == State::Buyback,
            StateError::InvalidState,
        );

        let f_nft_asset_sub_id = sha256((BASE_ASSET_ID, vault_sub_id));
        let f_nft_asset = AssetId::new(contract_id(), f_nft_asset_sub_id);
        require(msg_asset_id() == f_nft_asset, WithdrawError::InvalidAsset);

        let sent_amount = msg_amount();
        let rate = sent_amount * storage.share_price.get(nft_asset).read();

        burn(f_nft_asset_sub_id, sent_amount);
        transfer(receiver, BASE_ASSET_ID, rate);

        log(Withdraw {
            caller: msg_sender().unwrap(),
            receiver,
            underlying_asset: nft_asset,
            vault_sub_id,
            withdrawn_amount: sent_amount,
            burned_shares: sent_amount,
        });

        rate
    }

    /// Returns the amount of managed assets of the given asset.
    ///
    /// # Arguments
    ///
    /// * `underlying_asset`: [AssetId] - The asset for which the amount of managed assets should be returned.
    /// * `vault_sub_id`: [SubId] - The SubId of the vault.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of managed assets of the given asset.
    #[storage(read)]
    fn managed_assets(underlying_asset: AssetId, vault_sub_id: SubId) -> u64 {
        0
    }

    /// Returns the maximum amount of assets that can be deposited into the contract, for the given asset.
    ///
    /// # Additional Information
    ///
    /// Must account for any user or global limits.
    ///
    /// # Arguments
    ///
    /// * `receiver`: [Identity] - The hypothetical receiver of the shares.
    /// * `underlying_asset`: [AssetId] - The asset for which the maximum amount of depositable assets should be returned.
    /// * `vault_sub_id`: [SubId] - The SubId of the vault.
    ///
    /// # Returns
    ///
    /// * [Some(u64)] - The maximum amount of assets that can be deposited into the contract, for the given asset.
    /// * [None] - If the asset is not supported by the contract.
    #[storage(read)]
    fn max_depositable(
        receiver: Identity,
        underlying_asset: AssetId,
        vault_sub_id: SubId,
    ) -> Option<u64> {
        require(
            underlying_asset == BASE_ASSET_ID,
            DepositError::InvalidAsset,
        );

        let nft_asset = AssetId::from(vault_sub_id);
        match storage.state.get(nft_asset).try_read() {
            Some(state) => {
                match state {
                    State::Distribution => {
                        let f_nft_asset_sub_id = sha256((BASE_ASSET_ID, vault_sub_id));
                        let f_nft_asset = AssetId::new(contract_id(), f_nft_asset_sub_id);
                        let minted_amount = storage.minted_shares.get(f_nft_asset).read();
                        let total_supply = storage.total_supply.get(f_nft_asset).read();
                        let rate = storage.share_price.get(nft_asset).read();
                        Some((total_supply - minted_amount) * rate)
                    },
                    _ => Some(0),
                }
            },
            None => None,
        }
    }

    /// Returns the maximum amount of assets that can be withdrawn from the contract, for the given asset.
    ///
    /// # Additional Information
    ///
    /// Must account for any global limits.
    ///
    /// # Arguments
    ///
    /// * `underlying_asset`: [AssetId] - The asset for which the maximum amount of withdrawable assets should be returned.
    /// * `vault_sub_id`: [SubId] - The SubId of the vault.
    ///
    /// # Returns
    ///
    /// * [Some(u64)] - The maximum amount of assets that can be withdrawn from the contract, for the given asset.
    /// * [None] - If the asset is not supported by the contract.
    #[storage(read)]
    fn max_withdrawable(underlying_asset: AssetId, vault_sub_id: SubId) -> Option<u64> {
        require(
            underlying_asset == BASE_ASSET_ID,
            WithdrawError::InvalidAsset,
        );

        let nft_asset = AssetId::from(vault_sub_id);
        match storage.state.get(nft_asset).try_read() {
            Some(state) => {
                match state {
                    State::Buyback => {
                        let f_nft_asset_sub_id = sha256((BASE_ASSET_ID, vault_sub_id));
                        let f_nft_asset = AssetId::new(contract_id(), f_nft_asset_sub_id);
                        let minted_amount = storage.minted_shares.get(f_nft_asset).read();
                        let total_supply = storage.total_supply.get(f_nft_asset).read();
                        let rate = storage.share_price.get(nft_asset).read();
                        Some((total_supply - (total_supply - minted_amount)) * rate)
                    },
                    _ => Some(0),
                }
            },
            None => None,
        }
    }
}

impl SRC20 for Contract {
    #[storage(read)]
    fn total_assets() -> u64 {
        storage.total_assets.read()
    }

    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        storage.total_supply.get(asset).try_read()
    }

    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        match storage.total_supply.get(asset).try_read() {
            Some(_) => Some(String::from_ascii_str(from_str_array(NAME))),
            None => None,
        }
    }

    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        match storage.total_supply.get(asset).try_read() {
            Some(_) => Some(String::from_ascii_str(from_str_array(SYMBOL))),
            None => None,
        }
    }

    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        match storage.total_supply.get(asset).try_read() {
            Some(_) => Some(DECIMALS),
            None => None,
        }
    }
}
