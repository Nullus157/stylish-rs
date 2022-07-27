#[cfg(not(feature = "_tests"))]
compile_error!("please test with --all-features");

#[cfg(feature = "_tests")]
mod tests {
    #[test]
    fn one() {
        assert_eq!(stylish_plain::format!("{}", 1), "1");
    }

    #[test]
    fn two() {
        assert_eq!(stylish_plain::format!("{0}", 1), "1");
    }

    #[test]
    fn three() {
        let mut x = 5;
        assert_eq!(
            stylish_plain::format!("{}{0}", {
                x += 1;
                x
            }),
            "66"
        );
    }

    #[test]
    fn four() {
        assert_eq!(stylish_plain::format!("{x}", x = 1), "1")
    }

    #[test]
    fn five() {
        let x = 1;
        assert_eq!(stylish_plain::format!("{x}"), "1")
    }

    #[test]
    fn six() {
        #[allow(unused_variables)]
        let x = 1;
        assert_eq!(stylish_plain::format!("{x}", x = 2), "2")
    }

    #[test]
    fn large() {
        struct Foo([usize; 24]);

        impl stylish_core::Display for Foo {
            fn fmt(&self, f: &mut stylish_core::Formatter<'_>) -> stylish_core::Result {
                f.write_str("foo")
            }
        }

        assert_eq!(stylish_plain::format!("{:s}", Foo([0; 24])), "foo");
    }
}
