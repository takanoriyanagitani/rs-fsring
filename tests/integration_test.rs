#[cfg(test)]
mod del {

    mod delete_handler_new_checked {

        use std::fs::File;
        use std::path::Path;

        use rs_fsring::del;
        use rs_fsring::evt::Event;
        use rs_fsring::item::Name;

        #[test]
        #[ignore]
        fn test_noent() {
            let tp = Path::new("./test.d/del/delete_handler_new_checked/test_noent");
            std::fs::create_dir_all(tp).unwrap();

            let delete_handler = del::delete_handler_new_checked(|name: Name| Ok(name), tp);
            let evt = delete_handler(Name::from("not-exist.dat"));
            assert_eq!(evt, Event::Success);
        }

        #[test]
        #[ignore]
        fn test_exist() {
            let tp = Path::new("./test.d/del/delete_handler_new_checked/test_exist");
            std::fs::create_dir_all(tp).unwrap();

            let delete_handler = del::delete_handler_new_checked(|name: Name| Ok(name), tp);
            let name: Name = Name::from("empty.dat");
            let empty = tp.join(name.as_str());
            File::create(&empty).unwrap();
            assert_eq!(empty.exists(), true);

            let evt = delete_handler(name);
            assert_eq!(evt, Event::Success);
            assert_eq!(empty.exists(), false);
        }
    }
}
