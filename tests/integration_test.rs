#[cfg(test)]
mod del {

    mod truncate_fs_new {
        use std::path::Path;

        use rs_fsring::del;
        use rs_fsring::evt::Event;
        use rs_fsring::item::Name;

        #[test]
        #[ignore]
        fn test_noent() {
            let dir = Path::new("./test.d/del/truncate_fs_new/test_noent");
            std::fs::create_dir_all(dir).unwrap();

            let path_builder = |_: &Name| dir.join("not-exist.dat");
            let f = del::truncate_fs_new(path_builder);
            let evt = f(Name::from("not-exist.dat"));
            assert_eq!(evt, Event::Empty(Name::from("not-exist.dat")));
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
