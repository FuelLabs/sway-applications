predicate;

use std::{
    bytes::Bytes,
    constants::BASE_ASSET_ID,
    hash::sha256,
    inputs::input_predicate_data,
    outputs::{
        Output,
        output_amount,
        output_pointer,
        output_type,
    },
};
use merkle_proof::binary_merkle_proof::{LEAF, leaf_digest, node_digest, verify_proof};

configurable {
    CLAIM_TOKEN: ContractId = BASE_ASSET_ID,
}

const GTF_OUTPUT_COIN_TO = 0x202;
const GTF_OUTPUT_COIN_ASSET_ID = 0x204;

fn main(
    key: u64,
    amount: u64,
    identity: Identity,
    proof: Vec<b256>,
    merkle_root: b256,
    num_leaves: u64,
) -> bool {
    let proof_length = proof.len();
    assert((num_leaves > 1 && proof_length == path_length_from_key(key, num_leaves)) || (num_leaves <= 1 && proof_length == 0));
    assert(key < num_leaves);

    // Create leaf data
    let mut leaf_data = Bytes::with_capacity(48);
    __addr_of(identity).copy_bytes_to(leaf_data.buf.ptr(), 40);
    let amount_address = leaf_data.buf.ptr().add_uint_offset(40);
    amount_address.write(amount);
    leaf_data.len = 48;
    let leaf = leaf_data.sha256();

    // Create leaf hash
    let mut bytes = Bytes::with_capacity(33);
    let new_ptr = bytes.buf.ptr().add_uint_offset(1);
    bytes.buf.ptr().write_byte(LEAF);
    __addr_of(leaf).copy_bytes_to(new_ptr, 32);
    bytes.len = 33;

    let mut digest = bytes.sha256();
    // If proof length is zero then the leaf is the root
    if proof_length == 0 {
        return digest == merkle_root
    }

    let mut height = 1;
    let mut stable_end = key;

    // While the current subtree is complete, determine the position of the next
    // sibling using the complete subtree algorithm.
    while true {
        // Determine if the subtree is complete.
        let sub_tree_start_index = (key / (1 << height)) * (1 << height);
        let sub_tree_end_index = sub_tree_start_index + (1 << height) - 1;

        // If the Merkle Tree does not have a leaf at the `sub_tree_end_index`, we deem that the
        // subtree is not complete.
        if sub_tree_end_index >= num_leaves {
            break;
        }
        stable_end = sub_tree_end_index;
        assert(proof_length > height - 1);

        // Determine if the key is in the first or the second half of the subtree.
        if (key - sub_tree_start_index) < (1 << (height - 1)) {
            digest = node_digest(digest, proof.get(height - 1).unwrap());
        } else {
            digest = node_digest(proof.get(height - 1).unwrap(), digest);
        }

        height = height + 1;
    }

    // Determine if the next hash belongs to an orphan that was elevated.
    if stable_end != (num_leaves - 1) {
        assert(proof_length > height - 1);
        digest = node_digest(digest, proof.get(height - 1).unwrap());
        height = height + 1;
    }

    // All remaining elements in the proof set will belong to the left sibling.
    while (height - 1) < proof_length {
        digest = node_digest(proof.get(height - 1).unwrap(), digest);
        height = height + 1;
    }

    // Next check that only the appropriate tokens and amounts are transfered
    // Revert if output is not an Output::Coin
    let output_index = 0;
    match output_type(output_index) {
        Output::Coin => (),
        _ => revert(0),
    };

    // // Transaction details
    let to = Identity::Address(Address::from(__gtf::<b256>(output_index, GTF_OUTPUT_COIN_TO)));
    let asset_id = ContractId::from(__gtf::<b256>(output_index, GTF_OUTPUT_COIN_ASSET_ID));
    let amount_sent = output_amount(output_index);
    (merkle_root == digest) && (amount == amount_sent) && (to == identity) && (asset_id == CLAIM_TOKEN)
}

fn path_length_from_key(key: u64, num_leaves: u64) -> u64 {
    let mut total_length = 0;
    let mut num_leaves = num_leaves;
    let mut key = key;

    while true {
        // The height of the left subtree is equal to the offset of the starting bit of the path
        let path_length = starting_bit(num_leaves);
        // Determine the number of leaves in the left subtree
        let num_leaves_left_sub_tree = (1 << (path_length - 1));

        if key <= (num_leaves_left_sub_tree - 1) {
            // If the leaf is in the left subtreee, path length is full height of the left subtree
            total_length = total_length + path_length;
            break;
        } else if num_leaves_left_sub_tree == 1 {
            // If the left sub tree has only one leaf, path has one additional step
            total_length = total_length + 1;
            break;
        } else if (num_leaves - num_leaves_left_sub_tree) <= 1 {
            // If the right sub tree only has one leaf, path has one additonal step
            total_length = total_length + 1;
            break;
        } else {
            // Otherwise add 1 to height and loop
            total_length = total_length + 1;
            key = key - num_leaves_left_sub_tree;
            num_leaves = num_leaves - num_leaves_left_sub_tree;
        }
    }

    total_length
}

fn starting_bit(num_leaves: u64) -> u64 {
    let mut starting_bit = 0;

    while (1 << starting_bit) < num_leaves {
        starting_bit = starting_bit + 1;
    }

    starting_bit
}
