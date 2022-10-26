use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::evt::Event;
use crate::full;
use crate::item::Name;

fn truncate_as_del<P>(p: P) -> Result<(), ErrorKind>
where
    P: AsRef<Path>,
{
    File::create(p).map(|_| ()).map_err(|e| e.kind())
}

fn del_new<B>(path_builder: B) -> impl Fn(Name) -> Result<(), ErrorKind>
where
    B: Fn(Name) -> PathBuf,
{
    move |n: Name| {
        let p: PathBuf = path_builder(n);
        truncate_as_del(p)
    }
}

/// Creates new delete handler which uses a closure to build path from `Name`.
pub fn del_handler_new<B>(path_builder: B) -> impl Fn(Name) -> Event
where
    B: Fn(Name) -> PathBuf,
{
    let f = del_new(path_builder);
    move |n: Name| {
        f(n).map(|_| Event::Success).unwrap_or_else(|e| match e {
            ErrorKind::NotFound => Event::Success,
            _ => Event::UnexpectedError(format!("Unable to truncate: {}", e)),
        })
    }
}

/// Creates new delete handler which uses default path builder to build path from `Name`.
pub fn del_handler_new_default<P>(dirname: P) -> impl Fn(Name) -> Event
where
    P: AsRef<Path>,
{
    let path_builder = full::fullpath_builder_new(dirname);
    del_handler_new(path_builder)
}
