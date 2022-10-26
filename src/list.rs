use std::path::Path;

use crate::empty;
use crate::evt::Event;
use crate::item::Name;

pub mod u;

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

/// Creates checked list handler which uses default (non-)empty checker.
pub fn list_request_handler_new_default<L, P>(list: L, dirname: P) -> impl Fn() -> Event
where
    L: Fn() -> Result<Vec<Name>, Event>,
    P: AsRef<Path>,
{
    let empty_checker = empty::empty_checker_new_default(dirname);
    let non_empty_checker = empty::nonempty_checker_new(empty_checker);
    let filter = move |n: &Name| non_empty_checker(n.clone());
    list_request_handler_new(list, filter)
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

        #[test]
        fn test_unable2get_list() {
            let flist = || Err(Event::NoPerm("".into()));
            let filter = |_: &Name| Ok(true);
            let f = list::list_request_handler_new(flist, filter);
            let evt: Event = f();
            assert_eq!(evt, Event::NoPerm("".into()));
        }

        #[test]
        fn test_empty_all() {
            let flist = || Ok(vec![Name::from("00"), Name::from("01")]);
            let filter = |_: &Name| Ok(false);
            let f = list::list_request_handler_new(flist, filter);
            let evt: Event = f();
            assert_eq!(evt, Event::NamesGot(vec![]));
        }

        #[test]
        fn test_nonempty_all() {
            let flist = || Ok(vec![Name::from("00"), Name::from("01")]);
            let filter = |_: &Name| Ok(true);
            let f = list::list_request_handler_new(flist, filter);
            let evt: Event = f();
            assert_eq!(
                evt,
                Event::NamesGot(vec![Name::from("00"), Name::from("01"),])
            );
        }

        #[test]
        fn test_filter_err() {
            let flist = || Ok(vec![Name::from("00")]);
            let filter = |_: &Name| Err(Event::NoPerm("".into()));
            let f = list::list_request_handler_new(flist, filter);
            let evt: Event = f();
            assert_eq!(evt, Event::NoPerm("".into()),);
        }
    }

    mod list_request_handler_new_default {
        use std::path::Path;

        use crate::evt::Event;
        use crate::item::Name;
        use crate::list;

        #[test]
        #[ignore]
        fn test_all_empty() {
            let dirname = Path::new("./test.d/list/list_request_handler_new_default/empty.d");
            std::fs::create_dir_all(&dirname).unwrap();

            let lst = || Ok(vec![Name::from("42"), Name::from("31")]);
            let f = list::list_request_handler_new_default(lst, dirname);
            let evt = f();
            assert_eq!(evt, Event::NamesGot(vec![]));
        }
    }
}
