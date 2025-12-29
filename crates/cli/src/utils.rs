pub fn is_valid_name(s: &str) -> bool {
    let first = s.chars().next();
    first.is_some_and(|c| c.is_alphabetic() || c == '_' || c == '-')
        && s.chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        && s != "_"
        && s != "-"
        && ![
            "mod", "fn", "pub", "use", "struct", "enum", "impl", "self", "super", "crate",
        ]
        .contains(&s)
}
