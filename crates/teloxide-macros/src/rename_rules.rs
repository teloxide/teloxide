// Some concepts are from Serde.

use crate::error::{compile_error, Result};

use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
};

#[derive(Copy, Clone, Debug)]
pub(crate) enum RenameRule {
    /// -> `lowercase`
    LowerCase,
    /// -> `UPPERCASE`
    UpperCase,
    /// -> `PascalCase`
    PascalCase,
    /// -> `camelCase`
    CamelCase,
    /// -> `snake_case`
    SnakeCase,
    /// -> `SCREAMING_SNAKE_CASE`
    ScreamingSnakeCase,
    /// -> `kebab-case`
    KebabCase,
    /// -> `SCREAMING-KEBAB-CASE`
    ScreamingKebabCase,
    /// Leaves input as-is
    Identity,
}

impl RenameRule {
    /// Apply a renaming rule to a string, returning the version expected in the
    /// source.
    ///
    /// See tests for the details how it will work.
    pub fn apply(self, input: &str) -> String {
        use RenameRule::*;

        match self {
            LowerCase => input.to_lowercase(),
            UpperCase => input.to_uppercase(),
            PascalCase => input.to_pascal_case(),
            CamelCase => input.to_lower_camel_case(),
            SnakeCase => input.to_snake_case(),
            ScreamingSnakeCase => input.to_shouty_snake_case(),
            KebabCase => input.to_kebab_case(),
            ScreamingKebabCase => input.to_shouty_kebab_case(),
            Identity => input.to_owned(),
        }
    }

    pub fn parse(rule: &str) -> Result<Self> {
        use RenameRule::*;

        let rule = match rule {
            "lowercase" => LowerCase,
            "UPPERCASE" => UpperCase,
            "PascalCase" => PascalCase,
            "camelCase" => CamelCase,
            "snake_case" => SnakeCase,
            "SCREAMING_SNAKE_CASE" => ScreamingSnakeCase,
            "kebab-case" => KebabCase,
            "SCREAMING-KEBAB-CASE" => ScreamingKebabCase,
            "identity" => Identity,
            invalid => {
                return Err(compile_error(format!(
                    "invalid rename rule `{invalid}` (supported rules: `lowercase`, `UPPERCASE`, \
                     `PascalCase`, `camelCase`, `snake_case`, `SCREAMING_SNAKE_CASE`, \
                     `kebab-case`, `SCREAMING-KEBAB-CASE` and `identity`)"
                )))
            }
        };

        Ok(rule)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($input:expr => $output:expr) => {
            let rule = RenameRule::parse(TYPE).unwrap();

            assert_eq!(rule.apply($input), $output);
        };
    }

    #[test]
    fn test_lowercase() {
        const TYPE: &str = "lowercase";

        test_eq!("HelloWorld" => "helloworld");
        test_eq!("Hello_World" => "hello_world");
        test_eq!("Hello-World" => "hello-world");
        test_eq!("helloWorld" => "helloworld");
    }

    #[test]
    fn test_uppercase() {
        const TYPE: &str = "UPPERCASE";

        test_eq!("HelloWorld" => "HELLOWORLD");
        test_eq!("Hello_World" => "HELLO_WORLD");
        test_eq!("Hello-World" => "HELLO-WORLD");
        test_eq!("helloWorld" => "HELLOWORLD");
    }

    #[test]
    fn test_pascalcase() {
        const TYPE: &str = "PascalCase";

        test_eq!("HelloWorld" => "HelloWorld");
        test_eq!("Hello_World" => "HelloWorld");
        test_eq!("Hello-World" => "HelloWorld");
        test_eq!("helloWorld" => "HelloWorld");
    }

    #[test]
    fn test_camelcase() {
        const TYPE: &str = "camelCase";

        test_eq!("HelloWorld" => "helloWorld");
        test_eq!("Hello_World" => "helloWorld");
        test_eq!("Hello-World" => "helloWorld");
        test_eq!("helloWorld" => "helloWorld");
    }

    #[test]
    fn test_snakecase() {
        const TYPE: &str = "snake_case";

        test_eq!("HelloWorld" => "hello_world");
        test_eq!("Hello_World" => "hello_world");
        test_eq!("Hello-World" => "hello_world");
        test_eq!("helloWorld" => "hello_world");
    }

    #[test]
    fn test_screaming_snakecase() {
        const TYPE: &str = "SCREAMING_SNAKE_CASE";

        test_eq!("HelloWorld" => "HELLO_WORLD");
        test_eq!("Hello_World" => "HELLO_WORLD");
        test_eq!("Hello-World" => "HELLO_WORLD");
        test_eq!("helloWorld" => "HELLO_WORLD");
    }

    #[test]
    fn test_kebabcase() {
        const TYPE: &str = "kebab-case";

        test_eq!("HelloWorld" => "hello-world");
        test_eq!("Hello_World" => "hello-world");
        test_eq!("Hello-World" => "hello-world");
        test_eq!("helloWorld" => "hello-world");
    }

    #[test]
    fn test_screaming_kebabcase() {
        const TYPE: &str = "SCREAMING-KEBAB-CASE";

        test_eq!("HelloWorld" => "HELLO-WORLD");
        test_eq!("Hello_World" => "HELLO-WORLD");
        test_eq!("Hello-World" => "HELLO-WORLD");
        test_eq!("helloWorld" => "HELLO-WORLD");
    }
}
