use crate::compose::compose_err;

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

pub fn next_gen_by_prev_u8_new<F, G>(n2u: F, u2n: G) -> impl Fn(Name) -> Result<Name, Event>
where
    F: Fn(Name) -> Result<u8, Event>,
    G: Fn(u8) -> Result<Name, Event>,
{
    let next = move |prev: Name| {
        let pu: u8 = n2u(prev)?;
        Ok(pu + 1)
    };
    compose_err(next, u2n)
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

    mod next_gen_by_prev_u8_new {
        use crate::integer::u;
        use crate::item::Name;

        #[test]
        fn test_valid_name() {
            let n2u = |_: Name| Ok(0x41);
            let u2n = |_: u8| Ok(Name::from("42"));
            let f = u::next_gen_by_prev_u8_new(n2u, u2n);
            let n: Name = f(Name::from("41")).unwrap();
            assert_eq!(n, Name::from("42"));
        }
    }
}
