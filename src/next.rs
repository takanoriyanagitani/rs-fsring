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
}
