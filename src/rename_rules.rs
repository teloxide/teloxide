// Some concepts are from Serde.

use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase,
    ToShoutySnakeCase, ToSnakeCase,
};

/// Apply a renaming rule to an enum variant,
/// returning the version expected in the source.
///
/// The possible `rule` can be: `lowercase`, `UPPERCASE`, `PascalCase`,
/// `camelCase`, `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`,
/// `SCREAMING-KEBAB-CASE`. See tests for the details how it will work.
pub(crate) fn rename_by_rule(input: &str, rule: &str) -> String {
    match rule {
        "lowercase" => input.to_lowercase(),
        "UPPERCASE" => input.to_uppercase(),
        "PascalCase" => input.to_pascal_case(),
        "camelCase" => input.to_lower_camel_case(),
        "snake_case" => input.to_snake_case(),
        "SCREAMING_SNAKE_CASE" => input.to_shouty_snake_case(),
        "kebab-case" => input.to_kebab_case(),
        "SCREAMING-KEBAB-CASE" => input.to_shouty_kebab_case(),
        _ => rule.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($lval:expr, $rval:expr) => {
            assert_eq!(rename_by_rule($lval, TYPE), $rval);
        };
    }

    #[test]
    fn test_lowercase() {
        const TYPE: &str = "lowercase";

        test_eq!("HelloWorld", "helloworld");
        test_eq!("Hello_World", "hello_world");
        test_eq!("Hello-World", "hello-world");
        test_eq!("helloWorld", "helloworld");
    }

    #[test]
    fn test_uppercase() {
        const TYPE: &str = "UPPERCASE";

        test_eq!("HelloWorld", "HELLOWORLD");
        test_eq!("Hello_World", "HELLO_WORLD");
        test_eq!("Hello-World", "HELLO-WORLD");
        test_eq!("helloWorld", "HELLOWORLD");
    }

    #[test]
    fn test_pascalcase() {
        const TYPE: &str = "PascalCase";

        test_eq!("HelloWorld", "HelloWorld");
        test_eq!("Hello_World", "HelloWorld");
        test_eq!("Hello-World", "HelloWorld");
        test_eq!("helloWorld", "HelloWorld");
    }

    #[test]
    fn test_camelcase() {
        const TYPE: &str = "camelCase";

        test_eq!("HelloWorld", "helloWorld");
        test_eq!("Hello_World", "helloWorld");
        test_eq!("Hello-World", "helloWorld");
        test_eq!("helloWorld", "helloWorld");
    }

    #[test]
    fn test_snakecase() {
        const TYPE: &str = "snake_case";

        test_eq!("HelloWorld", "hello_world");
        test_eq!("Hello_World", "hello_world");
        test_eq!("Hello-World", "hello_world");
        test_eq!("helloWorld", "hello_world");
    }

    #[test]
    fn test_screaming_snakecase() {
        const TYPE: &str = "SCREAMING_SNAKE_CASE";

        test_eq!("HelloWorld", "HELLO_WORLD");
        test_eq!("Hello_World", "HELLO_WORLD");
        test_eq!("Hello-World", "HELLO_WORLD");
        test_eq!("helloWorld", "HELLO_WORLD");
    }

    #[test]
    fn test_kebabcase() {
        const TYPE: &str = "kebab-case";

        test_eq!("HelloWorld", "hello-world");
        test_eq!("Hello_World", "hello-world");
        test_eq!("Hello-World", "hello-world");
        test_eq!("helloWorld", "hello-world");
    }

    #[test]
    fn test_screaming_kebabcase() {
        const TYPE: &str = "SCREAMING-KEBAB-CASE";

        test_eq!("HelloWorld", "HELLO-WORLD");
        test_eq!("Hello_World", "HELLO-WORLD");
        test_eq!("Hello-World", "HELLO-WORLD");
        test_eq!("helloWorld", "HELLO-WORLD");
    }
}
