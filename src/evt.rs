use crate::item::{Item, Name, NamedItem};

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Success,

    /// The named buffer is now empty.
    Empty(Name),

    /// The named buffer is used.
    Used(Name),

    /// The named buffer wrote(can be dirty; implementation may return before fdatasync).
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

impl TryFrom<Event> for Vec<Name> {
    type Error = Event;
    fn try_from(v: Event) -> Result<Self, Self::Error> {
        match v {
            Event::NamesGot(names) => Ok(names),
            _ => Err(Event::BadRequest),
        }
    }
}

impl TryFrom<Event> for Item {
    type Error = Event;
    fn try_from(v: Event) -> Result<Self, Self::Error> {
        match v {
            Event::ItemGot(named) => Ok(named.into_item()),
            _ => Err(Event::BadRequest),
        }
    }
}

impl TryFrom<Event> for Name {
    type Error = Event;
    fn try_from(v: Event) -> Result<Self, Self::Error> {
        match v {
            Event::Empty(name) => Ok(name),
            Event::Used(name) => Ok(name),
            Event::ItemWrote(name) => Ok(name),
            Event::NoEntry(name) => Ok(name),
            Event::Broken(name) => Ok(name),
            Event::ItemGot(named) => Ok(Name::from(named)),
            _ => Err(Event::BadRequest),
        }
    }
}
