#[test]
fn one() {
    assert_eq!(stylish::plain::format!("{}", 1), "1");
}

#[test]
fn two() {
    assert_eq!(stylish::plain::format!("{0}", 1), "1");
}

#[test]
fn three() {
    let mut x = 5;
    assert_eq!(
        stylish::plain::format!("{}{0}", {
            x += 1;
            x
        }),
        "66"
    );
}
