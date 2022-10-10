use std::fs::remove_file;
use std::io::ErrorKind;
use std::path::Path;

use crate::err::RingError;
use crate::evt::Event;
use crate::item::Name;

use crate::compose::compose;
use crate::full::fullpath_builder_new;

/// Creates new delete handler which converts ENOENT error as Success.
pub fn new_delete_handler<D>(d: D) -> impl Fn(&Name) -> Event
where
    D: Fn(&Name) -> Result<(), RingError>,
{
    move |name: &Name| match d(name) {
        Ok(_) => Event::Success,
        Err(RingError::NoEntry) => Event::Success, // specified name absent.
        Err(e) => Event::UnexpectedError(e.into()),
    }
}

fn remove_by_fullpath<P>(full: P) -> Event
where
    P: AsRef<Path>,
{
    match remove_file(full) {
        Ok(_) => Event::Success,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Event::Success,
            _ => Event::UnexpectedError(format!("io error kind: {}", e.kind())),
        },
    }
}

fn remover_unchecked_new<P>(dirname: P) -> impl Fn(Name) -> Event
where
    P: AsRef<Path>,
{
    let builder = fullpath_builder_new(dirname);
    compose(builder, remove_by_fullpath)
}

/// Creates new checked delete handler which tries to remove by checked name.
pub fn delete_handler_new_checked<C, P>(checker: C, dirname: P) -> impl Fn(Name) -> Event
where
    C: Fn(Name) -> Result<Name, Event>,
    P: AsRef<Path>,
{
    let remover = remover_unchecked_new(dirname);
    move |name: Name| match checker(name) {
        Ok(n) => remover(n),
        Err(evt) => evt,
    }
}
