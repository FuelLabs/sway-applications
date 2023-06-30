script;

use counter_contract::CounterContract;

fn main(counter_contract_id: ContractId) {
    let abi_cast = abi(CounterContract, counter_contract_id); // An abi cast is a way to call a contract at a given contract_id with the given abi

    let a = abi_cast.read(); // Here we call the read method of the CounterContract

    assert(a == 0, "Counter should be 0"); // As we just deployed the contract, the counter should be 0

    let a = abi_cast.increment(); // Here we call the increment method of the CounterContract

    assert(a == 1, "Counter should be 1"); // After calling the increment method, the counter should increment by 1

    let a = abi_cast.increment(); // Again, we call the increment method of the CounterContract

    assert(a == 2, "Counter should be 2");  // After calling the increment method, the counter again increments by 1, so it should be 2

    abi_cast.clear(); // Here we call the clear method of the CounterContract

    let a = abi_cast.read(); // Here we call the read method of the CounterContract

    assert(a == 0, "Counter should be 0");  // After calling the clear method, the counter should be 0 again
}