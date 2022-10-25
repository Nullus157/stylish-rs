use stylish::plain::format;

// mod builders;
mod float;
mod num;

#[test]
fn test_format_flags() {
    // No residual flags left by pointer formatting
    let p = "".as_ptr();
    assert_eq!(format!("{:p} {:x}", p, 16), format!("{:p} 10", p));

    assert_eq!(format!("{: >3}", 'a'), "  a");
}

#[test]
fn test_pointer_formats_data_pointer() {
    let b: &[u8] = b"";
    let s: &str = "";
    assert_eq!(format!("{:p}", s), std::format!("{s:p}"));
    assert_eq!(format!("{:p}", b), std::format!("{b:p}"));
    assert_eq!(format!("{:p}", s), format!("{:p}", s.as_ptr()));
    assert_eq!(format!("{:p}", b), format!("{:p}", b.as_ptr()));
}

#[test]
fn pad_integral_resets() {
    struct Bar;

    impl core::fmt::Display for Bar {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            "1".fmt(f)?;
            f.pad_integral(true, "", "5")?;
            "1".fmt(f)
        }
    }

    assert_eq!(format!("{:<03}", Bar), "1  0051  ");
}
