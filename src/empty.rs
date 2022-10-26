use std::fs::Metadata;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::compose::compose;

use crate::item::Name;

use crate::evt::Event;
use crate::full;

fn len2empty(l: u64) -> bool {
    0 == l
}

fn meta2empty_new() -> impl Fn(Metadata) -> bool {
    compose(|m: Metadata| m.len(), len2empty)
}

fn kind2empty(k: ErrorKind) -> Result<bool, Event> {
    match k {
        ErrorKind::NotFound => Ok(true),
        _ => Err(Event::UnexpectedError(format!("{}", k))),
    }
}

fn path2empty<P>(p: P) -> Result<bool, Event>
where
    P: AsRef<Path>,
{
    let m2e = meta2empty_new();
    match std::fs::metadata(p) {
        Ok(m) => Ok(m2e(m)),
        Err(e) => kind2empty(e.kind()),
    }
}

/// Creates new empty checker which uses a closure to generate `PathBuf` from `Name`.
///
/// `std::io::ErrorKind::NotFound` will be converted to Ok(true).
pub fn name2empty_fs_new<F>(f: F) -> impl Fn(Name) -> Result<bool, Event>
where
    F: Fn(Name) -> PathBuf,
{
    compose(f, path2empty)
}

/// Creates non-empty checker which uses a closure to determin if the named item is empty.
pub fn nonempty_checker_new<E>(empty_checker: E) -> impl Fn(Name) -> Result<bool, Event>
where
    E: Fn(Name) -> Result<bool, Event>,
{
    move |n: Name| empty_checker(n).map(|empty: bool| !empty)
}

/// Creates default empty checker which uses dirname to create path builder.
pub fn empty_checker_new_default<P>(dirname: P) -> impl Fn(Name) -> Result<bool, Event>
where
    P: AsRef<Path>,
{
    let builder = full::fullpath_builder_new(dirname);
    name2empty_fs_new(builder)
}

#[cfg(test)]
mod test_empty {

    mod len2empty {
        use crate::empty;

        #[test]
        fn test_zero() {
            assert_eq!(true, empty::len2empty(0));
        }

        #[test]
        fn test_non0() {
            assert_eq!(false, empty::len2empty(42));
        }
    }

    mod kind2empty {
        use crate::empty;
        use std::io::ErrorKind;

        #[test]
        fn test_noent() {
            assert_eq!(true, empty::kind2empty(ErrorKind::NotFound).unwrap());
        }

        #[test]
        fn test_err() {
            let r = empty::kind2empty(ErrorKind::PermissionDenied);
            assert_eq!(true, r.is_err());
        }
    }

    mod nonempty_checker_new {
        use crate::empty;
        use crate::item::Name;

        #[test]
        fn test_empty() {
            let echk = |_: Name| Ok(true);
            let nchk = empty::nonempty_checker_new(echk);
            let not_empty: bool = nchk(Name::from("")).unwrap();
            assert_eq!(not_empty, false);
        }
    }

    mod empty_checker_new_default {
        use std::fs::File;
        use std::path::Path;

        use crate::empty;
        use crate::item::Name;

        #[test]
        #[ignore]
        fn test_empty() {
            let dirname = Path::new("./test.d/empty/empty_checker_new_default/empty.d");
            std::fs::create_dir_all(&dirname).unwrap();
            let name: &str = "00";
            let n: Name = Name::from(name);
            File::create(dirname.join(name)).unwrap();
            let f = empty::empty_checker_new_default(dirname);
            let empty: bool = f(n).unwrap();
            assert_eq!(empty, true);
        }
    }
}
