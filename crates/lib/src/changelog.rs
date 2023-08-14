use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

use crate::version::Version;

#[derive(Debug)]
pub struct Changelog {
    pub sections: Vec<Section>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Section {
    pub version: Version,
    pub date: Option<String>,
    pub categories: HashMap<CategoryType, HashSet<String>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CategoryType {
    MajorFeatures,
    Features,
    MinorFeatures,
    Graphics,
    Sounds,
    Optimizations,
    Balancing,
    CombatBalancing,
    CircuitNetwork,
    Changes,
    Bugfixes,
    Modding,
    Scripting,
    Gui,
    Control,
    Translation,
    Debug,
    EaseOfUse,
    Info,
    Locale,
    Other(String),
}

#[derive(Parser)]
#[grammar = "changelog/grammar.pest"]
struct ChangelogParser;

#[derive(Error, Debug)]
pub enum ChangelogParseError {
    #[error("Pest error when parsing")]
    Pest(#[source] Box<pest::error::Error<Rule>>),
}

pub fn parse(content: &str) -> Result<Changelog, ChangelogParseError> {
    let mut result = Changelog { sections: vec![] };

    let changelog = ChangelogParser::parse(Rule::changelog, content)
        .map_err(|e| ChangelogParseError::Pest(Box::new(e)))?
        .next()
        .unwrap();

    for section_pair in changelog.into_inner() {
        match section_pair.as_rule() {
            Rule::section => {
                let mut inner_rules = section_pair.into_inner();
                let ver_str = inner_rules.next().unwrap().as_str();
                let version = Version::from_str(ver_str).unwrap();

                let mut section = Section {
                    version,
                    date: None,
                    categories: HashMap::new(),
                };

                for remaining in inner_rules {
                    match remaining.as_rule() {
                        Rule::date => {
                            section.date = Some(remaining.as_str().to_owned());
                        }
                        Rule::category => {
                            let mut inner_rules = remaining.into_inner();
                            let category_type =
                                CategoryType::from_str(inner_rules.next().unwrap().as_str())
                                    .unwrap();
                            let entries = section.categories.entry(category_type).or_default();

                            for entry_pair in inner_rules {
                                match entry_pair.as_rule() {
                                    Rule::entry => {
                                        let str = entry_pair
                                            .into_inner()
                                            .map(|e| e.as_str())
                                            .collect::<Vec<_>>()
                                            .join("\n");

                                        entries.insert(str);
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                result.sections.push(section);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    result.sections.sort_by(|a, b| b.version.cmp(&a.version));

    Ok(result)
}

impl FromStr for CategoryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Major Features" => Ok(Self::MajorFeatures),
            "Features" => Ok(Self::Features),
            "Minor Features" => Ok(Self::MinorFeatures),
            "Graphics" => Ok(Self::Graphics),
            "Sounds" => Ok(Self::Sounds),
            "Optimizations" => Ok(Self::Optimizations),
            "Balancing" => Ok(Self::Balancing),
            "Combat Balancing" => Ok(Self::CombatBalancing),
            "Circuit Network" => Ok(Self::CircuitNetwork),
            "Changes" => Ok(Self::Changes),
            "Bugfixes" => Ok(Self::Bugfixes),
            "Modding" => Ok(Self::Modding),
            "Scripting" => Ok(Self::Scripting),
            "Gui" => Ok(Self::Gui),
            "Control" => Ok(Self::Control),
            "Translation" => Ok(Self::Translation),
            "Debug" => Ok(Self::Debug),
            "Ease of use" => Ok(Self::EaseOfUse),
            "Info" => Ok(Self::Info),
            "Locale" => Ok(Self::Locale),
            o => Ok(Self::Other(o.to_owned())),
        }
    }
}
