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

#[test]
fn four() {
    assert_eq!(stylish::plain::format!("{x}", x = 1), "1")
}

#[test]
fn five() {
    let x = 1;
    assert_eq!(stylish::plain::format!("{x}"), "1")
}

#[test]
fn six() {
    #[allow(unused_variables)]
    let x = 1;
    assert_eq!(stylish::plain::format!("{x}", x = 2), "2")
}

#[test]
fn error() {
    struct Foo;

    impl stylish::Display for Foo {
        fn fmt(&self, f: &mut stylish::Formatter<'_>) -> std::fmt::Result {
            Err(std::fmt::Error)
        }
    }

    use stylish::io::Write;
    let mut s = Vec::<u8>::new();
    let mut writer = stylish::plain::Write::new(&mut s);
    assert!(stylish::writeln!(writer, "{}", Foo).is_err());
}
