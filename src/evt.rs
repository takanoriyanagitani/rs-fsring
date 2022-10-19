use crate::item::{Name, NamedItem};

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Success,

    /// The named buffer is now empty.
    Removed(Name),

    /// The named buffer wrote(can be dirty; implementation may intentionally skip fsync).
    ItemWrote(Name),

    /// Named item got.
    ItemGot(NamedItem),

    /// Nothing to get or nothing to remove.
    NoEntry(Name),

    /// Unable to push an item(storage or buffer full).
    TooManyItemsAlready,

    /// Unexpected error(unable to check item existence, ...).
    NoPerm(String),

    /// Specified name already exists(retry with next name).
    Again,

    /// Invalid request
    BadRequest,

    /// Item names got.
    NamesGot(Vec<Name>),

    /// Item got, but its contents broken(power failure?).
    Broken(Name),

    /// Item got, but unreadable(bit rot?).
    InvalidItem(String),

    UnexpectedError(String),
}
