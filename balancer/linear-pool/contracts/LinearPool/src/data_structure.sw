library data_structure;

pub struct JoinPoolRequest {
    assets: ContractId;
    maxAmountsIn: u64;
    ///todo need some workaround when bytes operation are added
    ///bytes userData;
    fromInternalBalance: bool;
};


//Events
pub struct TargetsSet{
    token: ContractId, 
    lowerTarget: u64, 
    upperTarget: u64
};