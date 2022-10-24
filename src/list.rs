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

#[cfg(test)]
mod test_list {

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
