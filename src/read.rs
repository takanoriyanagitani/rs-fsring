use crate::err::RingError;
use crate::evt::Event;
use crate::item::{Item, Name, NamedItem};

/// Creates new reader which uses a closure to determin if an error means 'noent' or an actual error.
pub fn read_or_else<R, F>(r: R, f: F) -> impl Fn(&Name) -> Result<Option<Item>, RingError>
where
    R: Fn(&Name) -> Result<Item, RingError>,
    F: Fn(RingError) -> Result<Option<Item>, RingError>,
{
    move |name: &Name| match r(name) {
        Ok(item) => Ok(Some(item)),
        Err(e) => f(e),
    }
}

/// Creates new reader which converts specified error as 'noent'.
pub fn read_ignore_error<R>(r: R, e: RingError) -> impl Fn(&Name) -> Result<Option<Item>, RingError>
where
    R: Fn(&Name) -> Result<Item, RingError>,
{
    read_or_else(r, |re: RingError| match re {
        e => Ok(None),
        _ => Err(re),
    })
}

/// Creates new reader which converts noent as 'no entry'.
pub fn read_ignore_noent<R>(r: R) -> impl Fn(&Name) -> Result<Option<Item>, RingError>
where
    R: Fn(&Name) -> Result<Item, RingError>,
{
    read_ignore_error(r, RingError::NoEntry)
}

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
