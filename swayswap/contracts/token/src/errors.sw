library errors;

pub enum Error {
    IdentityAlreadyMint: (),
    CannotReinitialize: (),
    MintIsClosed: (),
    NotOwner: (),
}
