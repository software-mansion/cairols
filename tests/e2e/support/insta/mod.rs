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
    ($transform_type:ty, $fixture:expr, $before:literal, @$after:literal, $with_macros:expr, $config:expr) => {{
        let before = ::indoc::indoc!($before);
        let description = ::std::format!(
            "// transform: {transform}\n{before}",
            transform = stringify!($transform),
            before = before.trim_end(),
        );

        let after = crate::support::transform::conduct_transformation::<$transform_type>(&before, $with_macros, $fixture, $config);
        ::insta::with_settings!({ description => description }, {
            ::insta::assert_snapshot!(after, @$after);
        });
    }};
}

macro_rules! define_transform_macro {
    ($name:ident, $with_macros:literal) => {
        macro_rules! $name {
            ($transform_type:ty, $fixture:expr, $before:literal, @$after:literal, $config:expr) => {
                crate::support::insta::test_transform_inner!($transform_type, $fixture, $before, @$after, $with_macros, $config)
            };
            ($transform_type:ty, $fixture:expr, $before:literal, @$after:literal) => {
                $name!($transform_type, $fixture, $before, @$after, None)
            };
            ($transform_type:ty, $before:literal, @$after:literal, $config:expr) => {
                $name!($transform_type, crate::support::fixture::Fixture::new(), $before, @$after, $config)
            };
            ($transform_type:ty, $before:literal, @$after:literal) => {
                $name!($transform_type, crate::support::fixture::Fixture::new(), $before, @$after, None)
            };
        }
    };
}

define_transform_macro!(test_transform_plain, false);
define_transform_macro!(test_transform_with_macros, true);

pub(crate) use test_transform;
pub(crate) use test_transform_inner;
pub(crate) use test_transform_plain;
pub(crate) use test_transform_with_macros;
