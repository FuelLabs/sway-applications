// ANCHOR: module
library style_guide;
// ANCHOR_END: module
// ANCHOR: const
const MAXIMUM_DEPOSIT = 10;
// ANCHOR_END: const
// ANCHOR: structures
struct MultiSignatureWallet {
    owner_count: u64,
}

trait MetaData {
    // code
}

enum DepositError {
    IncorrectAmount: (),
    IncorrectAsset: (),
}
// ANCHOR_END: structures
// ANCHOR: function_case
fn authorize_user(user: Identity) {
    let blacklist_user = false;
    // code
}
// ANCHOR_END: function_case
// ANCHOR: getters
// Discouraged style
fn get_maximum_deposit() -> u64 {
    MAXIMUM_DEPOSIT
}

// Encouraged style
fn maximum_deposit() -> u64 {
    MAXIMUM_DEPOSIT
}
// ANCHOR_END: getters
// ANCHOR: type_annotation
fn execute() {
    // Avoid unless it's more helpful to annotate
    let executed: bool = false;

    // Generally encouraged
    let executed = false;
}
// ANCHOR_END: type_annotation
// ANCHOR: struct_shorthand_definition
struct Structure {
    amount: u64,
}
// ANCHOR_END: struct_shorthand_definition
// ANCHOR: struct_shorthand_use
fn call(amount: u64) {
    let structure = Structure { amount };
}
// ANCHOR_END: struct_shorthand_use
// ANCHOR: struct_shorthand_avoid
fn action(value: u64) {
    let amount = value;
    let structure = Structure { amount: value };
    let structure = Structure { amount: amount };
}
// ANCHOR_END: struct_shorthand_avoid
