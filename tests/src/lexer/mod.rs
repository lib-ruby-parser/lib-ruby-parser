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
            let fixture_path = format!("{}/{}", $dir, stringify!($fixture));
            test_file(&fixture_path);
        }
    };
}
pub(crate) use fixture_file;
