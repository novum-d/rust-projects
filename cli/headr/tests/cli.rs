use headr::parse_positive_Int;

#[test]
fn test_parse_positive_int() {
   let res = parse_positive_Int("42");
    assert!(res.is_ok());
}