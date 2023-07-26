library errors;

pub enum InitError {
    CannotReinitialize: (),
    NotInitialized: (),
}

// TODO: can rename back to AccessError once https://github.com/FuelLabs/fuels-rs/issues/791 is fixed
pub enum ValidityError {
    MaxTokensMinted: (),
    NoContractAdmin: (),
    SenderNotAdmin: (),
}
