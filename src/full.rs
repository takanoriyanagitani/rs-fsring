use std::path::{Path, PathBuf};

use crate::item::Name;

/// Creates fullpath builder which converts name into Pathbuf.
pub fn fullpath_builder_new<P>(dirname: P) -> impl Fn(Name) -> PathBuf
where
    P: AsRef<Path>,
{
    move |name: Name| dirname.as_ref().join(name.as_str())
}
