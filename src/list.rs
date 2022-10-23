use crate::err::RingError;
use crate::evt::Event;
use crate::item::Name;

fn names2checked<C>(unchecked: Vec<Name>, check: &C) -> Result<Vec<Name>, Event>
where
    C: Fn(&Name) -> Result<bool, Event>,
{
    let unchecked = unchecked.into_iter();
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

fn list_filtered_new<L, F>(list: L, filter: F) -> impl Fn() -> Result<Vec<Name>, Event>
where
    L: Fn() -> Result<Vec<Name>, Event>,
    F: Fn(&Name) -> Result<bool, Event>,
{
    move || {
        let unchecked: Vec<Name> = list()?;
        names2checked(unchecked, &filter)
    }
}

/// Creates checked list handler which uses closures to get unchecked list and check names.
pub fn list_request_handler_new<L, F>(list: L, filter: F) -> impl Fn() -> Event
where
    L: Fn() -> Result<Vec<Name>, Event>,
    F: Fn(&Name) -> Result<bool, Event>,
{
    let f = list_filtered_new(list, filter);
    move || match f() {
        Ok(names) => Event::NamesGot(names),
        Err(e) => e,
    }
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

    mod list_request_handler_new {
        use crate::evt::Event;
        use crate::item::Name;
        use crate::list;

        #[test]
        fn test_empty() {
            let flist = || Ok(vec![]);
            let filter = |_: &Name| Ok(true);
            let f = list::list_request_handler_new(flist, filter);
            let evt: Event = f();
            assert_eq!(evt, Event::NamesGot(vec![]));
        }
    }
}
