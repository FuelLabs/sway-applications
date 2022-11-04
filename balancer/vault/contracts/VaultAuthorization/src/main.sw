contract;
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.


use BalancerErrors::{
    USER_DOESNT_ALLOW_RELAYER,
    SENDER_NOT_ALLOWED,
};
// use Authorizer::{
//     can_perform,
// };




use std::{
    storage::StorageMap,
    hash::sha256,
    logging::log,
    reentrancy::is_reentrant,
    tx::tx_script_data_length,
    address::Address,
    chain::auth::{AuthError, msg_sender},
    constants::BASE_ASSET_ID,
    context::{msg_amount, call_frames::{contract_id, msg_asset_id}, balance_of},
    contract_id::ContractId,
    identity::Identity,
    result::Result,
    revert::{revert, require},
    token::{force_transfer_to_contract,transfer_to_output},
    vec::Vec,
    option::Option,
};


abi VaultAuthorization {
    // fn init_vault_authorization (name: str[20], authorizer: b256 );
    #[storage(read,write)]fn _set_authorizer(newAuthorizer: b256); 
    #[storage(read,write)]fn set_relayer_approval(sender: Address,relayer: Address,approved: bool); 

}

abi TemporarilyPausable {
    #[storage(read)]fn get_paused_state() -> (bool, u64, u64);
    #[storage(write)]fn _set_paused(paused: bool);
    fn _ensure_not_paused();
    fn _ensure_paused();
}

pub struct RelayerApprovalChanged{
    sender: Address,
    relayer: Address,
    approved: bool
}

/**
* @dev Manages access control of Vault permissioned functions by relying on the Authorizer and signature validation.
*
* Additionally handles relayer access and approval.
*/

// Ideally, we'd store the type hashes as immutable state variables to avoid computing the hash at runtime, but
// unfortunately immutable variables cannot be used in assembly, so we just keep the precomputed hashes instead.

// _JOIN_TYPE_HASH = keccak256("JoinPool(bytes calldata,address sender,uint2chain::auth::{AuthError, msg_sender},56 nonce,uint256 deadline)");
const _JOIN_TYPE_HASH: b256 = 0x3f7b71252bd19113ff48c19c6e004a9bcfcca320a0d74d58e85877cbd7dcae58;

// _EXIT_TYPE_HASH = keccak256("ExitPool(bytes calldata,address sender,uint256 nonce,uint256 deadline)");
const _EXIT_TYPE_HASH: b256 = 0x8bbc57f66ea936902f50a71ce12b92c43f3c5340bb40c27c4e90ab84eeae3353;

// _SWAP_TYPE_HASH = keccak256("Swap(bytes calldata,address sender,uint256 nonce,uint256 deadline)");
const _SWAP_TYPE_HASH: b256 = 0xe192dcbc143b1e244ad73b813fd3c097b832ad260a157340b4e5e5beda067abe;

// _BATCH_SWAP_TYPE_HASH = keccak256("BatchSwap(bytes calldata,address sender,uint256 nonce,uint256 deadline)");
const _BATCH_SWAP_TYPE_HASH: b256 = 0x9bfc43a4d98313c6766986ffd7c916c7481566d9f224c6819af0a53388aced3a;

// _SET_RELAYER_TYPE_HASH =
//     keccak256("SetRelayerApproval(bytes calldata,address sender,uint256 nonce,uint256 deadline)");
const _SET_RELAYER_TYPE_HASH: b256 = 0xa3f865aa351e51cfeb40f5178d1564bb629fe9030b83caf6361d1baaf5b90b5a;


storage {
    _approvedRelayers: StorageMap<Address, StorageMap<Address, bool>> = StorageMap{ },
    _authorizer: b256 = 0xa3f865aa351e51cfeb40f5178d1564bb629fe9030b83caf6361d1baaf5b90b5a,
    TemporarilyPausable_contract_id: b256 =0xa3f865aa351e51cfeb40f5178d1564bb629fe9030b83caf6361d1baaf5b90b5a,
}

#[storage(read,write)]
fn has_approved_relayer(user: Address, relayer: Address)->bool {
    return storage._approvedRelayers.get(user).get(relayer);
}

/**
    * @dev Reverts unless `user` is the caller, or the caller is approved by the Authorizer to call the entry point
    * function (that is, it is a relayer for that function) and either:
    *  a) `user` approved the caller as a relayer (via `setRelayerApproval`), or
    *  b) a valid signature from them was appended to the calldata.
    */
#[storage(read,write)]
fn _authenticate_for(user: Address) {
    let sender: Result<Identity, AuthError> = msg_sender();
    let sender: Address = match sender.unwrap() {
        Identity::Address(addr) => {
            addr
        },
        _ => {
            revert(0);
        },
    };
     
    if sender != user {
        // In this context, 'permission to call a function' means 'being a relayer for a function'.
        // todo: workaround for this
        // _authenticate_caller();

        // Being a relayer is not sufficient: `user` must have also approved the caller either via
        // `setRelayerApproval`, or by providing a signature appended to the calldata.
        if (!has_approved_relayer(user, sender)) {
            // _validateSignature(user, Errors.USER_DOESNT_ALLOW_RELAYER);
            let tmp = true;
        }
    } 
}

//Todo work around
// fn _authenticate_caller() {
//         let actionId: b256 = get_action_id(msg.sig);

//         require(can_perform(actionId, msg_sender.unwrap()), SENDER_NOT_ALLOWED);
// }

// /**
//     * @dev Returns true if `user` approved `relayer` to act as a relayer for them.
//     */
// fn _has_approved_relayer(user: Address, relayer: Address) ->bool {
//     return _approvedRelayers[user][relayer];

// }


fn _can_perform(actionId: b256, user: Address) ->bool {
    // Access control is delegated to the Authorizer.
    // return can_perform(actionId, user, Contract_Id());
    true
}

fn _type_hash()-> b256 {
    // This is a simple switch-case statement, trivially written in Solidity by chaining else-if statements, but the
    // assembly implementation results in much denser bytecode.
    // solhint-disable-next-line no-inline-assembly
    let calldata = 0;
    let mut hash: b256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    hash
    // asm ( r1: 224, r2: calldata, r3, r4 ) {
    //     srwq r3 r2;
    //     srl r4 r1 r3;
    // }
    //     if r4 == 0xb95cac28 {
    //         sha256(_JOIN_TYPE_HASH)
    //     } else if r4 == 0x8bdb3913 {
    //         sha256(_EXIT_TYPE_HASH)
    //     }else if r4 == 0x52bbbe29 {
    //         sha256(_SWAP_TYPE_HASH)
    //     }else if r4 == 0x945bcec9 {
    //         sha256(_BATCH_SWAP_TYPE_HASH)
    //     }else if r4 == 0xfa6e671d {
    //         sha256(_SET_RELAYER_TYPE_HASH)
    //     }else {
    //         hash
    //     }
    }

impl VaultAuthorization for Contract {

    // fn init_vault_authorization (name: str[20], authorizer: b256 ) {
    //     init_signature_validator("Balancer V2 Vault")
    // }

    #[storage(read,write)]
    fn _set_authorizer(newAuthorizer: b256) {
        is_reentrant();
        storage._authorizer = newAuthorizer;
    }


    #[storage(read,write)]
    fn set_relayer_approval(
        sender: Address,
        relayer: Address,
        approved: bool
    ) {
        is_reentrant();
        let x = abi(TemporarilyPausable, storage.TemporarilyPausable_contract_id);
        x._ensure_not_paused();
        _authenticate_for(sender);
        // storage._approvedRelayers.insert(sender, storageMap {});
        // storage._approvedRelayers.get(relayer).insert(relayer, approved);
        log(RelayerApprovalChanged{
            relayer: relayer, sender: sender, approved: approved
        })

    }
}    
    // assembly {
    //     // The function selector is located at the first 4 bytes of calldata. We copy the first full calldata
    //     // 256 word, and then perform a logical shift to the right, moving the selector to the least significant
    //     // 4 bytes.
    //     tx_script_data_length
    //     let selector := shr(224, calldataload(0))

    //     // With the selector in the least significant 4 bytes, we can use 4 byte literals with leading zeros,
    //     // resulting in dense bytecode (PUSH4 opcodes).
    //     switch selector
    //         case 0xb95cac28 {
    //             hash := _JOIN_TYPE_HASH
    //         }
    //         case 0x8bdb3913 {
    //             hash := _EXIT_TYPE_HASH
    //         }
    //         case 0x52bbbe29 {
    //             hash := _SWAP_TYPE_HASH
    //         }
    //         case 0x945bcec9 {
    //             hash := _BATCH_SWAP_TYPE_HASH
    //         }
    //         case 0xfa6e671d {
    //             hash := _SET_RELAYER_TYPE_HASH
    //         }
    //         default {
    //             hash := 0x0000000000000000000000000000000000000000000000000000000000000000
    //         }
    // }


