use std::path::{Path, PathBuf};

use crate::item::Name;

/// Creates fullpath builder which converts name into Pathbuf.
pub fn fullpath_builder_new<P>(dirname: P) -> impl Fn(Name) -> PathBuf
where
    P: AsRef<Path>,
{
    move |name: Name| dirname.as_ref().join(name.as_str())
}

#[cfg(test)]
mod test_full {

    mod fullpath_builder_new {
        use std::path::{Path, PathBuf};

        use crate::full;
        use crate::item::Name;

        #[test]
        fn test_u8() {
            let dirname: String = "./test.d".into();
            let f = full::fullpath_builder_new(dirname);
            let pb: PathBuf = f(Name::from("42"));
            assert_eq!(pb, Path::new("./test.d/42"));
        }
    }
}
