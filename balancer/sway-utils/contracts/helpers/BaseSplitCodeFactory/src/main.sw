contract;

use std::address::Address;

// address private immutable _creationCodeContractA;
// uint256 private immutable _creationCodeSizeA;

// address private immutable _creationCodeContractB;
// uint256 private immutable _creationCodeSizeB;

// /**
//  * @dev The creation code of a contract Foo can be obtained inside Solidity with `type(Foo).creationCode`.
//  */
// constructor(bytes memory creationCode) {
//     uint256 creationCodeSize = creationCode.length;

//     // We are going to deploy two contracts: one with approximately the first half of `creationCode`'s contents
//     // (A), and another with the remaining half (B).
//     // We store the lengths in both immutable and stack variables, since immutable variables cannot be read during
//     // construction.
//     uint256 creationCodeSizeA = creationCodeSize / 2;
//     _creationCodeSizeA = creationCodeSizeA;

//     uint256 creationCodeSizeB = creationCodeSize - creationCodeSizeA;
//     _creationCodeSizeB = creationCodeSizeB;

//     // To deploy the contracts, we're going to use `CodeDeployer.deploy()`, which expects a memory array with
//     // the code to deploy. Note that we cannot simply create arrays for A and B's code by copying or moving
//     // `creationCode`'s contents as they are expected to be very large (> 24kB), so we must operate in-place.

//     // Memory: [ code length ] [ A.data ] [ B.data ]

//     // Creating A's array is simple: we simply replace `creationCode`'s length with A's length. We'll later restore
//     // the original length.

//     bytes memory creationCodeA;
//     assembly {
//         creationCodeA := creationCode
//         mstore(creationCodeA, creationCodeSizeA)
//     }

//     // Memory: [ A.length ] [ A.data ] [ B.data ]
//     //         ^ creationCodeA

//     _creationCodeContractA = CodeDeployer.deploy(creationCodeA);

//     // Creating B's array is a bit more involved: since we cannot move B's contents, we are going to create a 'new'
//     // memory array starting at A's last 32 bytes, which will be replaced with B's length. We'll back-up this last
//     // byte to later restore it.

//     bytes memory creationCodeB;
//     bytes32 lastByteA;

//     assembly {
//         // `creationCode` points to the array's length, not data, so by adding A's length to it we arrive at A's
//         // last 32 bytes.
//         creationCodeB := add(creationCode, creationCodeSizeA)
//         lastByteA := mload(creationCodeB)
//         mstore(creationCodeB, creationCodeSizeB)
//     }

//     // Memory: [ A.length ] [ A.data[ : -1] ] [ B.length ][ B.data ]
//     //         ^ creationCodeA                ^ creationCodeB

//     _creationCodeContractB = CodeDeployer.deploy(creationCodeB);

//     // We now restore the original contents of `creationCode` by writing back the original length and A's last byte.
//     assembly {
//         mstore(creationCodeA, creationCodeSize)
//         mstore(creationCodeB, lastByteA)
//     }
// }

// few variables and constructor of the BaseSplitCodeFactory.sol contract ^ it sets some values

abi BaseSplitCodeFactory {
    fn get_creation_code_contracts() -> (Address, Address);
    fn get_creation_code() -> b256;
    fn _create(constructor_args: b256) -> Address;
}

// created stub const variables to complete the instruction(s)
// these variables must set inside the constructor function, but we have no construcor function in sway; hence creating these stubs
const _CREATION_CODE_CONTRACT_A: Address = ~Address::from(0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b);
const _CREATION_CODE_SIZE_A: u64 = 100;
const _CREATION_CODE_CONTRACT_B: Address = ~Address::from(0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b);
const _CREATION_CODE_SIZE_B: u64 = 100;

fn _get_creation_code_with_args(constructor_args: b256) -> b256 {
    let creation_code_contract_a: Address = _CREATION_CODE_CONTRACT_A;
    let creation_code_size_a: u64 = _CREATION_CODE_SIZE_A;
    let creation_code_contract_b: Address = _CREATION_CODE_CONTRACT_B;
    let creation_code_size_b: u64 = _CREATION_CODE_SIZE_B;

    let creation_code_size: u64 = creation_code_size_a + creation_code_size_b;
    // let constructor_args_size: u64 = constructor_args.length;
    let constructor_args_size: u64 = 100;

    let code_size: u64 = creation_code_size + constructor_args_size;

    // temp variables used in asm
    let temp1 = 0x40;
    let temp2 = 32;
    let temp3 = 0;
    // temp variables used in asm

    let mut constructor_args_data_ptr: u64 = 0;
    let mut constructor_args_code_data_ptr: u64 = 0;

    // these instructions might not work; there can be incorrect instructions
    // or we might need to change the instructions
    asm(temp1, temp2, temp3, code_size, creation_code_contract_a, creation_code_contract_b, creation_code_size_a, creation_code_size_b, constructor_args_data_ptr, constructor_args_code_data_ptr, constructor_args, creation_code_size, r1, r2, r3, r4, r5, r6) {
        // code := mload(0x40)
        lb r1 temp1 i0;

        // mstore(0x40, add(code, add(codeSize, 32)))
        // for mstore we need to do some add first
        add r2 code_size temp2;
        add r3 r1 r2;
        sb temp1 r3 i0;

        // mstore(code, codeSize)
        sb r1 code_size i0;

        // let dataStart := add(code, 32)
        add r4 r1 temp2;

        // extcodecopy(creationCodeContractA, dataStart, 0, creationCodeSizeA)
        // not sure if this is the correct sway asm equivalent of solidity extcodecopy()
        ccp creation_code_contract_a r4 temp3 creation_code_size_a;

        // extcodecopy(creationCodeContractB, add(dataStart, creationCodeSizeA), 0, creationCodeSizeB)
        add r5 r4 creation_code_size_a;
        ccp creation_code_contract_b r5 temp3 creation_code_size_b;

        // constructorArgsDataPtr := add(constructorArgs, 32)
        add constructor_args_data_ptr constructor_args temp2;

        // constructorArgsCodeDataPtr := add(add(code, 32), creationCodeSize)
        add r6 r1 temp2;
        add constructor_args_code_data_ptr r6 creation_code_size;
    }

    _memcpy(constructor_args_code_data_ptr, constructor_args_data_ptr, constructor_args_size);

    // assembly {
    //     constructorArgsDataPtr := add(constructorArgs, 32)
    //     constructorArgsCodeDataPtr := add(add(code, 32), creationCodeSize)
    // }

    // this asm block here, is combined into the asm block above this one
    // because of the fact in this one we need to access the code variable
    // which means r1 from the first block, but we cannot access the r1 from the first asm block
    // into second asm block, hence combining both the blocks into one

    return 0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b;
}

fn _memcpy(dest: u64, src: u64, len: u64) {
    let mut len = len;
    let mut src = src;
    let mut dest = dest;

    while(len >= 32) {
        asm(dest, src, r1) {
            // mstore(dest, mload(src))
            lb r1 src i0;
            sb dest r1 i0;
        }

        src += 32;
        dest += 32;

        len -= 32;
    }

    // original mask
    // uint256 mask = 256**(32 - len) - 1;
    // sway test mask
    let mask: u64 = 123;

    asm(mask, src, dest, src_part, dest_part, r1, r2, r3, r4) {
        // let srcpart := and(mload(src), not(mask))
        lb r1 src i0;
        not r2 mask;
        and src_part r1 r2;

        // let destpart := and(mload(dest), mask)
        lb r3 dest i0;
        and dest_part r3 mask;

        // mstore(dest, or(destpart, srcpart))
        or r4 dest_part src_part;
        sw dest r4 i0;
    }
}

impl BaseSplitCodeFactory for Contract {
    fn get_creation_code_contracts() -> (Address, Address) {
        return(_CREATION_CODE_CONTRACT_A, _CREATION_CODE_CONTRACT_B);
    }

    fn get_creation_code() -> b256 {
        return _get_creation_code_with_args(0x9299da6c73e6dc03eeabcce242bb347de3f5f56cd1c70926d76526d7ed199b8b);
    }

    fn _create(constructor_args: b256) -> Address {
        let creation_code: b256 = _get_creation_code_with_args(constructor_args);
        let mut destination: Address = ~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000);

        // temp variables
        let temp1 = 32;
        let temp2 = 0;
        // temp variables

        asm(destination, creation_code, temp1, r1, r2) {
            // destination := create(0, add(creationCode, 32), mload(creationCode))
            add r1 creation_code temp1;
            lb r2 creation_code i0;

            // we have nothing for create() in sway ~~~ create(0, add(creationCode, 32), mload(creationCode))
            // need to add that instruction after we get something like solidity asm create() in sway as well
        }

        if destination == ~Address::from(0x0000000000000000000000000000000000000000000000000000000000000000) {
            // assembly {
            //     returndatacopy(0, 0, returndatasize())
            //     revert(0, returndatasize())
            // }

            // couldnt find anything related to this instructions, in sway asm
            // need to add this after we got something similar
        }

        return destination;
    }
}
