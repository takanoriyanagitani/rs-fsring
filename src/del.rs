use std::path::PathBuf;

use crate::evt::Event;
use crate::item::Name;

/// Creates new remover which uses a closure to build `PathBuf` from `Name`.
///
/// Use this to keep hardlink if doing so is more SSD friendly.
pub fn truncate_fs_new<B>(path_builder: B) -> impl Fn(Name) -> Event
where
    B: Fn(&Name) -> PathBuf,
{
    move |n: Name| {
        let p: PathBuf = path_builder(&n);
        std::fs::File::create(p)
            .map(|_| Event::Empty(n))
            .unwrap_or_else(|e| Event::UnexpectedError(format!("{}", e)))
    }
}

/// Creates new delete request handler which uses closures to remove item/set removed name.
pub fn del_request_handler_new<T, S>(tr: T, mut set_removed: S) -> impl FnMut(Name) -> Event
where
    T: Fn(Name) -> Event,
    S: FnMut(Name) -> Result<(), Event>,
{
    move |n: Name| {
        let evt: Event = tr(n);
        match evt {
            Event::Empty(removed_name) => set_removed(removed_name)
                .map(|_| Event::Success)
                .unwrap_or_else(|e| Event::UnexpectedError(format!("{:#?}", e))),
            _ => evt,
        }
    }
}

#[cfg(test)]
mod test_del {

    mod del_request_handler_new {
        use crate::del;
        use crate::evt::Event;
        use crate::item::Name;

        #[test]
        fn test_removed() {
            let tr = |_: Name| Event::Empty(Name::from("42"));
            let set_removed = |_: Name| Ok(());
            let mut f = del::del_request_handler_new(tr, set_removed);
            let evt: Event = f(Name::from(""));
            assert_eq!(evt, Event::Success);
        }
    }
}
