library data_structures;

pub enum State {
    Void: (),
    Funding: (),
    Successful: (),
    Failed: (),
}

pub struct Info {
    claimed: bool,
    remaining_time: u64,
    state: State,
    target_amount: u64,
    total_pledge: u64,
}
