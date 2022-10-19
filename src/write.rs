use crate::evt::Event;
use crate::item::{Name, NamedItem};

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
