use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "changelog/grammar.pest"]
pub struct ChangelogParser;
