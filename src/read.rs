use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};
use std::path::{Path, PathBuf};

use crate::evt::Event;
use crate::full;
use crate::item::{Item, Name, NamedItem};

fn kind2event(n: Name, k: ErrorKind) -> Event {
    match k {
        // the buffer empty.
        ErrorKind::NotFound => Event::NoEntry(n),

        // permanent error. the buffer must be removed.
        ErrorKind::PermissionDenied => Event::Broken(n),

        // may be broken(bit rot detected by btrfs?)
        ErrorKind::InvalidInput => Event::Broken(n),
        ErrorKind::InvalidData => Event::Broken(n),
        ErrorKind::Unsupported => Event::Broken(n),
        ErrorKind::UnexpectedEof => Event::Broken(n),

        // Try again(network file system?).
        ErrorKind::TimedOut => Event::Again,

        // Try again.
        ErrorKind::OutOfMemory => Event::Again,

        _ => Event::UnexpectedError(format!("Unable to get named item: {}", k)),
    }
}

fn err2event(n: Name, e: std::io::Error, io_error_num: i32) -> Event {
    e.raw_os_error()
        .and_then(|raw_err_num: i32| {
            raw_err_num
                .eq(&io_error_num)
                .then(|| Event::Broken(n.clone()))
        })
        .unwrap_or_else(|| kind2event(n, e.kind()))
}

fn read2buf<R>(r: R, buf: &mut Vec<u8>) -> Result<(), std::io::Error>
where
    R: Read,
{
    let mut br = BufReader::new(r);
    br.read_to_end(buf).map(|_| ())
}

fn data2checked(
    n: Name,
    data: Vec<u8>,
    checksum: Vec<u8>,
    computed: Vec<u8>,
) -> Result<Item, Event> {
    (checksum == computed)
        .then_some(data)
        .map(Item::from)
        .ok_or(Event::Broken(n))
}

fn raw2item_with_checksum<C>(
    n: Name,
    mut raw: Vec<u8>,
    checksize: usize,
    checksum: &C,
) -> Result<Item, Event>
where
    C: Fn(&[u8]) -> Vec<u8>,
{
    let split_point: usize = raw
        .len()
        .checked_sub(checksize)
        .ok_or_else(|| Event::Broken(n.clone()))?;
    let chk: Vec<u8> = raw.split_off(split_point);
    let computed: Vec<u8> = checksum(&raw);
    data2checked(n, raw, chk, computed)
}

fn read2item_with_checksum<R, C>(
    n: Name,
    r: R,
    checksize: usize,
    checksum: &C,
    io_error_num: i32,
) -> Result<Item, Event>
where
    R: Read,
    C: Fn(&[u8]) -> Vec<u8>,
{
    let mut buf: Vec<u8> = Vec::new();
    read2buf(r, &mut buf)
        .map_err(|e| err2event(n.clone(), e, io_error_num))
        .and_then(|_| raw2item_with_checksum(n, buf, checksize, checksum))
}

fn path2item_with_checksum<P, C>(
    n: Name,
    p: P,
    checksize: usize,
    checksum: &C,
    io_err_num: i32,
) -> Result<Item, Event>
where
    P: AsRef<Path>,
    C: Fn(&[u8]) -> Vec<u8>,
{
    File::open(p)
        .map_err(|e| err2event(n.clone(), e, io_err_num))
        .and_then(|f: File| read2item_with_checksum(n, f, checksize, checksum, io_err_num))
}

/// Creates default checked read handler which uses default path builder.
///
/// # Arguments
/// - path_builder: Builds a path for a named item.
/// - checksize: Checksum byte length.
/// - checksum:  Computes checksum.
pub fn read_handler_new_with_checksum<B, C>(
    path_builder: B,
    checksize: usize,
    checksum: C,
) -> impl Fn(Name) -> Event
where
    B: Fn(Name) -> PathBuf,
    C: Fn(&[u8]) -> Vec<u8>,
{
    move |n: Name| {
        let p: PathBuf = path_builder(n.clone());
        // libc::EIO = 5(linux, windows, macos)
        match path2item_with_checksum(n.clone(), p, checksize, &checksum, 5) {
            Ok(item) => Event::ItemGot(NamedItem::new(item, n)),
            Err(e) => e,
        }
    }
}

/// Creates default checked read handler which uses default path builder.
///
/// # Arguments
/// - dirname: Path to open buffer files.
/// - checksize: Checksum byte length.
/// - checksum:  Computes checksum.
pub fn read_handler_new_default_with_checksum<P, C>(
    dirname: P,
    checksize: usize,
    checksum: C,
) -> impl Fn(Name) -> Event
where
    P: AsRef<Path>,
    C: Fn(&[u8]) -> Vec<u8>,
{
    let path_builder = full::fullpath_builder_new(dirname);
    read_handler_new_with_checksum(path_builder, checksize, checksum)
}

fn checksum_nop(_: &[u8]) -> Vec<u8> {
    vec![]
}

/// Creates new read handler which uses a closure to build path from `Name`.
pub fn read_handler_new<B>(path_builder: B) -> impl Fn(Name) -> Event
where
    B: Fn(Name) -> PathBuf,
{
    read_handler_new_with_checksum(path_builder, 0, checksum_nop)
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

    mod read_handler_new_default_with_checksum {
        use std::path::Path;

        use crate::evt::Event;
        use crate::item::Name;
        use crate::read;

        #[test]
        fn test_empty() {
            let dirname = Path::new("./test.d/read/read_handler_new_default_with_checksum/empty.d");
            std::fs::create_dir_all(&dirname).unwrap();
            let f = read::read_handler_new_default_with_checksum(dirname, 0, read::checksum_nop);
            let evt: Event = f(Name::from("not-exist.dat"));
            assert_eq!(evt, Event::NoEntry(Name::from("not-exist.dat")));
        }
    }
}
