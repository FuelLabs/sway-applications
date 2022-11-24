library errors;

pub enum InitError {
    BytecodeRootAlreadySet: (),
    BytecodeRootDoesNotMatch: (),
    BytecodeRootNotSet: (),
    PairDoesNotDefinePool: (),
}
