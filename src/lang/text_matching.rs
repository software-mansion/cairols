use jaro_winkler::jaro_winkler;

pub fn text_matches(proposed: impl AsRef<str>, typed: impl AsRef<str>) -> bool {
    text_matches_inner(proposed.as_ref(), typed.as_ref())
}

fn text_matches_inner(proposed: &str, typed: &str) -> bool {
    if typed.is_empty() {
        return true;
    }

    jaro_winkler(proposed, typed) > 0.7
}
