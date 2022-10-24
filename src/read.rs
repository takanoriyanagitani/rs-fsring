use crate::err::RingError;
use crate::evt::Event;
use crate::item::{Name, NamedItem};

pub fn read_handler_new<R>(r: R) -> impl Fn(&Name) -> Event
where
    R: Fn(&Name) -> Result<NamedItem, Event>,
{
    move |name: &Name| match r(name) {
        Ok(named) => Event::ItemGot(named),
        Err(e) => e,
    }
}

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

#[cfg(test)]
mod test_read {

    mod new_read_handler {
        use crate::evt::Event;
        use crate::item::{Item, Name, NamedItem};
        use crate::read;

        #[test]
        fn test_ok() {
            let ni: NamedItem = NamedItem::new(Item::from(vec![]), Name::from(""));
            let f = |_: &Name| Ok(ni.clone());
            let r = read::new_read_handler(f);
            let evt: Event = r(&Name::from(""));
            assert_eq!(evt, Event::ItemGot(ni.clone()));
        }
    }
}
