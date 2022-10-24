library traits;

pub trait Asset {
    fn amount(self) -> u64;
    fn contract_id(self) -> ContractId;
}
