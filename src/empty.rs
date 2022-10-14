use crate::err::RingError;
use crate::item::Name;

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
}
