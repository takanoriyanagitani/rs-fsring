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
