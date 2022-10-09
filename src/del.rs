use crate::err::RingError;
use crate::evt::Event;
use crate::item::Name;

/// Creates new delete handler which converts ENOENT error as Success.
pub fn new_delete_handler<D>(d: D) -> impl Fn(&Name) -> Event
where
    D: Fn(&Name) -> Result<(), RingError>,
{
    move |name: &Name| match d(name) {
        Ok(_) => Event::Success,
        Err(RingError::NoEntry) => Event::Success, // specified name absent.
        Err(e) => Event::UnexpectedError(e.into()),
    }
}
