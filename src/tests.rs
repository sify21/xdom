mod minidom_tests;
mod sxd_tests;
mod xmltree_tests;

const POM: &'static str = include_str!("tests/pom.xml");

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
