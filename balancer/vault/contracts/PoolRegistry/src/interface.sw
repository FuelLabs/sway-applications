library interface;

dep data_structures;

use data_structures::PoolSpecialization;

abi PoolRegistry{
    #[storage(read)]fn ensure_registered_pool(poolId: b256);
    #[storage(read,write)]fn register_pool(specialization :PoolSpecialization ) ->b256;
}
