use std::fs::Metadata;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::compose::compose;

use crate::item::Name;

use crate::evt::Event;

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

pub fn name2empty_fs_new<F>(f: F) -> impl Fn(Name) -> Result<bool, Event>
where
    F: Fn(Name) -> PathBuf,
{
    compose(f, path2empty)
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
}
