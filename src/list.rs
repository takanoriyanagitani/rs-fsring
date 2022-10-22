use crate::err::RingError;
use crate::evt::Event;
use crate::item::Name;

pub fn names2checked<C>(names: Vec<Name>, check: C) -> Result<Vec<Name>, Event>
where
    C: Fn(&Name) -> Result<bool, Event>,
{
    let unchecked = names.into_iter();
    let mapd = unchecked.flat_map(|n: Name| {
        let r: Option<Result<Name, Event>> = match check(&n) {
            Ok(true) => Some(Ok(n)),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        };
        r
    });
    mapd.collect()
}

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

/// Creates new list handler which uses a closure to try to get names.
pub fn new_list_handler<L>(l: L) -> impl Fn() -> Event
where
    L: Fn() -> Result<Vec<Name>, RingError>,
{
    move || match l() {
        Ok(v) => Event::NamesGot(v),
        Err(RingError::NoEntry) => Event::NoPerm("Unable to get list of names.".into()),
        Err(e) => Event::UnexpectedError(e.into()),
    }
}

#[cfg(test)]
mod test_list {

    mod new_list_handler {

        use crate::evt::Event;
        use crate::list;

        #[test]
        fn test_empty() {
            let f = || Ok(vec![]);
            let l = list::new_list_handler(f);
            let evt: Event = l();
            assert_eq!(evt, Event::NamesGot(vec![]));
        }
    }
}
