use crate::err::RingError;
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
