library token_asset;

dep traits;

use ::errors::AssetError;
use traits::Asset;

pub struct TokenAsset {
    /// The amount of the native asset that the struct is representing.
    amount: u64,
    /// The `ContractId` of the native asset that the struct is representing.
    contract_id: ContractId,
}

impl TokenAsset {
    fn new(amount: u64, contract_id: ContractId) -> Self {
        TokenAsset {
            amount,
            contract_id,
        }
    }
}

impl Asset for TokenAsset {
    fn amount(self) -> u64 {
        self.amount
    }

    fn contract_id(self) -> ContractId {
        self.contract_id
    }
}

impl core::ops::Add for TokenAsset {
    fn add(self, other: Self) -> Self {
        require(self.contract_id() == other.contract_id(), AssetError::AssetsAreNotTheSame);
        ~TokenAsset::new(self.amount() + other.amount(), self.contract_id())
    }
}

impl core::ops::Eq for TokenAsset {
    fn eq(self, other: Self) -> bool {
        self.contract_id() == other.contract_id()
    }
}

impl core::ops::Ord for TokenAsset {
    fn gt(self, other: Self) -> bool {
        require(self.contract_id() == other.contract_id(), AssetError::AssetsAreNotTheSame);
        self.amount() > other.amount()
    }
    fn lt(self, other: Self) -> bool {
        require(self.contract_id() == other.contract_id(), AssetError::AssetsAreNotTheSame);
        self.amount() < other.amount()
    }
}
