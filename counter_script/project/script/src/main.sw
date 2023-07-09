script;

use contract_abi::CounterContract;

fn main(counter_contract_id: ContractId, clear_count: bool) -> u64 {
    let abi_cast = abi(CounterContract, counter_contract_id.value); // An abi cast is a way to call a contract at a given contract_id with the given abi

    let a = abi_cast.count(); // Here we call the count method, which returns the current count

    require(a == 0, "Counter should be 0"); // As we just deployed the contract, the counter should be 0

    let a = abi_cast.increment(); // Here we call the increment method

    require(a == 1, "Counter should be 1"); // After calling the increment method, the counter should increment by 1

    let a = abi_cast.increment(); // Again, we call the increment method

    require(a == 2, "Counter should be 2");  // After calling the increment method, the counter again increments by 1, so it should be 2

    if clear_count {
        abi_cast.clear(); // Here we clear the counter if the `clear_count` argument passed to the script is true
    } 

    let a = abi_cast.count(); // Getting the current count

    return a; // We return the current count
}