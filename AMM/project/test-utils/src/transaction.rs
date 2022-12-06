use super::{data_structures::AMMContract, setup::scripts::MAXIMUM_INPUT_AMOUNT};
use fuels::{
    prelude::*,
    tx::{Bytes32, Input, Output, TxPointer, UtxoId},
    types::resource::Resource,
};

pub async fn transaction_inputs_outputs_for_scripts(
    wallet: &WalletUnlocked,
    provider: &Provider,
    amm: &AMMContract,
    assets: &Vec<AssetId>,
) -> (Vec<Input>, Vec<Output>) {
    let mut input_contracts: Vec<Input> = vec![transaction_input_contract(amm.id)];
    let mut output_contracts: Vec<Output> = vec![transaction_output_contract(0)];

    amm.pools
        .values()
        .into_iter()
        .enumerate()
        .for_each(|(index, pool)| {
            input_contracts.push(transaction_input_contract(pool.id));
            output_contracts.push(transaction_output_contract(index as u8 + 1));
        });

    let mut input_coins: Vec<Input> = vec![];
    let mut output_variables: Vec<Output> = vec![];

    let mut i = 0;
    while i < assets.len() {
        input_coins.extend(
            transaction_input_coin(
                &provider,
                wallet.address(),
                *assets.get(i).unwrap(),
                MAXIMUM_INPUT_AMOUNT,
            )
            .await,
        );
        output_variables.push(transaction_output_variable());
        i += 1;
    }

    (
        [input_contracts, input_coins].concat(),
        [output_contracts, output_variables].concat(),
    )
}

async fn transaction_input_coin(
    provider: &Provider,
    from: &Bech32Address,
    asset_id: AssetId,
    amount: u64,
) -> Vec<Input> {
    let coins = &provider
        .get_spendable_resources(from, asset_id, amount)
        .await
        .unwrap();

    let input_coins: Vec<Input> = coins
        .into_iter()
        .map(|coin| {
            let (coin_utxo_id, coin_amount) = match coin {
                Resource::Coin(coin) => (coin.utxo_id.clone(), coin.amount.clone()),
                _ => panic!(),
            };
            Input::CoinSigned {
                utxo_id: coin_utxo_id.into(),
                owner: Address::from(from),
                amount: coin_amount.into(),
                asset_id: asset_id,
                tx_pointer: TxPointer::default(),
                witness_index: 0,
                maturity: 0,
            }
        })
        .collect();

    input_coins
}

fn transaction_input_contract(contract_id: ContractId) -> Input {
    Input::Contract {
        utxo_id: UtxoId::new(Bytes32::zeroed(), 0),
        balance_root: Bytes32::zeroed(),
        state_root: Bytes32::zeroed(),
        tx_pointer: TxPointer::default(),
        contract_id,
    }
}

fn transaction_output_contract(input_index: u8) -> Output {
    Output::Contract {
        input_index,
        balance_root: Bytes32::zeroed(),
        state_root: Bytes32::zeroed(),
    }
}

fn transaction_output_variable() -> Output {
    Output::Variable {
        amount: 0,
        to: Address::zeroed(),
        asset_id: AssetId::default(),
    }
}
