#![forbid(unsafe_code)]

use regex::Regex;

/// Checks that the string fits the template. Returns true if it does. False otherwise.
/// # Arguments
/// * `expression` - a template
/// * `name` - a string to match
///
/// # Examples
///
/// ```console
/// let expression = "a(.*)a";
/// let name_1 = "abacaba";
/// let name_2 = "baa";
/// let marker_1 = ;
/// let marker_10 = helpers::create_marker(number_10);
///
/// assert_eq!(match_source_pattern(expression, name_1), true);
/// assert_eq!(match_source_pattern(expression, name_2), false);
/// ```
pub fn match_source_pattern(expression: &str, name: &str) -> bool {
    let markers = Regex::new(expression).unwrap().captures(name);
    if markers.is_none() {
        return false;
    }
    if markers.unwrap()[0] != *name {
        return false;
    }
    true
}

#[test]
fn test_match_source_pattern() {
    let expression = "a(.*)a";
    let name_1 = "abacaba";
    let name_2 = "baa";

    assert!(match_source_pattern(expression, name_1));
    assert!(!match_source_pattern(expression, name_2));
}

/// Replaces the `*` characters in the template with `(.*)` and the `.` character with `\\.`
/// # Arguments
/// * `template` - a template to change
/// # Examples
///
/// ```console
/// let template = "a*a.*";
///
/// assert_eq!(get_expression(template), "a(.*)\\.(.*)");
/// ```
pub fn get_expression(template: &str) -> Result<String, String> {
    let special_characters = ['^', '$', '+', '-', '?', '(', ')', '[', ']', '{', '}', '|'];
    let mut expression = String::new();
    for character in template.chars() {
        if special_characters.contains(&character) {
            return Err("Special characters are not allowed in the source template".to_string());
        }
        if character == '*' {
            expression.push_str("(.*)");
        } else if character == '.' {
            expression.push_str("\\.");
        } else {
            expression.push(character);
        }
    }
    Ok(expression)
}

#[test]
fn test_get_expression() {
    assert_eq!(get_expression("*"), Ok("(.*)".to_string()));
    assert_eq!(
        get_expression("Hello, World!"),
        Ok("Hello, World!".to_string())
    );
    assert_eq!(
        get_expression("a*b*c.d**d.txt"),
        Ok("a(.*)b(.*)c\\.d(.*)(.*)d\\.txt".to_string())
    );
    assert_eq!(
        get_expression("Hello, World?"),
        Err("Special characters are not allowed in the source template".to_string())
    );
}

/// A simple tool that converts a number to marker string.
///
/// # Arguments
/// * `marker_number` - a number to convert
///
/// # Examples
///
/// ```console
/// let number_1 = 1;
/// let number_10 = 10;
///
/// assert_eq!(create_marker(number_1), "#1");
/// assert_eq!(create_marker(number_10), "#{10}");
/// ```
pub fn create_marker(marker_number: usize) -> String {
    let mut res = "#".to_string();
    if marker_number >= 10 {
        res.push('{');
        res.push_str(marker_number.to_string().as_str());
        res.push('}');
    } else {
        res.push_str(marker_number.to_string().as_str());
    }
    res
}

#[test]
fn test_create_marker() {
    assert_eq!(create_marker(1), "#1");
    assert_eq!(create_marker(19), "#{19}");
    assert_eq!(create_marker(1337), "#{1337}");
}
