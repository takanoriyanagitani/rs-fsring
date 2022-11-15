use std::path::Path;

use crate::evt::Event;
use crate::item::{Item, Name, NamedItem};
use crate::write;

/// Creates new pusher which uses closures to get/set name and write `NamedItem`.
pub fn push_new<G, S, W>(
    mut get_name: G,
    mut set_next: S,
    wtr: W,
) -> impl FnMut(Item) -> Result<(), Event>
where
    G: FnMut() -> Result<Name, Event>,
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

fn push_new_unmanaged<G, W>(mut get_name: G, wtr: W) -> impl FnMut(Item) -> Result<(), Event>
where
    G: FnMut() -> Result<Name, Event>,
    W: Fn(NamedItem) -> Result<Name, Event>,
{
    move |item: Item| {
        let name: Name = get_name()?;
        let named: NamedItem = NamedItem::new(item, name);
        wtr(named)?;
        Ok(())
    }
}

/// Creates new unmanaged push handler which uses closures to get `Name` and write `NamedItem`.
pub fn push_handler_new_unmanaged<G, W>(get_name: G, wtr: W) -> impl FnMut(Item) -> Event
where
    G: FnMut() -> Result<Name, Event>,
    W: Fn(NamedItem) -> Result<Name, Event>,
{
    let mut f = push_new_unmanaged(get_name, wtr);
    move |i: Item| f(i).map(|_| Event::Success).unwrap_or_else(|e| e)
}

/// Creates new checked unmanaged push handler which uses default writer to write `NamedItem`.
///
/// # Arguments
/// - get_name: Gets next name.
/// - dirname:  Path to store buffer files.
/// - checksum:     Computes checksum.
pub fn push_handler_new_unmanaged_default_with_checksum<G, P, C>(
    get_name: G,
    dirname: P,
    checksum: C,
) -> impl FnMut(Item) -> Event
where
    G: FnMut() -> Result<Name, Event>,
    P: AsRef<Path>,
    C: Fn(&[u8]) -> Vec<u8>,
{
    let wtr = write::writer_checked_new_default_with_checksum(dirname, checksum);
    push_handler_new_unmanaged(get_name, wtr)
}

fn checksum_nop(_: &[u8]) -> Vec<u8> {
    vec![]
}

/// Creates new unmanaged push handler which uses default writer to write `NamedItem`.
pub fn push_handler_new_unmanaged_default<G, P>(
    get_name: G,
    dirname: P,
) -> impl FnMut(Item) -> Event
where
    G: FnMut() -> Result<Name, Event>,
    P: AsRef<Path>,
{
    push_handler_new_unmanaged_default_with_checksum(get_name, dirname, checksum_nop)
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
