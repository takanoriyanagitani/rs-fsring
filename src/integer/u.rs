use crate::evt::Event;

use crate::item::Name;

pub fn name_checker_u8_new() -> impl Fn(Name) -> Result<Name, Event> {
    move |name: Name| {
        let s: &str = name.as_str();
        str::parse(s)
            .map_err(|_| Event::BadRequest)
            .map(|_: u8| name)
    }
}
