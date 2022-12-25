library errors;

pub enum Error {
    AuthorizationError: (),
    DuplicateTransaction: (),
    TransactionCancelled: (),
}
