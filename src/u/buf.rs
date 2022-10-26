use std::path::Path;

use crate::{FsRingBuffer, RingBuffer};

use crate::evt::Event;
use crate::request::Request;

use crate::next;
use crate::read;

/// Creates default random ring buffer impl which uses u8 names.
pub fn ring_buffer_impl_u8_new_default<P>(dirname: P) -> Result<impl RingBuffer, Event>
where
    P: AsRef<Path>,
{
    let p: &Path = dirname.as_ref();

    let get = read::read_handler_new_default(p.to_path_buf());
    let del = crate::del::del_handler_new_default(p.to_path_buf());
    let list = crate::list::list_request_handler_new_default(
        crate::list::u::list_names_u8_all_new(),
        p.to_path_buf(),
    );

    let get_name = next::u::next_random_u8_new_from_path_default()?;
    let push = crate::push::push_handler_new_unmanaged_default(get_name, p.to_path_buf());

    Ok(FsRingBuffer {
        get,
        del,
        push,
        list,
    })
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
}
