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
