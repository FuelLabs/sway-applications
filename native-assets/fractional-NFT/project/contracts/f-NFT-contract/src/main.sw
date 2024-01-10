contract;

mod errors;

use errors::{
    DepositError,
    SubIdError,
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
    constants::ZERO_B256,
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
    /// The number of shares per NFT
    SHARES: u64 = 100_000_000,
}

storage {
    total_assets: u64 = 0,
    vault_asset: StorageMap<AssetId, bool> = StorageMap {},
}

impl SRC6 for Contract {
    /// Deposits a NFT into the contract and mints NFT shares to the receiver.
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
    /// * When the `vault_sub_id` is the not ZERO_B256
    /// * When more than 1 asset amount is sent.
    #[payable]
    #[storage(read, write)]
    fn deposit(receiver: Identity, vault_sub_id: SubId) -> u64 {
        require(vault_sub_id == ZERO_B256, SubIdError::InvalidSubId);

        let nft = msg_asset_id();
        require(msg_amount() == 1, DepositError::InvalidSRC20NFT);

        let f_nft_asset_sub_id = sha256((nft, vault_sub_id));
        let f_nft_asset = AssetId::new(contract_id(), f_nft_asset_sub_id);

        storage.total_assets.write(storage.total_assets.read() + 1);
        storage.vault_asset.insert(f_nft_asset, true);
        mint_to(receiver, f_nft_asset_sub_id, SHARES);

        log(Deposit {
            caller: msg_sender().unwrap(),
            receiver: receiver,
            underlying_asset: nft,
            vault_sub_id: vault_sub_id,
            deposited_amount: 1,
            minted_shares: SHARES,
        });

        SHARES
    }

    /// Burns all shares from the sender and transfers the NFT to the receiver.
    ///
    /// # Additional Information
    ///
    /// * All shares must be returned in the same transaction.
    ///
    /// # Arguments
    ///
    /// * `receiver`: [Identity] - The receiver of the NFT.
    /// * `underlying_asset`: [AssetId] - The asset for which the shares should be burned.
    /// * `vault_sub_id`: [SubId] - The SubId of the vault.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of NFTs withdrawn.
    ///
    /// # Reverts
    ///
    /// * When the `vault_sub_id` is the not ZERO_B256.
    /// * When the asset is not shares to an NFT.
    /// * When the amount sent isn't all shares of an NFT.
    #[payable]
    #[storage(read, write)]
    fn withdraw(
        receiver: Identity,
        underlying_asset: AssetId,
        vault_sub_id: SubId,
    ) -> u64 {
        require(vault_sub_id == ZERO_B256, SubIdError::InvalidSubId);

        let sent_amount = msg_amount();
        let f_nft_asset_sub_id = sha256((underlying_asset, vault_sub_id));
        let f_nft_asset = AssetId::new(contract_id(), f_nft_asset_sub_id);
        require(msg_asset_id() == f_nft_asset, WithdrawError::InvalidAsset);
        require(sent_amount == SHARES, WithdrawError::AllSharesNotReturned);

        burn(f_nft_asset_sub_id, SHARES);
        transfer(receiver, underlying_asset, 1);

        log(Withdraw {
            caller: msg_sender().unwrap(),
            receiver,
            underlying_asset,
            vault_sub_id,
            withdrawn_amount: SHARES,
            burned_shares: SHARES,
        });

        1
    }

    /// Returns the amount of managed assets of the given asset.
    ///
    /// # Arguments
    ///
    /// * `underlying_asset`: [AssetId] - The NFT for which the amount of managed assets should be returned.
    /// * `vault_sub_id`: [SubId] - The SubId of the vault.
    ///
    /// # Returns
    ///
    /// * [u64] - The amount of managed assets of the given asset.
    #[storage(read)]
    fn managed_assets(underlying_asset: AssetId, vault_sub_id: SubId) -> u64 {
        if vault_sub_id != ZERO_B256 {
            return 0;
        }

        match this_balance(underlying_asset) {
            1 => 1,
            _ => 0,
        }
    }

    /// Returns the maximum amount of assets that can be deposited into the contract, for the given NFT.
    ///
    /// # Additional Information
    ///
    /// Maximum is `Some(1)` as NFTs only have a supply of one.
    ///
    /// # Arguments
    ///
    /// * `receiver`: [Identity] - The hypothetical receiver of the shares.
    /// * `underlying_asset`: [AssetId] - The NFT for which the maximum amount of depositable assets should be returned.
    /// * `vault_sub_id`: [SubId] - The SubId of the vault.
    ///
    /// # Returns
    ///
    /// * [Some(u64)] - The maximum amount of assets that can be deposited into the contract, for the given NFT.
    /// * [None] - If the asset is not supported by the contract.
    #[storage(read)]
    fn max_depositable(
        receiver: Identity,
        underlying_asset: AssetId,
        vault_sub_id: SubId,
    ) -> Option<u64> {
        if vault_sub_id != ZERO_B256 {
            return None;
        }

        match this_balance(underlying_asset) {
            0 => Some(1),
            1 => Some(0),
            _ => None,
        }
    }

    /// Returns the maximum amount of assets that can be withdrawn from the contract, for the given NFT.
    ///
    /// # Additional Information
    ///
    /// Maximum is `Some(1)` as NFTs only have a supply of one.
    ///
    /// # Arguments
    ///
    /// * `underlying_asset`: [AssetId] - The NFT for which the maximum amount of withdrawable assets should be returned.
    /// * `vault_sub_id`: [SubId] - The SubId of the vault.
    ///
    /// # Returns
    ///
    /// * [Some(u64)] - The maximum amount of assets that can be withdrawn from the contract, for the given NFT.
    /// * [None] - If the asset is not supported by the contract.
    #[storage(read)]
    fn max_withdrawable(underlying_asset: AssetId, vault_sub_id: SubId) -> Option<u64> {
        if vault_sub_id != ZERO_B256 {
            return None;
        }
        
        match this_balance(underlying_asset) {
            0 => Some(0),
            1 => Some(1),
            _ => None,
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
        match storage.vault_asset.get(asset).try_read() {
            Some(_) => Some(SHARES),
            None => None,
        }
    }

    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        match storage.vault_asset.get(asset).try_read() {
            Some(_) => Some(String::from_ascii_str(from_str_array(NAME))),
            None => None,
        }
    }

    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        match storage.vault_asset.get(asset).try_read() {
            Some(_) => Some(String::from_ascii_str(from_str_array(SYMBOL))),
            None => None,
        }
    }

    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        match storage.vault_asset.get(asset).try_read() {
            Some(_) => Some(DECIMALS),
            None => None,
        }
    }
}
