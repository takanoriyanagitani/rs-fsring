use crate::err::RingError;
use crate::item::Name;

use crate::evt::Event;

/// Creates new empty which uses a closure to determin if an error means 'empty' or an actual error.
pub fn new_empty_or_else<E, F>(empty: E, f: F) -> impl Fn(&Name) -> Result<bool, RingError>
where
    E: Fn(&Name) -> Result<bool, RingError>,
    F: Fn(RingError) -> Result<bool, RingError>,
{
    move |name: &Name| empty(name).or_else(&f)
}

/// Creates new empty which converts noent as 'empty'.
pub fn new_empty_convert_noent2empty<E>(empty: E) -> impl Fn(&Name) -> Result<bool, RingError>
where
    E: Fn(&Name) -> Result<bool, RingError>,
{
    new_empty_or_else(empty, |re: RingError| match re {
        RingError::NoEntry => Ok(true),
        _ => Err(re),
    })
}

/// Creates new empty which uses a closure to determin if an item is 'empty'.
pub fn new_empty_by_len<L>(get_len: L) -> impl Fn(&Name) -> Result<bool, RingError>
where
    L: Fn(&Name) -> Result<u64, RingError>,
{
    move |name: &Name| {
        let l: u64 = get_len(name)?;
        Ok(0.eq(&l))
    }
}

pub fn new_next_retry_u8<N, E>(
    get_next: N,
    is_empty: E,
    retr_max: u8,
) -> impl Fn(Name) -> Result<Name, Event>
where
    N: Fn(Name) -> Result<Name, Event>,
    E: Fn(&Name) -> Result<bool, Event>,
{
    move |prev: Name| {
        let mut n1st: Name = prev;
        for _ in 0..retr_max {
            let nex: Name = get_next(n1st)?;
            match is_empty(&nex) {
                Ok(true) => return Ok(nex),
                Ok(false) => {}
                Err(_) => {}
            }
            n1st = nex;
        }
        Err(Event::TooManyItemsAlready)
    }
}

#[cfg(test)]
mod test_empty {

    mod new_empty_by_len {

        use crate::item::Name;

        use crate::empty;

        #[test]
        fn test_zero_len() {
            let f = |_: &Name| Ok(0);
            let ef = empty::new_empty_by_len(f);
            let is_empty: bool = ef(&Name::from("")).unwrap();
            assert_eq!(is_empty, true);
        }
    }

    mod new_next_retry_u8 {
        use crate::empty;
        use crate::evt::Event;
        use crate::item::Name;

        #[test]
        fn test_zero() {
            let get_next = |n: Name| Ok(n);
            let is_empty = |_: &Name| Err(Event::Again);
            let f = empty::new_next_retry_u8(get_next, is_empty, 0);
            let r = f(Name::from(""));
            assert_eq!(r, Err(Event::TooManyItemsAlready));
        }

        #[test]
        fn test_1() {
            let get_next = |n: Name| Ok(n);
            let is_empty = |_: &Name| Err(Event::Again);
            let f = empty::new_next_retry_u8(get_next, is_empty, 1);
            let r = f(Name::from(""));
            assert_eq!(r, Err(Event::TooManyItemsAlready));
        }

        #[test]
        fn test_fresh_prev() {
            let get_next = |_: Name| Ok(Name::from("42"));
            let is_empty = |_: &Name| Ok(true);
            let f = empty::new_next_retry_u8(get_next, is_empty, 1);
            let n: Name = f(Name::from("")).unwrap();
            assert_eq!(n, Name::from("42"));
        }
    }
}
