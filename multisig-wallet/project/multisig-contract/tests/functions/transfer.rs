use crate::utils::{
    abi_calls::{balance, constructor, nonce, transaction_hash, transfer},
    test_helpers::{base_asset_contract_id, format_and_sign, setup_env},
    MessageFormat, MessagePrefix, SignatureData, User, WalletType, VALID_SIGNER_PK,
};
use fuels::{prelude::*, signers::fuel_crypto::Message, tx::Bytes32};

use rand::{rngs::StdRng, Rng, SeedableRng};

mod success {

    use super::*;

    #[tokio::test]
    async fn transfers() {
        let (private_key, contract, deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let receiver_wallet = WalletUnlocked::new_random(None);

        let fuel_user_1 = User {
            address: Bits256::from_hex_str(
                "0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1",
            )
            .unwrap(),
            weight: 3,
        };
        let evm_user_1 = User {
            address: Bits256::from_hex_str(
                "0x000000000000000000000000db4aa29ef306fc8d28025b838ccd3feecaedb333",
            )
            .unwrap(),
            weight: 2,
        };
        let users = vec![fuel_user_1, evm_user_1];

        let _response = constructor(&contract, users, 5).await;

        let _receipt = deployer_wallet
            .force_transfer_to_contract(
                contract.get_contract_id(),
                200,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        // Check balances pre-transfer
        let initial_contract_balance = balance(&contract, base_asset_contract_id()).await.value;

        let initial_receiver_balance = deployer_wallet
            .get_provider()
            .unwrap()
            .get_asset_balance(receiver_wallet.address(), BASE_ASSET_ID)
            .await
            .unwrap();

        let to = Identity::Address(receiver_wallet.address().try_into().unwrap());

        let value = 200;

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);

        let nonce = nonce(&contract).await.value;

        let tx_hash = transaction_hash(&contract, to.clone(), value, data, nonce)
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        // Signature Data
        // - Fuel signature. Fuel wallet. No format. No prefix.
        let signature_data_fuel = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::None(),
            MessagePrefix::None(),
            WalletType::Fuel(),
        )
        .await;

        // - EVM signature. EVM wallet. EIP-191 personal sign format. Ethereum prefix.
        let signature_data_evm = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::EIP191PersonalSign(),
            MessagePrefix::Ethereum(),
            WalletType::EVM(),
        )
        .await;

        // - Must comply with ascending signers requirement of Multisig's count_approvals
        let signatures_data: Vec<SignatureData> = vec![signature_data_evm, signature_data_fuel];

        let _response = transfer(
            &contract,
            to,
            base_asset_contract_id(),
            value,
            data,
            signatures_data,
        )
        .await;

        // check balances post-transfer
        let final_contract_balance = balance(&contract, base_asset_contract_id()).await.value;

        let final_receiver_balance = deployer_wallet
            .get_provider()
            .unwrap()
            .get_asset_balance(receiver_wallet.address(), BASE_ASSET_ID)
            .await
            .unwrap();

        assert_eq!(initial_contract_balance, 200);
        assert_eq!(initial_receiver_balance, 0);

        assert_eq!(final_contract_balance, 0);
        assert_eq!(final_receiver_balance, 200);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn not_initialized() {
        let (private_key, contract, deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let receiver_wallet = WalletUnlocked::new_random(None);

        let _receipt = deployer_wallet
            .force_transfer_to_contract(
                contract.get_contract_id(),
                200,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        let to = Identity::Address(receiver_wallet.address().try_into().unwrap());

        let value = 200;

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);

        let nonce = nonce(&contract).await.value;

        let tx_hash = transaction_hash(&contract, to.clone(), value, data, nonce)
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        // Signature Data
        // - Fuel signature. Fuel wallet. No format. No prefix.
        let signature_data_fuel = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::None(),
            MessagePrefix::None(),
            WalletType::Fuel(),
        )
        .await;

        // - EVM signature. EVM wallet. EIP-191 personal sign format. Ethereum prefix.
        let signature_data_evm = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::EIP191PersonalSign(),
            MessagePrefix::Ethereum(),
            WalletType::EVM(),
        )
        .await;

        // - Must comply with ascending signers requirement of Multisig's count_approvals
        let signatures_data: Vec<SignatureData> = vec![signature_data_evm, signature_data_fuel];

        let _response = transfer(
            &contract,
            to,
            base_asset_contract_id(),
            value,
            data,
            signatures_data,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic]
    async fn insufficient_asset_amount() {
        let (private_key, contract, _deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let receiver_wallet = WalletUnlocked::new_random(None);

        let fuel_user_1 = User {
            address: Bits256::from_hex_str(
                "0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1",
            )
            .unwrap(),
            weight: 3,
        };
        let evm_user_1 = User {
            address: Bits256::from_hex_str(
                "0x000000000000000000000000db4aa29ef306fc8d28025b838ccd3feecaedb333",
            )
            .unwrap(),
            weight: 2,
        };
        let users = vec![fuel_user_1, evm_user_1];

        let _response = constructor(&contract, users, 5).await;

        let to = Identity::Address(receiver_wallet.address().try_into().unwrap());

        let value = 200;

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);

        let nonce = nonce(&contract).await.value;

        let tx_hash = transaction_hash(&contract, to.clone(), value, data, nonce)
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        // Signature Data
        // - Fuel signature. Fuel wallet. No format. No prefix.
        let signature_data_fuel = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::None(),
            MessagePrefix::None(),
            WalletType::Fuel(),
        )
        .await;

        // - EVM signature. EVM wallet. EIP-191 personal sign format. Ethereum prefix.
        let signature_data_evm = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::EIP191PersonalSign(),
            MessagePrefix::Ethereum(),
            WalletType::EVM(),
        )
        .await;

        // - Must comply with ascending signers requirement of Multisig's count_approvals
        let signatures_data: Vec<SignatureData> = vec![signature_data_evm, signature_data_fuel];

        let _response = transfer(
            &contract,
            to,
            base_asset_contract_id(),
            value,
            data,
            signatures_data,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic]
    async fn insufficient_approvals() {
        let (private_key, contract, deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let receiver_wallet = WalletUnlocked::new_random(None);

        let fuel_user_1 = User {
            address: Bits256::from_hex_str(
                "0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1",
            )
            .unwrap(),
            weight: 1,
        };
        let evm_user_1 = User {
            address: Bits256::from_hex_str(
                "0x000000000000000000000000db4aa29ef306fc8d28025b838ccd3feecaedb333",
            )
            .unwrap(),
            weight: 1,
        };
        let users = vec![fuel_user_1, evm_user_1];

        let _response = constructor(&contract, users, 5).await;

        let _receipt = deployer_wallet
            .force_transfer_to_contract(
                contract.get_contract_id(),
                200,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        let to = Identity::Address(receiver_wallet.address().try_into().unwrap());

        let value = 200;

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);

        let nonce = nonce(&contract).await.value;

        let tx_hash = transaction_hash(&contract, to.clone(), value, data, nonce)
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        // Signature Data
        // - Fuel signature. Fuel wallet. No format. No prefix.
        let signature_data_fuel = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::None(),
            MessagePrefix::None(),
            WalletType::Fuel(),
        )
        .await;

        // - EVM signature. EVM wallet. EIP-191 personal sign format. Ethereum prefix.
        let signature_data_evm = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::EIP191PersonalSign(),
            MessagePrefix::Ethereum(),
            WalletType::EVM(),
        )
        .await;

        // - Must comply with ascending signers requirement of Multisig's count_approvals
        let signatures_data: Vec<SignatureData> = vec![signature_data_evm, signature_data_fuel];

        let _response = transfer(
            &contract,
            to,
            base_asset_contract_id(),
            value,
            data,
            signatures_data,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic]
    async fn incorrect_signer_ordering() {
        let (private_key, contract, deployer_wallet) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let receiver_wallet = WalletUnlocked::new_random(None);

        let fuel_user_1 = User {
            address: Bits256::from_hex_str(
                "0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1",
            )
            .unwrap(),
            weight: 3,
        };
        let evm_user_1 = User {
            address: Bits256::from_hex_str(
                "0x000000000000000000000000db4aa29ef306fc8d28025b838ccd3feecaedb333",
            )
            .unwrap(),
            weight: 2,
        };
        let users = vec![fuel_user_1, evm_user_1];

        let _response = constructor(&contract, users, 5).await;

        let _receipt = deployer_wallet
            .force_transfer_to_contract(
                contract.get_contract_id(),
                200,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        let to = Identity::Address(receiver_wallet.address().try_into().unwrap());

        let value = 200;

        let mut rng = StdRng::seed_from_u64(1000);
        let data: Bytes32 = rng.gen();
        let data = Bits256(*data);

        let nonce = nonce(&contract).await.value;

        let tx_hash = transaction_hash(&contract, to.clone(), value, data, nonce)
            .await
            .value
            .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        // Signature Data
        // - Fuel signature. Fuel wallet. No format. No prefix.
        let signature_data_fuel = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::None(),
            MessagePrefix::None(),
            WalletType::Fuel(),
        )
        .await;

        // - EVM signature. EVM wallet. EIP-191 personal sign format. Ethereum prefix.
        let signature_data_evm = format_and_sign(
            private_key,
            tx_hash,
            MessageFormat::EIP191PersonalSign(),
            MessagePrefix::Ethereum(),
            WalletType::EVM(),
        )
        .await;

        // - Should comply with ascending signers requirement of Multisig's count_approvals, but here is incorrectly ordered.
        let signatures_data: Vec<SignatureData> = vec![signature_data_fuel, signature_data_evm];

        let _response = transfer(
            &contract,
            to,
            base_asset_contract_id(),
            value,
            data,
            signatures_data,
        )
        .await;
    }
}
