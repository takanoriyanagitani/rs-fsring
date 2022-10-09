use crate::err::RingError;
use crate::item::{Item, Name, NamedItem};

/// Creates new list which uses a closure to determin if an name must be ignored.
pub fn new_list_filtered<L, F>(list: L, exists: F) -> impl Fn() -> Result<Vec<Name>, RingError>
where
    L: Fn() -> Result<Vec<Name>, RingError>,
    F: Fn(&Name) -> Result<bool, RingError>,
{
    move || {
        let v: Vec<Name> = list()?;
        v.into_iter()
            .map(|name| exists(&name).map(|yes: bool| yes.then_some(name)))
            .flat_map(|ron: Result<Option<Name>, RingError>| ron.transpose())
            .collect()
    }
}
