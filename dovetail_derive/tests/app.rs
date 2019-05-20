use app;

#[test]
fn test_app_macro() {
    assert_eq!(4, adder::add_two(2));
}
