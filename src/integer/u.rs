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

#[cfg(test)]
mod test_u {

    mod name_checker_u8_new {
        use crate::integer::u;

        use crate::item::Name;

        #[test]
        fn test_zero() {
            let chk = u::name_checker_u8_new();
            let nam = chk(Name::from("0")).unwrap();
            assert_eq!(nam, Name::from("0"));
        }
    }
}
