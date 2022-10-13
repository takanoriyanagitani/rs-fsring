use crate::err::RingError;
use crate::evt::Event;
use crate::item::{Item, Name, NamedItem};

/// Creates checked writer from writer which uses a closure to determin if an name already exists.
pub fn new_checked_writer<W, E>(writer: W, exists: E) -> impl Fn(NamedItem) -> Result<(), RingError>
where
    W: Fn(NamedItem) -> Result<(), RingError>,
    E: Fn(&Name) -> Result<bool, RingError>,
{
    move |named_item| match exists(named_item.as_name()) {
        Ok(true) => Err(RingError::Exist),
        Ok(false) => writer(named_item),
        Err(e) => Err(RingError::ItemMayExists(e.into())),
    }
}

/// Creates new push which gets an name from name source for writer.
pub fn new_push<N, W>(name_source: N, writer: W) -> impl Fn(Item) -> Result<(), RingError>
where
    N: Fn() -> Result<Name, RingError>,
    W: Fn(NamedItem) -> Result<(), RingError>,
{
    move |item: Item| {
        let name: Name = name_source()?;
        let named: NamedItem = NamedItem::new(item, name);
        writer(named)
    }
}

/// Creates new push request handler.
pub fn new_push_request_handler<P>(p: P) -> impl Fn(Item) -> Event
where
    P: Fn(Item) -> Result<(), RingError>,
{
    move |item: Item| match p(item) {
        Ok(_) => Event::Success,
        Err(RingError::NoSpace) => Event::TooManyItemsAlready,
        Err(RingError::Exist) => Event::Again,
        Err(RingError::ItemMayExists(s)) => Event::NoPerm(s),
        Err(RingError::Locked(s)) => Event::NoPerm(s),
        Err(e) => Event::UnexpectedError(e.into()),
    }
}

#[cfg(test)]
mod test_push {

    mod new_push_request_handler {
        use crate::err::RingError;
        use crate::evt::Event;
        use crate::item::Item;
        use crate::push;

        #[test]
        fn test_ok() {
            let p = |_: Item| Ok(());
            let f = push::new_push_request_handler(p);
            let evt: Event = f(Item::from(vec![]));
            assert_eq!(evt, Event::Success);
        }

        #[test]
        fn test_toomany() {
            let p = |_: Item| Err(RingError::NoSpace);
            let f = push::new_push_request_handler(p);
            let evt: Event = f(Item::from(vec![]));
            assert_eq!(evt, Event::TooManyItemsAlready);
        }
    }
}
