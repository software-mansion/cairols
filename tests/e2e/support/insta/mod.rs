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

macro_rules! test_transform_inner {
    ($transform_type:ty, $fixture:expr, $before:literal, @$after:literal, $with_macros:expr) => {{
        let before = ::indoc::indoc!($before);
        let description = ::std::format!(
            "// transform: {transform}\n{before}",
            transform = stringify!($transform),
            before = before.trim_end(),
        );

        let after = crate::support::transform::conduct_transformation::<$transform_type>(&before, $with_macros, $fixture);
        ::insta::with_settings!({ description => description }, {
            ::insta::assert_snapshot!(after, @$after);
        });
    }};
}

macro_rules! test_transform_plain {
    ($transform_type:ty, $fixture:expr, $before:literal, @$after:literal) => {{
        crate::support::insta::test_transform_inner!($transform_type, $fixture, $before, @$after, false);
    }};
    ($transform_type:ty, $before:literal, @$after:literal) => {
        test_transform_plain!($transform_type, crate::support::fixture::Fixture::new(), $before, @$after)
    };
}

macro_rules! test_transform_with_macros {
    ($transform_type:ty, $fixture:expr, $before:literal, @$after:literal) => {{
        crate::support::insta::test_transform_inner!($transform_type, $fixture, $before, @$after, true);
    }};
    ($transform_type:ty, $before:literal, @$after:literal) => {
        test_transform_with_macros!($transform_type, crate::support::fixture::Fixture::new(), $before, @$after)
    };
}

pub(crate) use test_transform;
pub(crate) use test_transform_inner;
pub(crate) use test_transform_plain;
pub(crate) use test_transform_with_macros;
