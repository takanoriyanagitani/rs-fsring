use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::empty;
use crate::evt::Event;
use crate::full;
use crate::item::{Item, Name, NamedItem};

fn write_bytes<W>(w: &mut W, b: &[u8]) -> Result<(), Event>
where
    W: Write,
{
    w.write_all(b)
        .map_err(|e| Event::UnexpectedError(format!("Unable to write bytes: {}", e)))
}

fn write_flush<W>(mut w: W) -> Result<(), Event>
where
    W: Write,
{
    w.flush()
        .map_err(|e| Event::UnexpectedError(format!("Unable to flush: {}", e)))
}

fn item2write<W>(i: Item, mut w: W) -> Result<(), Event>
where
    W: Write,
{
    let mut bw = BufWriter::new(w.by_ref());
    let v: Vec<u8> = i.into();
    write_bytes(&mut bw, &v)?;
    write_flush(bw)?;
    write_flush(w)?;
    Ok(())
}

fn item2path<P>(i: Item, p: P) -> Result<(), Event>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let f: File = File::create(p.as_ref()).map_err(|e| {
        Event::UnexpectedError(format!("Unable to create named item({:#?}): {}", p, e))
    })?;
    item2write(i, f)
}

/// Creates new unchecked writer which uses a closure to build path to write a named item.
pub fn writer_unchecked_new<B>(path_builder: B) -> impl Fn(NamedItem) -> Result<Name, Event>
where
    B: Fn(Name) -> PathBuf,
{
    move |named: NamedItem| {
        let (name, item) = named.into_pair();
        let p: PathBuf = path_builder(name.clone());
        item2path(item, p)?;
        Ok(name)
    }
}

/// Creates new unchecked writer which uses default path builder.
pub fn writer_unchecked_new_default<P>(dirname: P) -> impl Fn(NamedItem) -> Result<Name, Event>
where
    P: AsRef<Path>,
{
    let path_builder = full::fullpath_builder_new(dirname);
    writer_unchecked_new(path_builder)
}

/// Creates checked writer which uses closures to check and write named item.
///
/// # Arguments
/// - unchecked: Writes `NamedItem` after check.
/// - is_empty:  Checks if `Name` is empty.
pub fn writer_checked_new<W, E>(
    unchecked: W,
    is_empty: E,
) -> impl Fn(NamedItem) -> Result<Name, Event>
where
    W: Fn(NamedItem) -> Result<Name, Event>,
    E: Fn(&Name) -> Result<bool, Event>,
{
    move |named: NamedItem| {
        let n: &Name = named.as_name();
        match is_empty(n) {
            Ok(true) => unchecked(named),
            Ok(false) => Err(Event::Again),
            e => Err(Event::UnexpectedError(format!("{:#?}", e))),
        }
    }
}

/// Creates new checked writer which uses default unchecked writer and default empty checker.
pub fn writer_checked_new_default<P>(dirname: P) -> impl Fn(NamedItem) -> Result<Name, Event>
where
    P: AsRef<Path>,
{
    let p: &Path = dirname.as_ref();
    let unchecked = writer_unchecked_new_default(p.to_path_buf());
    let empty_checker = empty::empty_checker_new_default(p.to_path_buf());
    let f = move |n: &Name| empty_checker(n.clone());
    writer_checked_new(unchecked, f)
}

#[cfg(test)]
mod test_write {

    mod writer_checked_new {
        use crate::evt::Event;
        use crate::item::{Item, Name, NamedItem};
        use crate::write;

        #[test]
        fn test_non_empty() {
            let unchecked = |_: NamedItem| Ok(Name::from(""));
            let is_empty = |_: &Name| Ok(false);
            let f = write::writer_checked_new(unchecked, is_empty);
            let r = f(NamedItem::new(Item::from(vec![]), Name::from("")));
            assert_eq!(r, Err(Event::Again));
        }
    }

    mod writer_unchecked_new_default {
        use std::io::ErrorKind;
        use std::path::Path;

        use crate::item::{Item, Name, NamedItem};
        use crate::write;

        #[test]
        #[ignore]
        fn test_dir_noent() {
            let dirname = Path::new("./test.d/write/writer_unchecked_new_default/dir_noent.d");
            std::fs::remove_dir_all(&dirname)
                .map(|_| ())
                .map_err(|e| e.kind())
                .or_else(|k| match k {
                    ErrorKind::NotFound => Ok(()),
                    _ => Err(k),
                })
                .unwrap();
            let f = write::writer_unchecked_new_default(dirname);
            let r = f(NamedItem::new(Item::from(vec![]), Name::from("empty.dat")));
            assert_eq!(r.is_err(), true);
        }
    }
}
