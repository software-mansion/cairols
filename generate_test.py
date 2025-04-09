from textwrap import dedent, indent
from typing import Literal
from itertools import chain
from pathlib import Path


def test(
    type_expression: str,
    type_type: Literal["extern_type", "builtin_alias"],
    context: Literal[
        "alias",
        "variable",
        "function_argument",
        "return_type",
        "struct_field",
        "turbofish_enum",
        # "turbofish_function",  # Ask the team if this is necessary.
        # "turbofish_trait",
        "trait_associated_type",
        "trait_associated_const",
        "trait_generic_bound",
        "impl_associated_type",
        "impl_associated_const",
        "impl_generic_bound",
    ],
    role: Literal["type", "type_parameter"],
    top_level_import: str | None = None,
) -> str:
    cairo_code_segments: list[str] = (
        [] if top_level_import is None else [top_level_import]
    )

    type_with_caret = (
        (type_expression + "<caret>")
        if role == "type"
        else f"Array<{type_expression}<caret>>"
    )

    match context:
        case "alias":
            cairo_code_segments.append(f"type TypeAlias = {type_with_caret};")

        case "variable":
            cairo_code_segments.append(
                dedent(f"""
                    fn foo() {{
                        let x: {type_with_caret} = 0x0;
                    }}
                """)
            )

        case "function_argument":
            cairo_code_segments.append(
                dedent(f"""
                    fn foo(x: {type_with_caret}) {{}}
                """)
            )

        case "return_type":
            cairo_code_segments.append(
                dedent(f"""
                    fn foo() -> {type_with_caret} {{ 0x0 }}
                """)
            )

        case "struct_field":
            cairo_code_segments.append(
                dedent(f"""
                    struct Struct {{
                        x: {type_with_caret}
                    }}
                """)
            )

        case "turbofish_enum":
            cairo_code_segments.append(
                dedent(f"""
                    fn foo() {{
                        let x = Result::<{type_with_caret}>::Err(());
                    }}
                """)
            )

        case "trait_associated_type":
            cairo_code_segments.append(
                dedent(f"""
                    trait Trait {{
                        type Type = {type_with_caret};
                    }}
                """)
            )

        case "trait_associated_const":
            cairo_code_segments.append(
                dedent(f"""
                trait Trait {{
                    const Const: {type_with_caret};
                }}
            """)
            )

        case "trait_generic_bound":
            cairo_code_segments.append(
                dedent(f"""
                    trait Trait<T, +Into<{type_with_caret}, T>> {{}}
                """)
            )

        case "impl_associated_type":
            cairo_code_segments.append(
                dedent(f"""
                    trait Trait {{
                        type Type;
                    }}

                    impl Impl of Trait {{
                        type Type = {type_with_caret};
                    }}
                """)
            )

        case "impl_associated_const":
            cairo_code_segments.append(
                dedent(f"""
                    trait Trait {{
                        const Const: {type_expression};
                    }}

                    impl Impl of Trait {{
                        const Const: {type_with_caret} = 0x0;
                    }}
                """)
            )

        case "impl_generic_bound":
            cairo_code_segments.append(
                dedent(f"""
                    trait Trait<T, +Into<{type_expression}, T>> {{}}
                    impl<T, +Into<{type_with_caret}, T>> Impl of Trait<T> {{}}
                """)
            )

    cairo_code = dedent("\n".join(cairo_code_segments))

    rust_code = dedent(f"""
        #[test]
        fn test_{type_type}_in_{context}_as_{role}() {{
            test_transform!(
                transform,
                r#"
                    {indent(cairo_code, 4 * "\t")}
                "#,
                @r#"
                "#
            )
        }}
    """)

    return rust_code


def generate_cases() -> str:
    contexts = [
        "alias",
        "variable",
        "function_argument",
        "return_type",
        "struct_field",
        "turbofish_enum",
        "trait_associated_type",
        "trait_associated_const",
        "trait_generic_bound",
        "impl_associated_type",
        "impl_associated_const",
        "impl_generic_bound",
    ]
    roles = ["type", "type_parameter"]

    u32 = "u32"

    u96 = "u96"
    u96_import = "use core::circuit::u96;"

    u32_cases = [
        test(u32, "extern_type", context, role)
        for context in contexts
        for role in roles
    ]
    u96_cases = [
        test(u96, "builtin_alias", context, role, u96_import)
        for context in contexts
        for role in roles
    ]

    whole_code = "\n".join(chain(u32_cases, u96_cases))

    return whole_code


if __name__ == "__main__":
    file = Path(__file__).parent / "types.rs"
    file.write_text(generate_cases())
