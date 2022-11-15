use std::path::Path;

use crate::{FsRingBuffer, RingBuffer};

use crate::evt::Event;
use crate::request::Request;

use crate::next;
use crate::read;

/// Creates default checked random ring buffer impl which uses u8 names.
///
/// # Arguments
/// - dirname: Path to read/write buffer files.
/// - checksize: Checksum byte length.
/// - check_read:  Computes checksum.
/// - check_write:  Computes checksum(use same closure for read).
pub fn ring_buffer_impl_u8_new_default_with_checksum<P, C>(
    dirname: P,
    checksize: usize,
    check_read: C,
    check_write: C,
) -> Result<impl RingBuffer, Event>
where
    P: AsRef<Path>,
    C: Fn(&[u8]) -> Vec<u8>,
{
    let p: &Path = dirname.as_ref();

    let get = read::read_handler_new_default_with_checksum(p.to_path_buf(), checksize, check_read);
    let del = crate::del::del_handler_new_default(p.to_path_buf());
    let list = crate::list::list_request_handler_new_default(
        crate::list::u::list_names_u8_all_new(),
        p.to_path_buf(),
    );

    let get_name = next::u::next_random_u8_new_from_path_default()?;
    let push = crate::push::push_handler_new_unmanaged_default_with_checksum(
        get_name,
        p.to_path_buf(),
        check_write,
    );

    Ok(FsRingBuffer {
        get,
        del,
        push,
        list,
    })
}

fn checksum_nop(_: &[u8]) -> Vec<u8> {
    vec![]
}

/// Creates default random ring buffer impl which uses u8 names.
pub fn ring_buffer_impl_u8_new_default<P>(dirname: P) -> Result<impl RingBuffer, Event>
where
    P: AsRef<Path>,
{
    ring_buffer_impl_u8_new_default_with_checksum(dirname, 0, checksum_nop, checksum_nop)
}

/// Creates default checked random ring buffer which uses u8 names.
///
/// # Arguments
/// - dirname: Path to read/write buffer files.
/// - checksize: Checksum byte length.
/// - check_read:  Computes checksum.
/// - check_write:  Computes checksum(use same closure for read).
pub fn ring_buffer_u8_new_default_with_checksum<P, C>(
    dirname: P,
    checksize: usize,
    check_read: C,
    check_write: C,
) -> Result<impl FnMut(Request) -> Event, Event>
where
    P: AsRef<Path>,
    C: Fn(&[u8]) -> Vec<u8>,
{
    let rb =
        ring_buffer_impl_u8_new_default_with_checksum(dirname, checksize, check_read, check_write)?;
    Ok(crate::ring_buffer_new(rb))
}

/// Creates default random ring buffer which uses u8 names.
pub fn ring_buffer_u8_new_default<P>(dirname: P) -> Result<impl FnMut(Request) -> Event, Event>
where
    P: AsRef<Path>,
{
    let rb = ring_buffer_impl_u8_new_default(dirname)?;
    Ok(crate::ring_buffer_new(rb))
}

#[cfg(test)]
mod test_buf {

    mod ring_buffer_u8_new_default {

        use std::io::ErrorKind;
        use std::path::Path;

        use crate::evt::Event;
        use crate::item::{Item, Name};
        use crate::request::Request;
        use crate::u::buf;

        fn dir_clean<P>(p: P) -> Result<(), ErrorKind>
        where
            P: AsRef<Path>,
        {
            std::fs::remove_dir_all(p)
                .map(|_| ())
                .map_err(|e| e.kind())
                .or_else(|e| match e {
                    ErrorKind::NotFound => Ok(()),
                    _ => Err(e),
                })
        }

        #[test]
        #[ignore]
        fn test_push() {
            let dirname = Path::new("./test.d/u/buf/ring_buffer_u8_new_default/push.d");
            let mut f = buf::ring_buffer_u8_new_default(dirname).unwrap();
            dir_clean(&dirname).unwrap();
            std::fs::create_dir_all(&dirname).unwrap();
            let req: Request = Request::Push(Item::from(vec![]));
            let evt: Event = f(req);
            assert_eq!(evt, Event::Success);
        }

        #[test]
        #[ignore]
        fn test_list() {
            let dirname = Path::new("./test.d/u/buf/ring_buffer_u8_new_default/list.d");
            let mut f = buf::ring_buffer_u8_new_default(dirname).unwrap();
            dir_clean(&dirname).unwrap();
            std::fs::create_dir_all(&dirname).unwrap();
            let evt: Event = f(Request::Push(Item::from(vec![0x42])));
            assert_eq!(evt, Event::Success);

            let lst: Event = f(Request::List);
            let mut i = std::fs::read_dir(&dirname).unwrap();
            let dirent = i.next().unwrap().unwrap();
            let name: String = dirent.file_name().into_string().unwrap();
            assert_eq!(lst, Event::NamesGot(vec![Name::from(name)]));
        }

        #[test]
        #[ignore]
        fn test_get() {
            let dirname = Path::new("./test.d/u/buf/ring_buffer_u8_new_default/get.d");
            let mut f = buf::ring_buffer_u8_new_default(dirname).unwrap();
            dir_clean(&dirname).unwrap();
            std::fs::create_dir_all(&dirname).unwrap();
            let evt: Event = f(Request::Push(Item::from((b"299792458").as_slice())));
            assert_eq!(evt, Event::Success);

            let lst: Event = f(Request::List);
            let names: Vec<Name> = lst.try_into().unwrap();
            let n: Name = names.into_iter().next().unwrap();

            let got: Event = f(Request::Get(n));
            let i: Item = got.try_into().unwrap();
            assert_eq!(i, Item::from((b"299792458").as_slice()));
        }

        #[test]
        #[ignore]
        fn test_del() {
            let dirname = Path::new("./test.d/u/buf/ring_buffer_u8_new_default/del.d");
            let mut f = buf::ring_buffer_u8_new_default(dirname).unwrap();
            dir_clean(&dirname).unwrap();
            std::fs::create_dir_all(&dirname).unwrap();
            let evt: Event = f(Request::Push(Item::from((b"299792458").as_slice())));
            assert_eq!(evt, Event::Success);

            let lst: Event = f(Request::List);
            let names: Vec<Name> = lst.try_into().unwrap();
            let n: Name = names.into_iter().next().unwrap();

            let got: Event = f(Request::Get(n));
            let tgt: Name = got.try_into().unwrap();

            let rmv: Event = f(Request::Del(tgt));
            assert_eq!(rmv, Event::Success);
        }
    }

    mod ring_buffer_u8_new_default_with_checksum {

        use std::fs::File;
        use std::io::ErrorKind;
        use std::path::Path;

        use crate::evt::Event;
        use crate::item::{Item, Name};
        use crate::request::Request;
        use crate::u::buf;

        fn dir_clean<P>(p: P) -> Result<(), ErrorKind>
        where
            P: AsRef<Path>,
        {
            std::fs::remove_dir_all(p)
                .map(|_| ())
                .map_err(|e| e.kind())
                .or_else(|e| match e {
                    ErrorKind::NotFound => Ok(()),
                    _ => Err(e),
                })
        }

        #[test]
        #[ignore]
        fn test_push() {
            let dirname =
                Path::new("./test.d/u/buf/ring_buffer_u8_new_default_with_checksum/push.d");
            let chk = |_: &[u8]| vec![];

            let mut f =
                buf::ring_buffer_u8_new_default_with_checksum(dirname, 0, chk, chk).unwrap();
            dir_clean(&dirname).unwrap();
            std::fs::create_dir_all(&dirname).unwrap();
            let req: Request = Request::Push(Item::from(vec![]));
            let evt: Event = f(req);
            assert_eq!(evt, Event::Success);
        }

        #[test]
        #[ignore]
        fn test_valid() {
            let dirname =
                Path::new("./test.d/u/buf/ring_buffer_u8_new_default_with_checksum/valid.d");
            let chk = |_: &[u8]| vec![];

            let mut f =
                buf::ring_buffer_u8_new_default_with_checksum(dirname, 0, chk, chk).unwrap();
            dir_clean(&dirname).unwrap();
            std::fs::create_dir_all(&dirname).unwrap();
            let name = dirname.join("42");
            File::create(name).unwrap();
            let req: Request = Request::Get(Name::from("42"));
            let evt: Event = f(req);
            match evt {
                Event::ItemGot(_) => {}
                _ => {
                    panic!("Unexpected event: {:#?}", evt);
                }
            }
        }

        #[test]
        #[ignore]
        fn test_checksum_invalid() {
            let dirname = Path::new(
                "./test.d/u/buf/ring_buffer_u8_new_default_with_checksum/checksum_invalid.d",
            );
            let chk = |_: &[u8]| {
                (0xcafef00d_dead_beaf_face_864299792458_u128)
                    .to_be_bytes()
                    .into()
            };

            let mut f =
                buf::ring_buffer_u8_new_default_with_checksum(dirname, 0, chk, chk).unwrap();
            dir_clean(&dirname).unwrap();
            std::fs::create_dir_all(&dirname).unwrap();
            let nm = Name::from("42");
            let name = dirname.join(nm.as_str());
            File::create(name).unwrap();
            let req: Request = Request::Get(nm.clone());
            let evt: Event = f(req);
            match evt {
                Event::Broken(broken_filename) => {
                    assert_eq!(Name::from(nm), broken_filename);
                }
                _ => {
                    panic!("Unexpected event: {:#?}", evt);
                }
            }
        }

        #[test]
        #[ignore]
        fn test_checksum_valid() {
            let dirname = Path::new(
                "./test.d/u/buf/ring_buffer_u8_new_default_with_checksum/checksum_valid.d",
            );
            let chk = |_: &[u8]| (b"cafef00ddeadbeafface864299792458".to_vec());

            let mut f =
                buf::ring_buffer_u8_new_default_with_checksum(dirname, 32, chk, chk).unwrap();
            dir_clean(&dirname).unwrap();
            std::fs::create_dir_all(&dirname).unwrap();
            let nm = Name::from("42");
            let name = dirname.join(nm.as_str());
            std::fs::write(name, b"FFcafef00ddeadbeafface864299792458").unwrap();
            let req: Request = Request::Get(nm.clone());
            let evt: Event = f(req);
            match evt {
                Event::ItemGot(named) => {
                    let (name, item) = named.into_pair();
                    assert_eq!(item, Item::from(Vec::from("FF")));
                    assert_eq!(name, Name::from("42"));
                }
                _ => {
                    panic!("Unexpected event: {:#?}", evt);
                }
            }
        }
    }
}
