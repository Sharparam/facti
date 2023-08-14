use facti_lib::changelog::{ChangelogParser, Rule};
use pest::Parser;

#[test]
fn test_basic_changelog() {
    let content = include_str!("parsing/changelog.txt");
    let changelog = ChangelogParser::parse(Rule::changelog, content);

    assert!(changelog.is_ok());
}

#[test]
fn test_multi_section() {
    let content = include_str!("parsing/multi_section.txt");
    let changelog = ChangelogParser::parse(Rule::changelog, content);

    assert!(changelog.is_ok());
}

#[test]
fn test_colon_in_heading() {
    let content = include_str!("parsing/colon_in_heading.txt");
    let changelog = ChangelogParser::parse(Rule::changelog, content);

    assert!(changelog.is_ok());
}
