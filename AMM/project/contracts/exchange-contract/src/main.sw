contract;

dep errors;
dep events;
dep utils;

use errors::{InitError, InputError, TransactionError};
use events::{
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
    logging::log,
    math::*,
    token::{
        burn,
        mint,
        transfer,
    },
};
use utils::{
    determine_assets,
    maximum_input_for_exact_output,
    minimum_output_given_exact_input,
    multiply_divide,
};

storage {
    /// Deposit amounts per (depositer, asset) that can be used to add liquidity or be withdrawn.
    deposits: StorageMap<(Identity, ContractId), u64> = StorageMap {},
    /// Total amount of the liquidity pool asset that has a unique identifier different from the identifiers of assets on either side of the pool.
    liquidity_pool_supply: u64 = 0,
    /// The unique identifiers that make up the pool that can be set only once using the `constructor`.
    pair: Option<AssetPair> = Option::None,
}

impl Exchange for Contract {
    #[storage(read, write)]
    fn add_liquidity(desired_liquidity: u64, deadline: u64) -> u64 {
        require(storage.pair.is_some(), InitError::AssetPairNotSet);
        require(deadline > height(), InputError::DeadlinePassed(deadline));
        require(msg_amount() == 0, InputError::ExpectedZeroAmount);
        require(MINIMUM_LIQUIDITY <= desired_liquidity, InputError::CannotAddLessThanMinimumLiquidity(desired_liquidity));

        let sender = msg_sender().unwrap();
        let reserves = storage.pair.unwrap();
        let deposits = AssetPair::of(Asset::new(reserves.a.id, storage.deposits.get((sender, reserves.a.id))), Asset::new(reserves.b.id, storage.deposits.get((sender, reserves.b.id))));

        // checking this because this will either result in a math error or adding no liquidity at all
        require(deposits.a.amount != 0, TransactionError::ExpectedNonZeroDeposit(deposits.a.id));
        require(deposits.b.amount != 0, TransactionError::ExpectedNonZeroDeposit(deposits.b.id));

        let total_liquidity = storage.liquidity_pool_supply;

        let mut added_assets = AssetPair::of(Asset::new(reserves.a.id, 0), Asset::new(reserves.b.id, 0));
        let mut added_liquidity = 0;

        // adding liquidity for the first time
        // use up all the deposited amounts of assets to determine the ratio
        if reserves.a.amount == 0 && reserves.b.amount == 0 {
            added_liquidity = (deposits.a.amount * deposits.b.amount).sqrt();
            require(desired_liquidity <= added_liquidity, TransactionError::DesiredAmountTooHigh(desired_liquidity));
            added_assets.a.amount = deposits.a.amount;
            added_assets.b.amount = deposits.b.amount;

            // add amounts to reserves
            storage.pair = Option::Some(added_assets);

            // mint liquidity pool asset and transfer to sender
            mint(added_liquidity);
            storage.liquidity_pool_supply = added_liquidity;
            transfer(added_liquidity, contract_id(), sender);
        } else { // adding further liquidity based on current ratio
            // attempt to add liquidity by using up the deposited asset A amount
            let b_to_attempt = multiply_divide(deposits.a.amount, reserves.b.amount, reserves.a.amount);

            // continue adding based on asset A if deposited asset B amount is sufficient
            if b_to_attempt <= deposits.b.amount {
                added_liquidity = multiply_divide(b_to_attempt, total_liquidity, reserves.b.amount);
                require(desired_liquidity <= added_liquidity, TransactionError::DesiredAmountTooHigh(desired_liquidity));
                added_assets.a.amount = deposits.a.amount;
                added_assets.b.amount = b_to_attempt;
            } else { // attempt to add liquidity by using up the deposited asset B amount
                let a_to_attempt = multiply_divide(deposits.b.amount, reserves.a.amount, reserves.b.amount);
                added_liquidity = multiply_divide(a_to_attempt, total_liquidity, reserves.a.amount);
                require(desired_liquidity <= added_liquidity, TransactionError::DesiredAmountTooHigh(desired_liquidity));
                added_assets.a.amount = a_to_attempt;
                added_assets.b.amount = deposits.b.amount;
            }

            // add new asset amounts to reserves
            storage.pair = Option::Some(reserves + added_assets);

            // mint liquidity pool asset and transfer to sender
            mint(added_liquidity);
            storage.liquidity_pool_supply = total_liquidity + added_liquidity;
            transfer(added_liquidity, contract_id(), sender);

            // transfer remaining deposit amounts back to the sender
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

    #[storage(read, write)]
    fn constructor(asset_a_id: ContractId, asset_b_id: ContractId) {
        require(storage.pair.is_none(), InitError::AssetPairAlreadySet);
        require(asset_a_id != asset_b_id, InitError::IdenticalAssets);

        storage.pair = Option::Some(AssetPair::of(Asset::new(asset_a_id, 0), Asset::new(asset_b_id, 0)));
        log(DefineAssetPairEvent {
            asset_a_id,
            asset_b_id,
        });
    }

    #[storage(read, write)]
    fn deposit() {
        require(storage.pair.is_some(), InitError::AssetPairNotSet);

        let deposit_asset = msg_asset_id();

        require(deposit_asset == storage.pair.unwrap().a.id || deposit_asset == storage.pair.unwrap().b.id, InputError::InvalidAsset);

        let sender = msg_sender().unwrap();
        let amount = msg_amount();
        let new_balance = storage.deposits.get((sender, deposit_asset)) + amount;
        storage.deposits.insert((sender, deposit_asset), new_balance);

        log(DepositEvent {
            deposited_asset: Asset::new(deposit_asset, amount),
            new_balance,
        });
    }

    #[storage(read, write)]
    fn remove_liquidity(min_asset_a: u64, min_asset_b: u64, deadline: u64) -> RemoveLiquidityInfo {
        require(storage.pair.is_some(), InitError::AssetPairNotSet);

        let total_liquidity = storage.liquidity_pool_supply;
        require(total_liquidity > 0, TransactionError::NoLiquidityToRemove);

        let reserves = storage.pair.unwrap();

        require(min_asset_a > 0, InputError::ExpectedNonZeroParameter(reserves.a.id));
        require(min_asset_b > 0, InputError::ExpectedNonZeroParameter(reserves.b.id));
        require(deadline > height(), InputError::DeadlinePassed(deadline));

        let burned_liquidity = Asset::new(contract_id(), msg_amount());

        require(burned_liquidity.id == msg_asset_id(), InputError::InvalidAsset);
        require(burned_liquidity.amount > 0, InputError::ExpectedNonZeroAmount(burned_liquidity.id));

        let mut removed_assets = AssetPair::of(Asset::new(reserves.a.id, 0), Asset::new(reserves.b.id, 0));
        removed_assets.a.amount = multiply_divide(burned_liquidity.amount, reserves.a.amount, total_liquidity);
        removed_assets.b.amount = multiply_divide(burned_liquidity.amount, reserves.b.amount, total_liquidity);

        require(removed_assets.a.amount >= min_asset_a, TransactionError::DesiredAmountTooHigh(min_asset_a));
        require(removed_assets.b.amount >= min_asset_b, TransactionError::DesiredAmountTooHigh(min_asset_b));

        burn(burned_liquidity.amount);
        storage.liquidity_pool_supply = total_liquidity - burned_liquidity.amount;
        storage.pair = Option::Some(reserves - removed_assets);

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

    #[storage(read, write)]
    fn swap_exact_input(min_output: Option<u64>, deadline: u64) -> u64 {
        require(deadline >= height(), InputError::DeadlinePassed(deadline));

        let reserves = storage.pair;
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
        storage.pair = Option::Some(AssetPair::of(input_asset, output_asset).sort(reserves.unwrap()));

        log(SwapEvent {
            input: input_asset,
            output: output_asset,
        });

        bought
    }

    #[storage(read, write)]
    fn swap_exact_output(output: u64, deadline: u64) -> u64 {
        let reserves = storage.pair;
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
        storage.pair = Option::Some(AssetPair::of(input_asset, output_asset).sort(reserves.unwrap()));

        log(SwapEvent {
            input: input_asset,
            output: output_asset,
        });

        sold
    }

    #[storage(read, write)]
    fn withdraw(asset: Asset) {
        require(storage.pair.is_some(), InitError::AssetPairNotSet);

        require(asset.id == storage.pair.unwrap().a.id || asset.id == storage.pair.unwrap().b.id, InputError::InvalidAsset);

        let sender = msg_sender().unwrap();
        let deposited_amount = storage.deposits.get((sender, asset.id));

        require(deposited_amount >= asset.amount, TransactionError::DesiredAmountTooHigh(asset.amount));

        let new_amount = deposited_amount - asset.amount;
        storage.deposits.insert((sender, asset.id), new_amount);
        transfer(asset.amount, asset.id, sender);

        log(WithdrawEvent {
            withdrawn_asset: asset,
            remaining_balance: new_amount,
        });
    }

    #[storage(read)]
    fn balance(asset_id: ContractId) -> u64 {
        require(storage.pair.is_some(), InitError::AssetPairNotSet);
        require(asset_id == storage.pair.unwrap().a.id || asset_id == storage.pair.unwrap().b.id, InputError::InvalidAsset);

        storage.deposits.get((msg_sender().unwrap(), asset_id))
    }

    #[storage(read)]
    fn pool_info() -> PoolInfo {
        require(storage.pair.is_some(), InitError::AssetPairNotSet);

        PoolInfo {
            reserves: storage.pair.unwrap(),
            liquidity: storage.liquidity_pool_supply,
        }
    }

    #[storage(read)]
    fn preview_add_liquidity(asset: Asset) -> PreviewAddLiquidityInfo {
        require(storage.pair.is_some(), InitError::AssetPairNotSet);

        let sender = msg_sender().unwrap();
        let total_liquidity = storage.liquidity_pool_supply;
        let reserves = storage.pair.unwrap();
        let deposits = AssetPair::of(Asset::new(reserves.a.id, storage.deposits.get((sender, reserves.a.id))), Asset::new(reserves.b.id, storage.deposits.get((sender, reserves.b.id))));

        let mut added_assets = AssetPair::of(Asset::new(reserves.a.id, 0), Asset::new(reserves.b.id, 0));
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
                added_assets.b.amount = multiply_divide(asset.amount, reserves.b.amount, reserves.a.amount);
            } else {
                added_assets.a.amount = multiply_divide(asset.amount, reserves.a.amount, reserves.b.amount);
                added_assets.b.amount = asset.amount;
            }
            added_liquidity = multiply_divide(added_assets.b.amount, total_liquidity, reserves.b.amount);
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

    #[storage(read)]
    fn preview_swap_exact_input(exact_input_asset: Asset) -> PreviewSwapInfo {
        let (input_asset, mut output_asset) = determine_assets(exact_input_asset.id, storage.pair);

        output_asset.amount = minimum_output_given_exact_input(exact_input_asset.amount, input_asset.amount, output_asset.amount, LIQUIDITY_MINER_FEE);
        let sufficient_reserve = output_asset.amount <= output_asset.amount;

        PreviewSwapInfo {
            other_asset: output_asset,
            sufficient_reserve,
        }
    }

    #[storage(read)]
    fn preview_swap_exact_output(exact_output_asset: Asset) -> PreviewSwapInfo {
        let (output_asset, mut input_asset) = determine_assets(exact_output_asset.id, storage.pair);

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
