// cargo run --bin foo_in_bin_dir

fn main() {
    println!("{:?}", "ok");
}

#[test]
fn test_func1() {
    assert_eq!(1 + 2, 3);
}
#[test]
fn test_func2() {
    assert_eq!(1 + 2, 3);
}
