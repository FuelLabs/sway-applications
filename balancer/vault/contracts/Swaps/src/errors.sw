library errors;

pub enum Error {
    UNKNOWN_AMOUNT_IN_FIRST_SWAP: (),
    CANNOT_SWAP_SAME_TOKEN: (),
    SWAP_LIMIT: (),
    SWAP_DEADLINE: (),
    MALCONSTRUCTED_MULTIHOP_SWAP: (),
    OUT_OF_BOUNDS: (),
    MALCONSTRUCTED_MULTIHOP_SWP: (),
    INSUFFICIENT_ETH: (),
}