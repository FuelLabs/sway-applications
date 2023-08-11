library;

/// Determines the type of error during initialisation.
pub enum InitError {
    /// The exchange bytecode root has already been set.
    BytecodeRootAlreadySet: (),
    /// The exchange bytecode root passed in does not match the set exchange bytecode root.
    BytecodeRootDoesNotMatch: (),
    /// The exchange bytecode root has not been set.
    BytecodeRootNotSet: (),
    /// The asset pair passed in does not match the asset pair from the set exchange bytecode root.
    PairDoesNotDefinePool: (),
}
