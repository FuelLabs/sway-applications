contract;

mod errors;

use errors::{DepositError, SubIdError, WithdrawError};
use standards::{
    src20::{
        SetDecimalsEvent,
        SetNameEvent,
        SetSymbolEvent,
        SRC20,
        TotalSupplyEvent,
    },
    src6::{
        Deposit,
        SRC6,
        Withdraw,
    },
};
use std::{
    asset::{
        burn,
        mint_to,
        transfer,
    },
    call_frames::msg_asset_id,
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
    /// The total number of unique assets minted by this contract.
    ///
    /// # Additional Information
    ///
    /// This is the number of NFTs that have ever been deposited.
    total_assets: u64 = 0,
    /// The validity of an asset as a share minted by this contract.
    ///
    /// # Additional Information
    ///
    /// maps(Fractional NFT Share) => valid share
    vault_asset: StorageMap<AssetId, bool> = StorageMap {},
}

impl SRC6 for Contract {
    /// Deposits a NFT into the contract and mints F-NFT shares to the receiver.
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
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `2`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src6::SRC6;
    /// use std::constants::DEFAULT_SUB_ID;
    ///
    /// fn foo(vault_contract: ContractId, receiver: Identity, nft: AssetId) {
    ///     let vault_abi = abi(SRC6, vault_contract);
    ///     let _ = vault_abi {
    ///         gas: 10000,
    ///         coins: 1,
    ///         asset_id: nft,
    ///     }.deposit(receiver, DEFAULT_SUB_ID);
    /// }
    /// ```
    #[payable]
    #[storage(read, write)]
    fn deposit(receiver: Identity, vault_sub_id: SubId) -> u64 {
        require(vault_sub_id == SubId::zero(), SubIdError::InvalidSubId);
        require(msg_amount() == 1, DepositError::InvalidSRC20NFT);

        let nft = msg_asset_id();
        let f_nft_asset_sub_id = sha256((nft, vault_sub_id));
        let f_nft_asset = AssetId::new(ContractId::this(), f_nft_asset_sub_id);
        let sender = msg_sender().unwrap();

        storage.total_assets.write(storage.total_assets.read() + 1);
        storage.vault_asset.insert(f_nft_asset, true);
        mint_to(receiver, f_nft_asset_sub_id, SHARES);
        TotalSupplyEvent::new(f_nft_asset, SHARES, sender).log();

        log(Deposit {
            caller: sender,
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
    /// * When the amount sent isn't all shares of an NFT.
    /// * When the asset is not shares to an NFT.
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src6::SRC6;
    /// use std::constants::DEFAULT_SUB_ID;
    ///
    /// fn foo(vault_contract: ContractId, receiver: Identity, nft: AssetId, shares: AssetId) {
    ///     let vault_abi = abi(SRC6, vault_contract);
    ///     let _ = vault_abi {
    ///         gas: 10000,
    ///         coins: 100_000_000,
    ///         asset_id: shares,
    ///     }.withdraw(receiver, nft, DEFAULT_SUB_ID);
    /// }
    /// ```
    #[payable]
    #[storage(read, write)]
    fn withdraw(
        receiver: Identity,
        underlying_asset: AssetId,
        vault_sub_id: SubId,
    ) -> u64 {
        require(vault_sub_id == ZERO_B256, SubIdError::InvalidSubId);

        let sent_amount = msg_amount();
        let sender = msg_sender().unwrap();
        require(sent_amount == SHARES, WithdrawError::AllSharesNotReturned);

        let f_nft_asset_sub_id = sha256((underlying_asset, vault_sub_id));
        let f_nft_asset = AssetId::new(ContractId::this(), f_nft_asset_sub_id);
        require(msg_asset_id() == f_nft_asset, WithdrawError::InvalidAsset);

        burn(f_nft_asset_sub_id, SHARES);
        TotalSupplyEvent::new(f_nft_asset, 0, sender).log();

        transfer(receiver, underlying_asset, 1);

        log(Withdraw {
            caller: sender,
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
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src6::SRC6;
    /// use std::constants::DEFAULT_SUB_ID;
    ///
    /// fn foo(vault_contract: ContractId, nft: AssetId) {
    ///     let vault_abi = abi(SRC6, vault_contract);
    ///     let managed_assets = vault_abi.managed_assets(nft, DEFAULT_SUB_ID);
    ///     assert(managed_assets == 1);
    /// }
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src6::SRC6;
    /// use std::constants::DEFAULT_SUB_ID;
    ///
    /// fn foo(vault_contract: ContractId, receiver: Identity, nft: AssetId) {
    ///     let vault_abi = abi(SRC6, vault_contract);
    ///     let max_depositable = vault_abi.max_depositable(receiver, nft, DEFAULT_SUB_ID);
    ///     assert(max_depositable == Some(0));
    /// }
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src6::SRC6;
    /// use std::constants::DEFAULT_SUB_ID;
    ///
    /// fn foo(vault_contract: ContractId, nft: AssetId) {
    ///     let vault_abi = abi(SRC6, vault_contract);
    ///     let max_withdrawable = vault_abi.max_withdrawable(nft, DEFAULT_SUB_ID);
    ///     assert(max_withdrawable == Some(1));
    /// }
    /// ```
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
    /// Returns the total number of differentiating Fractional NFTs minted by this vault.
    ///
    /// # Returns
    ///
    /// * [u64] - The number of assets that this contract has minted.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    ///
    /// fn foo(vault_contract: ContractId) {
    ///     let vault_abi = abi(SRC20, vault_contract);
    ///     let total_assets = vault_abi.total_assets();
    ///     assert(total_assets != 0);
    /// }
    /// ```
    #[storage(read)]
    fn total_assets() -> u64 {
        storage.total_assets.read()
    }

    /// Returns the total supply of coins for an Fractional NFT share asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the total supply.
    ///
    /// # Returns
    ///
    /// * [Option<u64>] - The total supply of coins for `asset`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    ///
    /// fn foo(vault_contract: ContractId, share: AssetId) {
    ///     let vault_abi = abi(SRC20, vault_contract);
    ///     let total_supply = vault_abi.total_supply(share);
    ///     assert(total_supply.unwrap() != 0);
    /// }
    /// ```
    #[storage(read)]
    fn total_supply(asset: AssetId) -> Option<u64> {
        match storage.vault_asset.get(asset).try_read() {
            Some(_) => Some(SHARES),
            None => None,
        }
    }

    /// Returns the name of the asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the name.
    ///
    /// # Returns
    ///
    /// * [Option<String>] - The name of `asset`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(vault_contract: ContractId, share: AssetId) {
    ///     let vault_abi = abi(SRC20, vault_contract);
    ///     let name = vault_abi.name(share);
    ///     assert(name.is_some());
    /// }
    /// ```
    #[storage(read)]
    fn name(asset: AssetId) -> Option<String> {
        match storage.vault_asset.get(asset).try_read() {
            Some(_) => Some(String::from_ascii_str(from_str_array(NAME))),
            None => None,
        }
    }

    /// Returns the symbol of the asset.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the symbol.
    ///
    /// # Returns
    ///
    /// * [Option<String>] - The symbol of `asset`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    /// use std::string::String;
    ///
    /// fn foo(vault_contract: ContractId, share: AssetId) {
    ///     let vault_abi = abi(SRC20, vault_contract);
    ///     let symbol = vault_abi.symbol(share);
    ///     assert(symbol.is_some());
    /// }
    /// ```
    #[storage(read)]
    fn symbol(asset: AssetId) -> Option<String> {
        match storage.vault_asset.get(asset).try_read() {
            Some(_) => Some(String::from_ascii_str(from_str_array(SYMBOL))),
            None => None,
        }
    }

    /// Returns the number of decimals the asset uses.
    ///
    /// # Additional Information
    ///
    /// e.g. 8, means to divide the coins amount by 100000000 to get its user representation.
    ///
    /// # Arguments
    ///
    /// * `asset`: [AssetId] - The asset of which to query the decimals.
    ///
    /// # Returns
    ///
    /// * [Option<u8>] - The decimal precision used by `asset`.
    ///
    /// # Number of Storage Accesses
    ///
    /// * Reads: `1`
    ///
    /// # Examples
    ///
    /// ```sway
    /// use src20::SRC20;
    ///
    /// fn foo(vault_contract: ContractId, share: AssetId) {
    ///     let vault_abi = abi(SRC20, vault_contract);
    ///     let decimals = vault_abi.decimals(share);
    ///     assert(decimals.unwrap() == 8u8);
    /// }
    /// ```
    #[storage(read)]
    fn decimals(asset: AssetId) -> Option<u8> {
        match storage.vault_asset.get(asset).try_read() {
            Some(_) => Some(DECIMALS),
            None => None,
        }
    }
}

abi EmitSRC20Events {
    fn emit_src20_events(asset: AssetId);
}

impl EmitSRC20Events for Contract {
    fn emit_src20_events(asset: AssetId) {
        // Metadata that is stored as a configurable should only be emitted once.
        let sender = msg_sender().unwrap();
        let name = Some(String::from_ascii_str(from_str_array(NAME)));
        let symbol = Some(String::from_ascii_str(from_str_array(SYMBOL)));

        SetNameEvent::new(asset, name, sender).log();
        SetSymbolEvent::new(asset, symbol, sender).log();
        SetDecimalsEvent::new(asset, DECIMALS, sender).log();
        TotalSupplyEvent::new(asset, SHARES, sender).log();
    }
}
