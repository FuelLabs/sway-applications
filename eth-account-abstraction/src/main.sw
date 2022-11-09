contract;

dep utils;

use utils::{recover_signer, SignatureData};

abi MultiSignatureWallet {
    #[storage(read, write)]
    fn count_approvals(transaction_hash: b256, signatures: Vec<SignatureData>) -> u64;
}

storage {
    /// Number of approvals per user
    weighting: StorageMap<b256, u64> = StorageMap {},
}

impl MultiSignatureWallet for Contract {
    #[storage(read, write)]
    fn count_approvals(message_hash: b256, signatures_data: Vec<SignatureData>) -> u64 {
        //construct storage.weighting
        storage.weighting.insert(0xe10f526b192593793b7a1559a391445faba82a1d669e3eb2dcd17f9c121b24b1, 2);
        storage.weighting.insert(0x000000000000000000000000db4aa29ef306fc8d28025b838ccd3feecaedb333, 4);

        // The signers must have increasing values in order to check for duplicates or a zero-value
        let mut previous_signer = b256::min();

        let mut approval_count = 0;
        let mut index = 0;
        while index < signatures_data.len() {
            let signer = recover_signer(message_hash, signatures_data.get(index).unwrap());

            require(previous_signer < signer, "IncorrectSignerOrdering");

            previous_signer = signer;
            approval_count += storage.weighting.get(signer);

            index += 1;
        }

        approval_count
    }
}
