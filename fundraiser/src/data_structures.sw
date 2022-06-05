library data_structures;

pub enum Initialized {
    True: (),
    False: (),
}

pub enum State {
    Funding: (),
    Successful: (),
    Failed: (),
}

struct Campaign {
    author: Sender,
    asset: ContractId,
    claimed: bool,
    deadline: u64,
    state: State,
    target_amount: u64,
    total_pledge: u64,
}
