mod fixture;
pub(crate) use fixture::test_file;

#[allow(non_snake_case)]
mod gen;
#[allow(non_snake_case)]
mod manual;

macro_rules! fixture_file {
    ($dir:literal, $fixture:ident) => {
        #[test]
        fn $fixture() {
            let test_name = concat!("(test ", $dir, "/", stringify!($fixture), ")");
            let src = include_str!(concat!("../../../", $dir, "/", stringify!($fixture)));
            test_file(test_name, src);
        }
    };
}
pub(crate) use fixture_file;
