script;

use interface::Counter;

fn main(counter_contract_id: ContractId, clear_count: bool) -> u64 {
    // An abi cast is a way to call a contract at a given contract_id with the given abi
    let abi_cast = abi(Counter, counter_contract_id.value);
    // Here we call the count method, which returns the current count
    let a = abi_cast.count(); 
    // As we just deployed the contract, the counter should be 0
    require(a == 0, "Counter should be 0");
    // Here we call the increment method
    let a = abi_cast.increment(); 
    // After calling the increment method, the counter should increment by 1
    require(a == 1, "Counter should be 1"); 
    // Again, we call the increment method
    let a = abi_cast.increment(); 
    // After calling the increment method, the counter again increments by 1, so it should be 2
    require(a == 2, "Counter should be 2"); 
    if clear_count {
        // Here we clear the counter if the `clear_count` argument passed to the script is true
        abi_cast.clear(); 
    }
    // Getting the current count
    let a = abi_cast.count(); 
    // We return the current count
    return a; 
}
