pub mod compose;
pub mod del;
pub mod empty;
pub mod err;
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

pub trait RingBuffer {
    fn handle(&mut self, req: Request) -> Event;
}

pub fn ring_buffer_new<R>(mut r: R) -> impl FnMut(Request) -> Event
where
    R: RingBuffer,
{
    move |req: Request| r.handle(req)
}

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
        }
    }
}
