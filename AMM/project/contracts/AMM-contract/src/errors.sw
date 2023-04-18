library;

pub enum InitError {
    BytecodeRootAlreadySet: (),
    BytecodeRootDoesNotMatch: (),
    BytecodeRootNotSet: (),
    PairDoesNotDefinePool: (),
}
