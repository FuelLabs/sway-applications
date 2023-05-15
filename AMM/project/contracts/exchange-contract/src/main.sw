contract;

mod errors;
mod events;
mod utils;

use ::errors::{InitError, InputError, TransactionError};
use ::events::{
    AddLiquidityEvent,
    DefineAssetPairEvent,
    DepositEvent,
    RemoveLiquidityEvent,
    SwapEvent,
    WithdrawEvent,
};
use libraries::{
    data_structures::{
        Asset,
        AssetPair,
        PoolInfo,
        PreviewAddLiquidityInfo,
        PreviewSwapInfo,
        RemoveLiquidityInfo,
    },
    Exchange,
};
use std::{
    auth::msg_sender,
    block::height,
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    context::msg_amount,
    math::*,
    token::{
        burn,
        mint,
        transfer,
    },
};
use ::utils::{
    determine_assets,
    maximum_input_for_exact_output,
    minimum_output_given_exact_input,
    proportional_value,
};

configurable {
    LIQUIDITY_MINER_FEE: u64 = 333,
    MINIMUM_LIQUIDITY: u64 = 100,
}

storage {
    /// Deposit amounts per (depositor, asset) that can be used to add liquidity or be withdrawn.
    deposits: StorageMap<(Identity, ContractId), u64> = StorageMap {},
    /// Total amount of the liquidity pool asset that has a unique identifier different from the identifiers of assets on either side of the pool.
    liquidity_pool_supply: u64 = 0,
    /// The unique identifiers that make up the pool that can be set only once using the `constructor`.
    pair: Option<AssetPair> = Option::None,
}

impl Exchange for Contract {
    /// Mint liquidity pool asset at current ratio and transfer to the sender.
    ///
    /// ### Additional Information
    ///
    /// When liquidity is added for the first time, all deposited amounts are used to determine the ratio.
    /// When adding further liquidity, extra amounts of deposits are refunded.
    ///
    /// ### Arguments
    ///
    /// * `desired_liquidity`: `u64` - The minimum amount of liquidity to add.
    /// * `deadline`: `u64` - The limit on block height for operation.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the current block height is not less than `deadline`.
    /// * When the `msg_amount` with function call is not 0.
    /// * When the `desired_liquidity` is less than `MINIMUM_LIQUIDITY`.
    /// * When asset A or B deposits are 0.
    /// * When calculated liquidity to add is less than `desired liquidity`.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `5`
    /// * Writes: `6`
    #[storage(read, write)]
    fn add_liquidity(desired_liquidity: u64, deadline: u64) -> u64 {
        require(storage.pair.read().is_some(), InitError::AssetPairNotSet);
        require(deadline > height(), InputError::DeadlinePassed(deadline));
        require(MINIMUM_LIQUIDITY <= desired_liquidity, InputError::CannotAddLessThanMinimumLiquidity(desired_liquidity));

        let sender = msg_sender().unwrap();
        let reserves = storage.pair.read().unwrap();

        let (deposit_a, deposit_b) = (
            storage.deposits.get((sender, reserves.a.id)).try_read().unwrap_or(0),
            storage.deposits.get((sender, reserves.b.id)).try_read().unwrap_or(0),
        );
        let deposits = AssetPair::new(Asset::new(reserves.a.id, deposit_a), Asset::new(reserves.b.id, deposit_b));

        // checking this because this will either result in a math error or adding no liquidity at all.
        require(deposits.a.amount != 0, TransactionError::ExpectedNonZeroDeposit(deposits.a.id));
        require(deposits.b.amount != 0, TransactionError::ExpectedNonZeroDeposit(deposits.b.id));

        let total_liquidity = storage.liquidity_pool_supply.read();

        let mut added_assets = AssetPair::new(Asset::new(reserves.a.id, 0), Asset::new(reserves.b.id, 0));
        let mut added_liquidity = 0;

        // adding liquidity for the first time
        // use up all the deposited amounts of assets to determine the ratio.
        if reserves.a.amount == 0 && reserves.b.amount == 0 {
            added_liquidity = (deposits.a.amount * deposits.b.amount).sqrt();
            require(desired_liquidity <= added_liquidity, TransactionError::DesiredAmountTooHigh(desired_liquidity));
            added_assets.a.amount = deposits.a.amount;
            added_assets.b.amount = deposits.b.amount;

            // add amounts to reserves.
            storage.pair.write(Option::Some(added_assets));

            // mint liquidity pool asset and transfer to sender.
            mint(added_liquidity);
            storage.liquidity_pool_supply.write(added_liquidity);
            transfer(added_liquidity, contract_id(), sender);
        } else { // adding further liquidity based on current ratio.
            // attempt to add liquidity by using up the deposited asset A amount.
            let b_to_attempt = proportional_value(deposits.a.amount, reserves.b.amount, reserves.a.amount);

            // continue adding based on asset A if deposited asset B amount is sufficient.
            if b_to_attempt <= deposits.b.amount {
                added_liquidity = proportional_value(b_to_attempt, total_liquidity, reserves.b.amount);
                require(desired_liquidity <= added_liquidity, TransactionError::DesiredAmountTooHigh(desired_liquidity));
                added_assets.a.amount = deposits.a.amount;
                added_assets.b.amount = b_to_attempt;
            } else { // attempt to add liquidity by using up the deposited asset B amount.
                let a_to_attempt = proportional_value(deposits.b.amount, reserves.a.amount, reserves.b.amount);
                added_liquidity = proportional_value(a_to_attempt, total_liquidity, reserves.a.amount);
                require(desired_liquidity <= added_liquidity, TransactionError::DesiredAmountTooHigh(desired_liquidity));
                added_assets.a.amount = a_to_attempt;
                added_assets.b.amount = deposits.b.amount;
            }

            // add new asset amounts to reserves.
            storage.pair.write(Option::Some(reserves + added_assets));

            // mint liquidity pool asset and transfer to sender.
            mint(added_liquidity);
            storage.liquidity_pool_supply.write(total_liquidity + added_liquidity);
            transfer(added_liquidity, contract_id(), sender);

            // transfer remaining deposit amounts back to the sender.
            let refund = deposits - added_assets;

            if refund.a.amount > 0 {
                transfer(refund.a.amount, refund.a.id, sender);
            }

            if refund.b.amount > 0 {
                transfer(refund.b.amount, refund.b.id, sender);
            }
        }

        storage.deposits.insert((sender, deposits.a.id), 0);
        storage.deposits.insert((sender, deposits.b.id), 0);

        log(AddLiquidityEvent {
            added_assets,
            liquidity: Asset::new(contract_id(), added_liquidity),
        });

        added_liquidity
    }

    /// Initialize contract by specifying the asset pair that makes up the pool.
    ///
    /// # Arguments
    ///
    /// - `asset_a` - unique identifier of one asset
    /// - `asset_b` - unique identifier of the other asset
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`
    /// * When the passed pair describes identical assets


    /// Initialize contract by specifying the asset pair that makes up the pool.
    ///
    /// ### Arguments
    ///
    /// * `asset_a`: `ContractId` - The unique identifier of one asset.
    /// * `asset_b`: `ContractId` - The unique identifier of the other asset.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the passed pair describes identical assets.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    #[storage(read, write)]
    fn constructor(asset_a: ContractId, asset_b: ContractId) {
        require(storage.pair.read().is_none(), InitError::AssetPairAlreadySet);
        require(asset_a != asset_b, InitError::IdenticalAssets);

        storage.pair.write(Option::Some(AssetPair::new(Asset::new(asset_a, 0), Asset::new(asset_b, 0))));

        log(DefineAssetPairEvent {
            asset_a_id: asset_a,
            asset_b_id: asset_b,
        });
    }

    /// Deposit asset to later add to the liquidity pool or withdraw.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `4`
    /// * Writes: `1`
    #[payable]
    #[storage(read, write)]
    fn deposit() {
        require(storage.pair.read().is_some(), InitError::AssetPairNotSet);

        let deposit_asset = msg_asset_id();

        require(deposit_asset == storage.pair.read().unwrap().a.id || deposit_asset == storage.pair.read().unwrap().b.id, InputError::InvalidAsset);

        let sender = msg_sender().unwrap();
        let amount = msg_amount();
        let new_balance = storage.deposits.get((sender, deposit_asset)).try_read().unwrap_or(0) + amount;
        storage.deposits.insert((sender, deposit_asset), new_balance);

        log(DepositEvent {
            deposited_asset: Asset::new(deposit_asset, amount),
            new_balance,
        });
    }

    /// Burn liquidity pool asset at current ratio and transfer asset A and asset B to the sender.
    ///
    /// ### Arguments
    ///
    /// * `min_asset_a`: `u64` - The minimum amount of asset A to receive after burn.
    /// * `min_asset_b`: `u64` - minimum amount of asset B to receive after burn.
    /// * `deadline`: `u64` - The limit on block height for operation.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When there are no liquidity pool assets to burn.
    /// * When the `msg_asset_id` does not identify the liquidity pool asset.
    /// * When `min_asset_a` or `min_asset_b` is 0.
    /// * When the current block height is not less than `deadline`.
    /// * When the `msg_amount` with function call is 0.
    /// * When the minimum amounts for asset A and asset B to receive after burn cannot be satisfied.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `3`
    /// * Writes: `2`
    #[payable]
    #[storage(read, write)]
    fn remove_liquidity(min_asset_a: u64, min_asset_b: u64, deadline: u64) -> RemoveLiquidityInfo {
        require(storage.pair.read().is_some(), InitError::AssetPairNotSet);

        let total_liquidity = storage.liquidity_pool_supply.read();
        require(total_liquidity > 0, TransactionError::NoLiquidityToRemove);

        let reserves = storage.pair.read().unwrap();

        require(min_asset_a > 0, InputError::ExpectedNonZeroParameter(reserves.a.id));
        require(min_asset_b > 0, InputError::ExpectedNonZeroParameter(reserves.b.id));
        require(deadline > height(), InputError::DeadlinePassed(deadline));

        let burned_liquidity = Asset::new(contract_id(), msg_amount());

        require(burned_liquidity.id == msg_asset_id(), InputError::InvalidAsset);
        require(burned_liquidity.amount > 0, InputError::ExpectedNonZeroAmount(burned_liquidity.id));

        let mut removed_assets = AssetPair::new(Asset::new(reserves.a.id, 0), Asset::new(reserves.b.id, 0));
        removed_assets.a.amount = proportional_value(burned_liquidity.amount, reserves.a.amount, total_liquidity);
        removed_assets.b.amount = proportional_value(burned_liquidity.amount, reserves.b.amount, total_liquidity);

        require(removed_assets.a.amount >= min_asset_a, TransactionError::DesiredAmountTooHigh(min_asset_a));
        require(removed_assets.b.amount >= min_asset_b, TransactionError::DesiredAmountTooHigh(min_asset_b));

        burn(burned_liquidity.amount);
        storage.liquidity_pool_supply.write(total_liquidity - burned_liquidity.amount);
        storage.pair.write(Option::Some(reserves - removed_assets));

        let sender = msg_sender().unwrap();
        transfer(removed_assets.a.amount, removed_assets.a.id, sender);
        transfer(removed_assets.b.amount, removed_assets.b.id, sender);

        log(RemoveLiquidityEvent {
            removed_reserve: removed_assets,
            burned_liquidity,
        });

        RemoveLiquidityInfo {
            removed_amounts: removed_assets,
            burned_liquidity,
        }
    }

    /// Swap forwarded amount of forwarded asset for other asset and transfer to sender.
    ///
    /// ### Arguments
    ///
    /// * `min_output`: `Option<u64>` - The minimum output required (to protect against excessive slippage).
    /// * `deadline`: `u64` - The limit on block height for operation.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    /// * When the current block height is not less than `deadline`.
    /// * When the `msg_amount` with function call is 0.
    /// * When `min_output` is provided and is lower than the output amount.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    #[payable]
    #[storage(read, write)]
    fn swap_exact_input(min_output: Option<u64>, deadline: u64) -> u64 {
        require(deadline >= height(), InputError::DeadlinePassed(deadline));

        let reserves = storage.pair.read();
        let (mut input_asset, mut output_asset) = determine_assets(msg_asset_id(), reserves);

        let exact_input = msg_amount();
        require(exact_input > 0, InputError::ExpectedNonZeroAmount(input_asset.id));

        let bought = minimum_output_given_exact_input(exact_input, input_asset.amount, output_asset.amount, LIQUIDITY_MINER_FEE);

        if min_output.is_some() {
            require(bought >= min_output.unwrap(), TransactionError::DesiredAmountTooHigh(min_output.unwrap()));
        }

        transfer(bought, output_asset.id, msg_sender().unwrap());

        input_asset.amount = input_asset.amount + exact_input;
        output_asset.amount = output_asset.amount - bought;
        storage.pair.write(Option::Some(AssetPair::new(input_asset, output_asset).sort(reserves.unwrap())));

        log(SwapEvent {
            input: input_asset,
            output: output_asset,
        });

        bought
    }

    /// Swap forwarded asset for `exact_output_amount` of other asset and transfer to sender.
    ///
    /// ### Arguments
    ///
    /// * `output`: `u64` - The exact output amount to receive.
    /// * `deadline`: `u64` - The limit on block height for operation.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    /// * When `output` is 0.
    /// * When the current block height is not less than ` deadline `.
    /// * When the `msg_amount` with function call is 0.
    /// * When the `msg_amount` is insufficient for swap.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `1`
    /// * Writes: `1`
    #[payable]
    #[storage(read, write)]
    fn swap_exact_output(output: u64, deadline: u64) -> u64 {
        let reserves = storage.pair.read();
        let (mut input_asset, mut output_asset) = determine_assets(msg_asset_id(), reserves);

        require(deadline > height(), InputError::DeadlinePassed(deadline));
        require(output > 0, InputError::ExpectedNonZeroParameter(output_asset.id));
        require(output <= output_asset.amount, TransactionError::InsufficientReserve(output_asset.id));

        let input_amount = msg_amount();
        require(input_amount > 0, InputError::ExpectedNonZeroAmount(input_asset.id));

        let sold = maximum_input_for_exact_output(output, input_asset.amount, output_asset.amount, LIQUIDITY_MINER_FEE);

        require(sold > 0, TransactionError::DesiredAmountTooLow(output));
        require(input_amount >= sold, TransactionError::DesiredAmountTooHigh(input_amount));

        let sender = msg_sender().unwrap();

        let refund = input_amount - sold;
        if refund > 0 {
            transfer(refund, input_asset.id, sender);
        };

        transfer(output, output_asset.id, sender);

        input_asset.amount = input_asset.amount + sold;
        output_asset.amount = output_asset.amount - output;
        storage.pair.write(Option::Some(AssetPair::new(input_asset, output_asset).sort(reserves.unwrap())));

        log(SwapEvent {
            input: input_asset,
            output: output_asset,
        });

        sold
    }

    ///  Withdraw coins that have not been added to a liquidity pool yet.
    ///
    /// ### Arguments
    ///
    /// * `asset`: `Asset` - The id and amount of asset to withdraw.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    /// * When the deposited amount by the sender stored in the contract is insufficient.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `4`
    /// * Writes: `1`
    #[storage(read, write)]
    fn withdraw(asset: Asset) {
        require(storage.pair.read().is_some(), InitError::AssetPairNotSet);

        require(asset.id == storage.pair.read().unwrap().a.id || asset.id == storage.pair.read().unwrap().b.id, InputError::InvalidAsset);

        let sender = msg_sender().unwrap();
        let deposited_amount = storage.deposits.get((sender, asset.id)).try_read().unwrap_or(0);

        require(deposited_amount >= asset.amount, TransactionError::DesiredAmountTooHigh(asset.amount));

        let new_amount = deposited_amount - asset.amount;
        storage.deposits.insert((sender, asset.id), new_amount);
        transfer(asset.amount, asset.id, sender);

        log(WithdrawEvent {
            withdrawn_asset: asset,
            remaining_balance: new_amount,
        });
    }

    /// Get current balance of the sender for a given asset on the contract.
    ///
    /// ### Arguments
    ///
    /// * `asset_id`: `ContractId` - The id of the asset to get balance of.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `4`
    #[storage(read)]
    fn balance(asset_id: ContractId) -> u64 {
        require(storage.pair.read().is_some(), InitError::AssetPairNotSet);
        require(asset_id == storage.pair.read().unwrap().a.id || asset_id == storage.pair.read().unwrap().b.id, InputError::InvalidAsset);

        storage.deposits.get((msg_sender().unwrap(), asset_id)).try_read().unwrap_or(0)
    }

    /// Get the pool info of the exchange contract.
    ///
    /// ### Additional Information
    ///
    /// The pool info consists of:
    /// - Identifier of asset A,
    /// - Identifier of asset B,
    /// - Asset A amount in reserves,
    /// - Asset B amount in reserves,
    /// - Liquidity pool asset supply amount.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `3`
    #[storage(read)]
    fn pool_info() -> PoolInfo {
        require(storage.pair.read().is_some(), InitError::AssetPairNotSet);

        PoolInfo {
            reserves: storage.pair.read().unwrap(),
            liquidity: storage.liquidity_pool_supply.read(),
        }
    }

    ///  Get the preview info of adding liquidity.
    ///
    /// ### Additional Information
    ///
    /// The preview info consists of:
    /// - Other asset amount to input for desired liquidity,
    /// - Liquidity pool asset amount to be received.
    ///
    /// ### Arguments
    ///
    /// * `asset`: `Asset` - The id and amount of asset to add.
    ///
     /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `5`
    #[storage(read)]
    fn preview_add_liquidity(asset: Asset) -> PreviewAddLiquidityInfo {
        require(storage.pair.read().is_some(), InitError::AssetPairNotSet);

        let sender = msg_sender().unwrap();
        let total_liquidity = storage.liquidity_pool_supply.read();
        let reserves = storage.pair.read().unwrap();

        let (deposit_a, deposit_b) = (
            storage.deposits.get((sender, reserves.a.id)).try_read().unwrap_or(0),
            storage.deposits.get((sender, reserves.b.id)).try_read().unwrap_or(0),
        );
        let deposits = AssetPair::new(Asset::new(reserves.a.id, deposit_a), Asset::new(reserves.b.id, deposit_b));

        let mut added_assets = AssetPair::new(Asset::new(reserves.a.id, 0), Asset::new(reserves.b.id, 0));
        let mut added_liquidity = 0;

        if total_liquidity == 0 {
            added_assets.a.amount = if deposits.a.amount == 0 {
                asset.amount
            } else {
                deposits.a.amount
            };
            added_assets.b.amount = if deposits.b.amount == 0 {
                asset.amount
            } else {
                deposits.b.amount
            };
            added_liquidity = (added_assets.a.amount * added_assets.b.amount).sqrt()
        } else {
            if asset.id == reserves.a.id {
                added_assets.a.amount = asset.amount;
                added_assets.b.amount = proportional_value(asset.amount, reserves.b.amount, reserves.a.amount);
            } else {
                added_assets.a.amount = proportional_value(asset.amount, reserves.a.amount, reserves.b.amount);
                added_assets.b.amount = asset.amount;
            }
            added_liquidity = proportional_value(added_assets.b.amount, total_liquidity, reserves.b.amount);
        }

        PreviewAddLiquidityInfo {
            other_asset_to_add: if asset.id == reserves.a.id {
                added_assets.b
            } else {
                added_assets.a
            },
            liquidity_asset_to_receive: Asset::new(contract_id(), added_liquidity),
        }
    }

    ///  Get information about the output asset for a `swap_exact_input` without doing the swap operation.
    ///
    /// ### Additional Information
    ///
    /// The preview info while swapping `exact_input` of input asset consists of:
    /// - The minimum amount of output asset to receive,
    /// - Whether the output asset reserves are sufficient for the swap or not.
    ///
    /// ### Arguments
    ///
    /// * `exact_input_asset`: `Asset` - The asset to sell.
    ///
     /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn preview_swap_exact_input(exact_input_asset: Asset) -> PreviewSwapInfo {
        let (input_asset, mut output_asset) = determine_assets(exact_input_asset.id, storage.pair.read());

        output_asset.amount = minimum_output_given_exact_input(exact_input_asset.amount, input_asset.amount, output_asset.amount, LIQUIDITY_MINER_FEE);
        let sufficient_reserve = output_asset.amount <= output_asset.amount;

        PreviewSwapInfo {
            other_asset: output_asset,
            sufficient_reserve,
        }
    }

    ///  GGet information about the input asset for a `swap_exact_output` without doing the swap operation.
    ///
    /// ### Additional Information
    ///
    /// The preview info while swapping to get `exact_output` amount of output asset consists of:
    /// - The maximum amount of input asset to forward,
    /// - Whether the input asset reserves are sufficient for the swap or not.
    ///
    /// ### Arguments
    ///
    /// * `exact_output_asset`: `Asset` - The asset to buy.
    ///
    /// # Reverts
    ///
    /// * When the contract has not been initialized, i.e., asset pair in storage is `None`.
    /// * When the `msg_asset_id` does not identify asset A or asset B.
    /// * When the `exact_output` is less than the reserve amount of the output asset.
    ///
    /// ### Number of Storage Accesses
    ///
    /// * Reads: `1`
    #[storage(read)]
    fn preview_swap_exact_output(exact_output_asset: Asset) -> PreviewSwapInfo {
        let (output_asset, mut input_asset) = determine_assets(exact_output_asset.id, storage.pair.read());

        require(exact_output_asset.amount <= output_asset.amount, TransactionError::DesiredAmountTooHigh(exact_output_asset.amount));

        input_asset.amount = maximum_input_for_exact_output(exact_output_asset.amount, input_asset.amount, output_asset.amount, LIQUIDITY_MINER_FEE);
        require(input_asset.amount > 0, TransactionError::DesiredAmountTooLow(exact_output_asset.amount));
        let sufficient_reserve = exact_output_asset.amount <= output_asset.amount;

        PreviewSwapInfo {
            other_asset: input_asset,
            sufficient_reserve,
        }
    }
}
