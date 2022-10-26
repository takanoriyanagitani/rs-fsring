use crate::evt::Event;
use crate::item::Name;

/// Creates new trivial list getter.
pub fn list_names_u8_all_new() -> impl Fn() -> Result<Vec<Name>, Event> {
    move || {
        let i = 0..256;
        let names = i.map(|num| num as u8).map(Name::from);
        Ok(names.collect())
    }
}

#[cfg(test)]
mod test_u {

    mod list_names_u8_all_new {
        use crate::list;

        #[test]
        fn test_len() {
            let f = list::u::list_names_u8_all_new();
            let v: Vec<_> = f().unwrap();
            assert_eq!(v.len(), 256);
        }
    }
}
