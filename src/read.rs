use crate::err::RingError;
use crate::evt::Event;
use crate::item::{Name, NamedItem};

/// Creates new read handler.
pub fn new_read_handler<R>(r: R) -> impl Fn(&Name) -> Event
where
    R: Fn(&Name) -> Result<NamedItem, RingError>,
{
    move |name: &Name| match r(name) {
        Ok(named) => Event::ItemGot(named),
        Err(RingError::NoEntry) => Event::NoEntry(name.clone()),
        Err(RingError::Broken(_)) => Event::Broken(name.clone()),
        Err(RingError::InvalidItem(s)) => Event::InvalidItem(s),
        Err(e) => Event::UnexpectedError(e.into()),
    }
}
