#[test]
fn show_last_name() {
    // Don't
    let user_name = "tomoki hamada";
    let mut splits = user_name.split_whitespace();
    let last = splits.nth(1).unwrap();
    assert_eq!(last, "hamada");

    // Do
    let user_name = UserName {
        first: "tomoki".to_owned(),
        last: "hamada".to_owned(),
    };
    assert_eq!(user_name.last, "hamada");
}

#[allow(dead_code)]
struct UserName {
    first: String,
    last: String,
}
