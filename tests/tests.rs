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
        fn fmt(&self, _: &mut stylish::Formatter<'_>) -> std::fmt::Result {
            Err(std::fmt::Error)
        }
    }

    use stylish::io::Write;
    let mut s = Vec::<u8>::new();
    let mut writer = stylish::plain::Plain::new(&mut s);
    assert!(stylish::writeln!(writer, "{:s}", Foo).is_err());
}

#[test]
fn large() {
    struct Foo([usize; 24]);

    impl stylish::Display for Foo {
        fn fmt(&self, f: &mut stylish::Formatter<'_>) -> std::fmt::Result {
            f.write_str("foo")
        }
    }

    assert_eq!(stylish::plain::format!("{:s}", Foo([0; 24])), "foo");
}

#[test]
fn blue() {
    assert_eq!(stylish::ansi::format!("{:(fg=blue)}", 2), "[34;49;22m2[0m");
    assert_eq!(
        stylish::html::format!("{:(fg=blue)}", 2),
        r#"<span style="color: blue; background-color: inherit; font-weight: inherit">2</span>"#
    );
    assert_eq!(stylish::plain::format!("{:(fg=blue)}", 2), "2");
}

#[test]
fn bold() {
    assert_eq!(stylish::ansi::format!("{:(bold)}", 2), "[39;49;1m2[0m");
    assert_eq!(
        stylish::html::format!("{:(bold)}", 2),
        r#"<span style="color: inherit; background-color: inherit; font-weight: bolder">2</span>"#
    );
    assert_eq!(stylish::plain::format!("{:(bold)}", 2), "2");
}

#[test]
fn blue_bold() {
    assert_eq!(
        stylish::ansi::format!("{:(fg=blue,bold)}", 2),
        "[34;49;1m2[0m"
    );
    assert_eq!(
        stylish::html::format!("{:(fg=blue,bold)}", 2),
        r#"<span style="color: blue; background-color: inherit; font-weight: bolder">2</span>"#
    );
    assert_eq!(stylish::plain::format!("{:(fg=blue,bold)}", 2), "2");
}

#[test]
fn bold_blue() {
    assert_eq!(
        stylish::ansi::format!("{:(bold,fg=blue)}", 2),
        "[34;49;1m2[0m"
    );
    assert_eq!(
        stylish::html::format!("{:(bold,fg=blue)}", 2),
        r#"<span style="color: blue; background-color: inherit; font-weight: bolder">2</span>"#
    );
    assert_eq!(stylish::plain::format!("{:(bold,fg=blue)}", 2), "2");
}
