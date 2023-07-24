library;

/// Determines the type of error during initialisation.
///
/// ### Variants
///
/// * `BytecodeRootAlreadySet`: () - The exchange bytecode root has already been set.
/// * `BytecodeRootDoesNotMatch`: () - The exchange bytecode root passed in does not match the set exchange bytecode root.
/// * `BytecodeRootNotSet`: () - The exchange bytecode root has not been set.
/// * `PairDoesNotDefinePool`: () - The asset pair passed in does not match the asset pair from the set exchange bytecode root.
pub enum InitError {
    BytecodeRootAlreadySet: (),
    BytecodeRootDoesNotMatch: (),
    BytecodeRootNotSet: (),
    PairDoesNotDefinePool: (),
}
