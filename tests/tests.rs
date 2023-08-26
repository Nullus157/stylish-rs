#![allow(uncommon_codepoints)]

#[cfg(not(feature = "_tests"))]
compile_error!("please test with --all-features");

#[cfg(feature = "_tests")]
mod tests {
    #[test]
    fn one() {
        assert_eq!(stylish::plain::format!("{}", 1), "1");
    }

    #[test]
    fn let_() {
        let x = stylish::format_args!("{}", 1);
        assert_eq!(stylish::plain::format!("{:s}", x), "1");
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
    fn dyn_display() {
        let x: &dyn std::fmt::Display = &1;
        assert_eq!(stylish::plain::format!("{}", *x), "1");
    }

    #[test]
    fn error() {
        struct Foo;

        impl stylish::Display for Foo {
            fn fmt(&self, _: &mut stylish::Formatter<'_>) -> std::fmt::Result {
                Err(std::fmt::Error)
            }
        }

        let mut s = Vec::<u8>::new();
        let mut writer = stylish::io::Plain::new(&mut s);
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
        assert_eq!(
            stylish::ansi::format!("{:(fg=blue)}", 2),
            "\x1b[34m2\x1b[0m"
        );
        assert_eq!(
            stylish::html::format!("{:(fg=blue)}", 2),
            "<span style=color:blue>2</span>"
        );
        assert_eq!(stylish::plain::format!("{:(fg=blue)}", 2), "2");
    }

    #[test]
    fn bold() {
        assert_eq!(stylish::ansi::format!("{:(bold)}", 2), "\x1b[1m2\x1b[0m");
        assert_eq!(
            stylish::html::format!("{:(bold)}", 2),
            "<span style=font-weight:bolder>2</span>"
        );
        assert_eq!(stylish::plain::format!("{:(bold)}", 2), "2");
    }

    #[test]
    fn blue_bold() {
        assert_eq!(
            stylish::ansi::format!("{:(fg=blue,bold)}", 2),
            "\x1b[34;1m2\x1b[0m"
        );
        assert_eq!(
            stylish::html::format!("{:(fg=blue,bold)}", 2),
            "<span style=color:blue;font-weight:bolder>2</span>"
        );
        assert_eq!(stylish::plain::format!("{:(fg=blue,bold)}", 2), "2");
    }

    #[test]
    fn bold_blue() {
        assert_eq!(
            stylish::ansi::format!("{:(bold,fg=blue)}", 2),
            "\x1b[34;1m2\x1b[0m"
        );
        assert_eq!(
            stylish::html::format!("{:(bold,fg=blue)}", 2),
            "<span style=color:blue;font-weight:bolder>2</span>"
        );
        assert_eq!(stylish::plain::format!("{:(bold,fg=blue)}", 2), "2");
    }

    #[test]
    fn bold_then_blue() {
        assert_eq!(
            stylish::ansi::format!("plain {:(bold)} {:(fg=blue)}", "bold", "blue"),
            "plain \x1b[1mbold\x1b[0m \x1b[34mblue\x1b[0m"
        );
        assert_eq!(
            stylish::html::format!("plain {:(bold)} {:(fg=blue)}", "bold", "blue"),
            "plain <span style=font-weight:bolder>bold</span> <span style=color:blue>blue</span>"
        );
        assert_eq!(
            stylish::plain::format!("plain {:(bold)} {:(fg=blue)}", "bold", "blue"),
            "plain bold blue"
        );
    }

    #[test]
    fn bold_then_bold_blue_then_blue() {
        assert_eq!(
            stylish::ansi::format!(
                "{:(bold)}{:(bold,fg=blue)}{:(fg=blue)}",
                "bold",
                "bold-blue",
                "blue"
            ),
            "\x1b[1mbold\x1b[34mbold-blue\x1b[22mblue\x1b[0m"
        );
        assert_eq!(
            stylish::html::format!("{:(bold)}{:(bold,fg=blue)}{:(fg=blue)}", "bold", "bold-blue", "blue"),
            "<span style=font-weight:bolder>bold</span><span style=color:blue;font-weight:bolder>bold-blue</span><span style=color:blue>blue</span>"
        );
        assert_eq!(
            stylish::plain::format!(
                "{:(bold)}{:(bold,fg=blue)}{:(fg=blue)}",
                "bold",
                "bold-blue",
                "blue"
            ),
            "boldbold-blueblue"
        );
    }

    #[test]
    fn builtin_macros() {
        assert_eq!(stylish::plain::format!(concat!("a", "b")), "ab");
        assert_eq!(
            stylish::plain::format!(concat!(stringify!("a"), "b")),
            r#""a"b"#
        );
    }

    #[test]
    fn non_ascii_idents() {
        let 𓀄 = 5;
        assert_eq!(stylish::plain::format!("{𓀄}"), "5");
        assert_eq!(stylish::plain::format!("{}", 𓀄), "5");
        assert_eq!(stylish::plain::format!("{𓀄}", 𓀄 = 5), "5");
    }

    #[test]
    fn empty_string() {
        assert_eq!(
            stylish::ansi::format!("{:(fg=blue)}{:(fg=red)}{:(fg=blue)}", "foo", "", "bar"),
            "\u{1b}[34mfoobar\u{1b}[0m",
        );
        assert_eq!(
            stylish::html::format!("{:(fg=blue)}{:(fg=red)}{:(fg=blue)}", "foo", "", "bar"),
            "<span style=color:blue>foobar</span>",
        );
    }

    #[test]
    fn capture_raw_strings_and_idents() {
        let r#type = "apple";
        assert_eq!(
            stylish::plain::format!(r#"The fruit is an {type}"#),
            "The fruit is an apple"
        );

        let r#type = "orange";
        assert_eq!(
            stylish::plain::format!(r"The fruit is an {type}"),
            "The fruit is an orange"
        );

        let r#type = "mango";
        assert_eq!(
            stylish::plain::format!("The fruit is a {type}"),
            "The fruit is a mango"
        );
    }

    #[cfg(stylish_proc_macro_expand)]
    #[test]
    fn extended_builtin_macros() {
        // `file!()` is not supported by `with_builtin_macros`, but it works with
        // `proc_macro_expand`
        assert_eq!(stylish::plain::format!(file!()), file!());
    }
}
