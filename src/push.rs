use crate::evt::Event;
use crate::item::{Item, Name, NamedItem};

/// Creates new pusher which uses closures to get/set name and write `NamedItem`.
pub fn push_new<G, S, W>(
    get_name: G,
    mut set_next: S,
    wtr: W,
) -> impl FnMut(Item) -> Result<(), Event>
where
    G: Fn() -> Result<Name, Event>,
    S: FnMut(Name) -> Result<(), Event>,
    W: Fn(NamedItem) -> Result<Name, Event>,
{
    move |item: Item| {
        let name: Name = get_name()?;
        let named: NamedItem = NamedItem::new(item, name);
        match wtr(named) {
            Ok(prev) => {
                set_next(prev)?;
                Ok(())
            }
            Err(evt) => Err(evt),
        }
    }
}

#[cfg(test)]
mod test_push {

    mod push_new {

        use crate::evt::Event;
        use crate::item::{Item, Name, NamedItem};

        use crate::push;

        #[test]
        fn test_no_next() {
            let get_name = || Err(Event::TooManyItemsAlready);
            let set_next = |_: Name| Ok(());
            let wtr = |i: NamedItem| Ok(i.into());
            let mut p = push::push_new(get_name, set_next, wtr);
            let r = p(Item::from(vec![]));
            assert_eq!(r, Err(Event::TooManyItemsAlready));
        }
    }
}
