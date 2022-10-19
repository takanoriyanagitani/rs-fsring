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

#[cfg(test)]
mod empty {

    mod name2empty_fs_new {

        use std::fs::File;
        use std::path::Path;

        use rs_fsring::empty;
        use rs_fsring::item::Name;

        #[test]
        #[ignore]
        fn test_noent() {
            let tp = Path::new("./test.d/empty/name2empty_fs_new/test_noent");
            std::fs::create_dir_all(tp).unwrap();

            let f = empty::name2empty_fs_new(|_name: Name| tp.join("noent.dat"));

            let is_empty: bool = f(Name::from("")).unwrap();
            assert_eq!(is_empty, true);
        }

        #[test]
        #[ignore]
        fn test_zero() {
            let tp = Path::new("./test.d/empty/name2empty_fs_new/test_zero");
            std::fs::create_dir_all(tp).unwrap();

            let p = tp.join("empty.dat");
            File::create(&p).unwrap();

            let f = empty::name2empty_fs_new(|_name: Name| p.clone());

            let is_empty: bool = f(Name::from("")).unwrap();
            assert_eq!(is_empty, true);
        }

        #[test]
        #[ignore]
        fn test_non_empty() {
            let tp = Path::new("./test.d/empty/name2empty_fs_new/test_non_empty");
            std::fs::create_dir_all(tp).unwrap();

            let p = tp.join("nonempty.dat");
            std::fs::write(&p, b"hw").unwrap();

            let f = empty::name2empty_fs_new(|_name: Name| p.clone());

            let is_empty: bool = f(Name::from("")).unwrap();
            assert_eq!(is_empty, false);
        }
    }
}
