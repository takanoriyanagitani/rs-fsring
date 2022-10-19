use crate::evt::Event;
use crate::item::Name;

fn get_next_or_retry_u8<N, E>(prev: Name, get_next: N, is_empty: E, cnt: u8) -> Result<Name, Event>
where
    N: Fn(Name) -> Result<Name, Event>,
    E: Fn(&Name) -> Result<bool, Event>,
{
    match cnt < 255 {
        false => Err(Event::TooManyItemsAlready),
        true => {
            let nex: Name = get_next(prev)?;
            match is_empty(&nex) {
                Ok(true) => Ok(nex),
                Ok(false) => get_next_or_retry_u8(nex, get_next, is_empty, 1 + cnt),
                Err(e) => Err(e),
            }
        }
    }
}

pub fn get_next_retry_u8<N, E>(prev: Name, get_next: N, is_empty: E) -> Result<Name, Event>
where
    N: Fn(Name) -> Result<Name, Event>,
    E: Fn(&Name) -> Result<bool, Event>,
{
    get_next_or_retry_u8(prev, get_next, is_empty, 0)
}

#[cfg(test)]
mod test_next {

    mod get_next_or_retry_u8 {
        use crate::evt::Event;
        use crate::item::Name;
        use crate::next;

        #[test]
        fn test_no_retry() {
            let n: Name = next::get_next_retry_u8(
                Name::from("00"),
                |_prev: Name| Ok(Name::from("01")),
                |_noent: &Name| Ok(true),
            )
            .unwrap();
            assert_eq!(n, Name::from("01"));
        }

        #[test]
        fn test_toomany() {
            let r = next::get_next_retry_u8(
                Name::from("00"),
                |_prev: Name| Ok(Name::from("01")),
                |_noent: &Name| Ok(false),
            );
            assert_eq!(r, Err(Event::TooManyItemsAlready));
        }

        #[test]
        fn test_unexpected_name() {
            let r = next::get_next_retry_u8(
                Name::from("00"),
                |_prev: Name| Err(Event::UnexpectedError("".into())),
                |_noent: &Name| Ok(false),
            );
            assert_eq!(r, Err(Event::UnexpectedError("".into())));
        }

        #[test]
        fn test_unexpected_empty() {
            let r = next::get_next_retry_u8(
                Name::from("00"),
                |_prev: Name| Ok(Name::from("01")),
                |_noent: &Name| Err(Event::UnexpectedError("".into())),
            );
            assert_eq!(r, Err(Event::UnexpectedError("".into())));
        }
    }
}
