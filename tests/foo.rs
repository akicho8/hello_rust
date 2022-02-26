// cargo test
// cargo test --test foo
// C-c C-c C-f

extern crate hello_rust;

mod common;

#[test]
fn tests_dir_test1() {
    common::setup();
    hello_rust::lib_func1();
    assert_eq!(1 + 2, 3);
}
