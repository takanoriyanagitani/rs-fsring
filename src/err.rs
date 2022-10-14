#[derive(Debug)]
pub enum RingError {
    /// Nothing to get, list or remove.
    NoEntry,

    /// No space to push an item.
    NoSpace,

    /// Must not overwrite.
    Exist,

    /// Unable to check item existence(I/O error?).
    ItemMayExists(String),

    /// An item exists, but broken(power failure?).
    Broken(String),

    /// An item exists, but unreadable(bit rot?).
    InvalidItem(String),

    /// No perm to write/remove an item.
    Locked(String),
}

impl From<RingError> for String {
    fn from(e: RingError) -> Self {
        match e {
            RingError::NoEntry => String::from("No such item"),
            RingError::NoSpace => String::from("No more space"),
            RingError::Exist => String::from("Must not overwrite"),
            RingError::Broken(s) => format!("Broken item: {}", s),
            RingError::ItemMayExists(s) => format!("Item may exists or missing: {}", s),
            RingError::InvalidItem(s) => format!("Item unreadable: {}", s),
            RingError::Locked(s) => format!("Unable to remove: {}", s),
        }
    }
}
