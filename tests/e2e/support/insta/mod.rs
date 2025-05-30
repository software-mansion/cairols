macro_rules! test_transform {
    ($transform:expr, $before:literal, @$after:literal) => {{
        let before = ::indoc::indoc!($before);
        let description = ::std::format!(
            "// transform: {transform}\n{before}",
            transform = stringify!($transform),
            before = before.trim_end(),
        );
        let after = $transform(before);
        ::insta::with_settings!({ description => description }, {
            ::insta::assert_snapshot!(after, @$after);
        });
    }};
}

macro_rules! test_transform_plain {
    ($transform_type:ty, $code:literal, @$after:literal) => {{
        let before = ::indoc::indoc!($code);
        let description = ::std::format!(
            "// transform: {transform}\n{before}",
            transform = stringify!($transform),
            before = before.trim_end(),
        );

        let report = crate::support::transform::conduct_transformation::<$transform_type>(&before, false, crate::support::fixture::Fixture::new());
        ::insta::with_settings!({ description => description }, {
            ::insta::assert_snapshot!(report, @$after);
        });
    }};
}

macro_rules! test_transform_and_macros {
    ($transform_type:ty, $fixture:expr, $code:literal,  @$after:literal) => {{
        let code = ::indoc::indoc!($code);
        let before = ::regex::Regex::new(r"(\v)*<macro>.*</macro>\n").unwrap().replace_all(code, "");
        let before_with_macros = code.replace("<macro>", "").replace("</macro>", "");
        let description = ::std::format!(
            "// transform: {transform}\n{before}",
            transform = stringify!($transform),
            before = before_with_macros.trim_end(),
        );

        let after = crate::support::transform::conduct_transformation::<$transform_type>(&before, false, $fixture);
        let after_with_macros = crate::support::transform::conduct_transformation::<$transform_type>(&before_with_macros, true, $fixture);
        let report = after + "\n\n==============================\n\n" + &after_with_macros;
        ::insta::with_settings!({ description => description }, {
            ::insta::assert_snapshot!(report, @$after);
        });
    }};
    ($transform_type:ty, $code:literal, @$after:literal) => {
        test_transform_and_macros!($transform_type, crate::support::fixture::Fixture::new(), $code, @$after)
    };
}

pub(crate) use test_transform;
pub(crate) use test_transform_and_macros;
pub(crate) use test_transform_plain;
