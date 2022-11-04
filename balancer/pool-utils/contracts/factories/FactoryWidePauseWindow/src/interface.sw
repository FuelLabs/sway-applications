library interface;

abi FactoryWidePauseWindow {
    #[storage(read)]fn get_pause_configuration() -> (u64, u64);
}