use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::evt::Event;
use crate::item::Name;

pub fn next_random_u8_new<R>(mut random_source: R) -> impl FnMut() -> Result<Name, Event>
where
    R: FnMut() -> Result<u8, Event>,
{
    move || random_source().map(Name::from)
}

pub fn next_random_u8_new_from_read<R>(mut r: R) -> impl FnMut() -> Result<Name, Event>
where
    R: Read,
{
    let mut buf: [u8; 1] = [0; 1];
    let rs = move || {
        r.read_exact(&mut buf)
            .map(|_| buf[0])
            .map_err(|e| Event::UnexpectedError(format!("Unable to read next u8 from read: {}", e)))
    };
    next_random_u8_new(rs)
}

pub fn next_random_u8_new_from_path<P>(p: P) -> Result<impl FnMut() -> Result<Name, Event>, Event>
where
    P: AsRef<Path>,
{
    let f: File = File::open(p).map_err(|e| {
        Event::UnexpectedError(format!("Unable to open random u8 source file: {}", e))
    })?;
    Ok(next_random_u8_new_from_read(f))
}

pub fn next_random_u8_new_from_path_default() -> Result<impl FnMut() -> Result<Name, Event>, Event>
{
    next_random_u8_new_from_path("/dev/urandom")
}

#[cfg(test)]
mod test_u {

    mod next_random_u8_new_from_read {

        use crate::item::Name;
        use crate::next;

        #[test]
        fn test_number() {
            let rs: &[u8] = &[0x29, 0x97, 0x92, 0x45, 0x80];
            let mut f = next::u::next_random_u8_new_from_read(rs);
            assert_eq!(f(), Ok(Name::from("29")));
            assert_eq!(f(), Ok(Name::from("97")));
            assert_eq!(f(), Ok(Name::from("92")));
            assert_eq!(f(), Ok(Name::from("45")));
            assert_eq!(f(), Ok(Name::from("80")));
            assert_eq!(f().is_err(), true);
        }
    }

    mod next_random_u8_new_from_path_default {
        use crate::item::Name;
        use crate::next;

        #[test]
        #[ignore]
        fn test_single() {
            let mut f = next::u::next_random_u8_new_from_path_default().unwrap();
            let n: Name = f().unwrap();
            let s: String = n.into();
            let r = u8::from_str_radix(s.as_str(), 16);
            assert_eq!(r.is_ok(), true);
        }
    }
}
