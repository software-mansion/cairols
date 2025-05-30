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

#[expect(unused_macros)]
macro_rules! test_transform_and_macros {
    ($transform:expr, $code:literal, @$after:literal) => {{
        let code = ::indoc::indoc!($code);
        let before = ::regex::Regex::new(r"(\v)*<macro>.*</macro>\n").unwrap().replace_all(code, "");
        let before_with_macros = code.replace("<macro>", "").replace("</macro>", "");
        let description = ::std::format!(
            "// transform: {transform}\n{before}",
            transform = stringify!($transform),
            before = before_with_macros.trim_end(),
        );

        let after = $transform(&before, false);
        let after_macros = $transform(&before_with_macros, true);
        let report = after + "\n\n==============================\n\n" + &after_macros;
        ::insta::with_settings!({ description => description }, {
            ::insta::assert_snapshot!(report, @$after);
        });
    }};
}

pub(crate) use test_transform;
#[expect(unused_imports)]
pub(crate) use test_transform_and_macros;
