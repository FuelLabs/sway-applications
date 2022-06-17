library data_structures;

pub struct Proposal {
    yes_votes: u64,
    no_votes: u64,
    approval_percentage: u64,
    end_height: u64,
    data: b256,
}
