use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};
use std::path::{Path, PathBuf};

use crate::evt::Event;
use crate::full;
use crate::item::{Item, Name, NamedItem};

fn read2buf<R>(r: R, buf: &mut Vec<u8>) -> Result<(), ErrorKind>
where
    R: Read,
{
    let mut br = BufReader::new(r);
    br.read_to_end(buf).map_err(|e| e.kind())?;
    Ok(())
}

fn read2item<R>(r: R) -> Result<Item, ErrorKind>
where
    R: Read,
{
    let mut buf: Vec<u8> = Vec::new();
    read2buf(r, &mut buf).map(|_| buf).map(Item::from)
}

fn path2item<P>(p: P) -> Result<Item, ErrorKind>
where
    P: AsRef<Path>,
{
    let f: File = File::open(p).map_err(|e| e.kind())?;
    read2item(f)
}

/// Creates new read handler which uses a closure to build path from `Name`.
pub fn read_handler_new<B>(path_builder: B) -> impl Fn(Name) -> Event
where
    B: Fn(Name) -> PathBuf,
{
    move |n: Name| {
        let p: PathBuf = path_builder(n.clone());
        match path2item(p) {
            Ok(item) => Event::ItemGot(NamedItem::new(item, n)),
            Err(ErrorKind::NotFound) => Event::NoEntry(n),
            Err(e) => Event::UnexpectedError(format!("Unable to get named item: {}", e)),
        }
    }
}

/// Creates default read handler which uses default path builder.
pub fn read_handler_new_default<P>(dirname: P) -> impl Fn(Name) -> Event
where
    P: AsRef<Path>,
{
    let path_builder = full::fullpath_builder_new(dirname);
    read_handler_new(path_builder)
}

#[cfg(test)]
mod test_read {

    mod read2item {
        use crate::item::Item;
        use crate::read;

        #[test]
        fn test_small() {
            let content: &[u8] = &[0x42];
            let i: Item = read::read2item(content).unwrap();
            assert_eq!(i, Item::from(vec![0x42]));
        }
    }

    mod read_handler_new_default {
        use std::path::Path;

        use crate::evt::Event;
        use crate::item::Name;
        use crate::read;

        #[test]
        fn test_empty() {
            let dirname = Path::new("./test.d/read/read_handler_new_default/empty.d");
            std::fs::create_dir_all(&dirname).unwrap();
            let f = read::read_handler_new_default(dirname);
            let evt: Event = f(Name::from("not-exist.dat"));
            assert_eq!(evt, Event::NoEntry(Name::from("not-exist.dat")));
        }
    }
}
