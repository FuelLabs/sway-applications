use fuels::{
    prelude::*,
    tx::{Bytes32, Input, Output, TxPointer, UtxoId},
    types::resource::Resource,
};

pub async fn transaction_input_coin(
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
                _ => panic!("Resource type does not match"),
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

pub fn transaction_input_contract(contract_id: ContractId) -> Input {
    Input::Contract {
        utxo_id: UtxoId::new(Bytes32::zeroed(), 0),
        balance_root: Bytes32::zeroed(),
        state_root: Bytes32::zeroed(),
        tx_pointer: TxPointer::default(),
        contract_id,
    }
}

pub fn transaction_output_contract(input_index: u8) -> Output {
    Output::Contract {
        input_index,
        balance_root: Bytes32::zeroed(),
        state_root: Bytes32::zeroed(),
    }
}

pub fn transaction_output_variable() -> Output {
    Output::Variable {
        amount: 0,
        to: Address::zeroed(),
        asset_id: AssetId::default(),
    }
}
