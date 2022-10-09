use crate::err::RingError;
use crate::item::Name;

/// Creates new remover which uses a closure to convert an error.
pub fn delete_or_else<D, F>(d: D, f: F) -> impl Fn(&Name) -> Result<(), RingError>
where
    F: Fn(RingError) -> Result<(), RingError>,
    D: Fn(&Name) -> Result<(), RingError>,
{
    move |name: &Name| d(name).or_else(&f)
}

/// Creates new remover which ignores specified error.
pub fn delete_ignore_err<D>(d: D, e: RingError) -> impl Fn(&Name) -> Result<(), RingError>
where
    D: Fn(&Name) -> Result<(), RingError>,
{
    let f = |re: RingError| match re {
        e => Ok(()),
        _ => Err(re),
    };
    delete_or_else(d, f)
}
