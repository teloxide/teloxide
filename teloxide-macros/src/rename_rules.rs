pub fn rename_by_rule(input: &str, rule: &str) -> String {
    match rule {
        "lowercase" => input.to_string().to_lowercase(),
        _ => rule.to_string(),
    }
}
