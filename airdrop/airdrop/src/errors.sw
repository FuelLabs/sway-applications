library errors;

enum AccessError {
    UserAlreadyClaimed: (),
}

enum StateError {
    AlreadyInitalized: (),
    NotInitalized: (),
}
