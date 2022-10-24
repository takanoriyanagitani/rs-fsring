use crate::evt::Event;
use crate::item::{Name, NamedItem};

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
