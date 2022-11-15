pub mod compose;
pub mod del;
pub mod empty;
pub mod evt;
pub mod full;
pub mod integer;
pub mod item;
pub mod list;
pub mod next;
pub mod push;
pub mod read;
pub mod request;
pub mod u;
pub mod write;

use crate::evt::Event;
use crate::request::Request;

use crate::item::{Item, Name};

fn remove_item<R>(buf: &mut R, n: Name) -> Result<u64, Event>
where
    R: RingBuffer,
{
    let q: Request = Request::Del(n);
    match buf.handle(q) {
        Event::Success => Ok(1),
        e => Err(e),
    }
}

fn remove_broken_item<R>(buf: &mut R, n: Name) -> Result<u64, Event>
where
    R: RingBuffer,
{
    let q: Request = Request::Get(n);
    match buf.handle(q) {
        Event::Broken(broken_name) => remove_item(buf, broken_name),
        _ => Ok(0),
    }
}

fn remove_broken_items<R>(buf: &mut R, names: Vec<Name>) -> Result<u64, Event>
where
    R: RingBuffer,
{
    names.into_iter().try_fold(0, |tot, name| {
        remove_broken_item(buf, name).map(|cnt: u64| cnt + tot)
    })
}

fn remove_broken_items_from_list<R>(buf: &mut R) -> Result<u64, Event>
where
    R: RingBuffer,
{
    let q: Request = Request::List;
    match buf.handle(q) {
        Event::NamesGot(names) => remove_broken_items(buf, names),
        e => Err(e),
    }
}

/// Removes broken buffer items.
///
/// The buffer with checksum function required.
pub fn remove_broken_buffers<R>(buf: &mut R) -> Event
where
    R: RingBuffer,
{
    remove_broken_items_from_list(buf)
        .map(Event::BrokenItemsRemoved)
        .unwrap_or_else(|e| {
            Event::UnexpectedError(format!("Unable to remove broken items: {:#?}", e))
        })
}

/// An interface for creating request handler.
pub trait RingBuffer {
    fn handle(&mut self, req: Request) -> Event;
}

/// Creates new request handler.
pub fn ring_buffer_new<R>(mut r: R) -> impl FnMut(Request) -> Event
where
    R: RingBuffer,
{
    move |req: Request| r.handle(req)
}

/// Helper struct for creating request handler.
pub struct FsRingBuffer<G, D, P, L> {
    pub get: G,
    pub del: D,
    pub push: P,
    pub list: L,
}

impl<G, D, P, L> FsRingBuffer<G, D, P, L>
where
    G: Fn(Name) -> Event,
    D: Fn(Name) -> Event,
    P: FnMut(Item) -> Event,
    L: Fn() -> Event,
{
    fn handle_get(&mut self, name: Name) -> Event {
        (self.get)(name)
    }

    fn handle_del(&mut self, name: Name) -> Event {
        (self.del)(name)
    }

    fn handle_push(&mut self, item: Item) -> Event {
        (self.push)(item)
    }

    fn handle_list(&mut self) -> Event {
        (self.list)()
    }
}

impl<G, D, P, L> RingBuffer for FsRingBuffer<G, D, P, L>
where
    G: Fn(Name) -> Event,
    D: Fn(Name) -> Event,
    P: FnMut(Item) -> Event,
    L: Fn() -> Event,
{
    fn handle(&mut self, req: Request) -> Event {
        match req {
            Request::Get(name) => self.handle_get(name),
            Request::Del(name) => self.handle_del(name),
            Request::Push(item) => self.handle_push(item),
            Request::List => self.handle_list(),
            Request::Vacuum => remove_broken_buffers(self),
        }
    }
}

#[cfg(test)]
mod test_lib {

    mod remove_broken_buffers {
        use std::fs;
        use std::path::Path;

        use crate::evt::Event;
        use crate::request::Request;
        use crate::u::buf::ring_buffer_u8_new_default;
        use crate::u::buf::ring_buffer_u8_new_default_with_checksum;

        #[test]
        #[ignore]
        fn test_without_checksum() {
            let dirname = Path::new("./test.d/lib/remove_broken_buffers/test_without_checksum.d");
            fs::remove_dir_all(&dirname).ok();
            fs::create_dir_all(&dirname).unwrap();

            let mut handler = ring_buffer_u8_new_default(&dirname).unwrap();
            let filename = dirname.join("42");
            fs::write(filename, b"cafef00ddeadbeafface864299792458").unwrap();
            let q: Request = Request::Vacuum;
            match handler(q) {
                Event::BrokenItemsRemoved(cnt) => {
                    assert_eq!(cnt, 0);
                }
                e => {
                    panic!("Unexpected event: {:#?}", e)
                }
            }
        }

        #[test]
        #[ignore]
        fn test_invalid() {
            let dirname = Path::new("./test.d/lib/remove_broken_buffers/test_invalid.d");
            fs::remove_dir_all(&dirname).ok();
            fs::create_dir_all(&dirname).unwrap();

            let chk = |_: &[u8]| b"cafef00ddeadbeafface864299792458".to_vec();

            let mut handler =
                ring_buffer_u8_new_default_with_checksum(&dirname, 7, chk, chk).unwrap();
            let filename = dirname.join("42");
            fs::write(filename, b"cafef00ddeadbeafface864299792458").unwrap();
            let q: Request = Request::Vacuum;
            match handler(q) {
                Event::BrokenItemsRemoved(cnt) => {
                    assert_eq!(cnt, 1);
                }
                e => {
                    panic!("Unexpected event: {:#?}", e)
                }
            }
        }
    }
}
