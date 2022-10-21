use crate::evt::Event;
use crate::item::Name;

pub fn get_next_simple_retry<N>(get_next: N, limit: usize) -> Event
where
    N: FnMut() -> Option<Result<Name, Event>>,
{
    let i = std::iter::from_fn(get_next);
    let l = i.take(limit);
    let mut f = l.flat_map(|r| r.ok());
    f.next()
        .map(Event::Empty)
        .unwrap_or_else(|| Event::TooManyItemsAlready)
}

pub fn get_next_checked_new<N, E>(
    mut get_next_unchecked: N,
    is_empty: E,
) -> impl FnMut() -> Result<Name, Event>
where
    N: FnMut() -> Result<Name, Event>,
    E: Fn(&Name) -> Result<bool, Event>,
{
    move || {
        let n: Name = get_next_unchecked()?;
        match is_empty(&n) {
            Ok(true) => Ok(n),
            Err(Event::NoEntry(n)) => Ok(n),
            Ok(false) => Err(Event::Used(n)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test_next {

    mod get_next_simple_retry {

        use crate::evt::Event;
        use crate::item::Name;
        use crate::next;

        #[test]
        fn test_empty_candidates() {
            let get_next = || None;
            let limit = 256;
            let evt: Event = next::get_next_simple_retry(get_next, limit);
            assert_eq!(evt, Event::TooManyItemsAlready);
        }

        #[test]
        fn test_no_empty() {
            let get_next = || Some(Err(Event::Used(Name::from(""))));
            let limit = 256;
            let evt: Event = next::get_next_simple_retry(get_next, limit);
            assert_eq!(evt, Event::TooManyItemsAlready);
        }

        #[test]
        fn test_empty1st() {
            let get_next = || Some(Ok(Name::from("42")));
            let limit = 256;
            let evt: Event = next::get_next_simple_retry(get_next, limit);
            assert_eq!(evt, Event::Empty(Name::from("42")));
        }
    }

    mod get_next_checked_new {

        use crate::evt::Event;
        use crate::item::Name;
        use crate::next;

        #[test]
        fn test_empty() {
            let get_next_unchecked = || Ok(Name::from("42"));
            let is_empty = |_: &Name| Ok(true);

            let mut f = next::get_next_checked_new(get_next_unchecked, is_empty);
            let n: Name = f().unwrap();
            assert_eq!(n, Name::from("42"));
        }

        #[test]
        fn test_noent() {
            let get_next_unchecked = || Ok(Name::from(""));
            let is_empty = |_: &Name| Err(Event::NoEntry(Name::from("42")));

            let mut f = next::get_next_checked_new(get_next_unchecked, is_empty);
            let n: Name = f().unwrap();
            assert_eq!(n, Name::from("42"));
        }

        #[test]
        fn test_used() {
            let get_next_unchecked = || Ok(Name::from("42"));
            let is_empty = |_: &Name| Ok(false);

            let mut f = next::get_next_checked_new(get_next_unchecked, is_empty);
            let r = f();
            assert_eq!(r, Err(Event::Used(Name::from("42"))));
        }

        #[test]
        fn test_unexpected() {
            let get_next_unchecked = || Ok(Name::from(""));
            let is_empty = |_: &Name| Err(Event::Broken(Name::from("42")));

            let mut f = next::get_next_checked_new(get_next_unchecked, is_empty);
            let r = f();
            assert_eq!(r, Err(Event::Broken(Name::from("42"))));
        }
    }
}
