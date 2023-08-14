use facti_lib::changelog;

#[test]
fn test_basic_changelog() {
    let content = include_str!("parsing/changelog.txt");
    let changelog = changelog::parse(content);

    assert!(changelog.is_ok());
    dbg!(changelog.unwrap());
}

#[test]
fn test_multi_section() {
    let content = include_str!("parsing/multi_section.txt");
    let changelog = changelog::parse(content);

    assert!(changelog.is_ok());
}

#[test]
fn test_colon_in_heading() {
    let content = include_str!("parsing/colon_in_heading.txt");
    let changelog = changelog::parse(content);

    assert!(changelog.is_ok());
}

#[test]
fn test_real_parse() {
    let content = include_str!("parsing/small.txt");
    let changelog = changelog::parse(content);

    assert!(changelog.is_ok());
}

#[test]
fn test_minimal() {
    let content = include_str!("parsing/minimal.txt");
    let changelog = changelog::parse(content);

    assert!(changelog.is_ok());
}

#[test]
fn test_wrong_section_order() {
    let content = include_str!("parsing/wrong_section_order.txt");
    let changelog = changelog::parse(content);

    // TODO: Add asserts to check it's now in correct order
    assert!(changelog.is_ok());
}
