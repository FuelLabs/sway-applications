library data_structures;

pub enum State {
    Funding: (),
    Successful: (),
    Failed: (),
}

pub struct Status {
    state: State,
    target_amount: u64,
    remaining_time: u64,
    pledged_amount: u64,
}
