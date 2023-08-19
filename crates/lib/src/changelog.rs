use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Write},
    str::FromStr,
};

use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::version::Version;

/// Version section start is a sequence of 99 dashes exactly.
const SECTION_START: &str = "---------------------------------------------------------------------------------------------------";

/// Contains all the sections part of a Factorio mod changelog.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Changelog {
    pub sections: Vec<Section>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Section {
    pub version: Version,
    pub date: Option<String>,
    pub categories: HashMap<CategoryType, HashSet<String>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
pub enum ParseChangelogError {
    #[error("Pest error when parsing")]
    Pest(#[source] Box<pest::error::Error<Rule>>),
}

impl Changelog {
    /// Parses a [`Changelog`] from a string.
    ///
    /// The given string must be a valid changelog as specified by
    /// [the changelog format spec][spec].
    ///
    /// [spec]: https://wiki.factorio.com/Tutorial:Mod_changelog_format
    pub fn parse<T: AsRef<str>>(s: T) -> Result<Self, ParseChangelogError> {
        s.as_ref().parse()
    }

    /// Sorts the [`sections`][Changelog::sections] by version.
    pub fn sort(&mut self) {
        self.sections.sort_by(|a, b| b.version.cmp(&a.version));
    }

    /// Converts the [`Changelog`] to a string, with sorted sections according
    /// to [`Section::version`].
    pub fn to_string_sorted(&self) -> Result<String, fmt::Error> {
        let mut sorted = self.to_owned();
        sorted.sort();

        let mut s = String::new();

        for section in sorted.sections {
            write!(s, "{}", section)?;
        }

        Ok(s)
    }
}

impl FromStr for Changelog {
    type Err = ParseChangelogError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self { sections: vec![] };

        let changelog = ChangelogParser::parse(Rule::changelog, s)
            .map_err(|e| ParseChangelogError::Pest(Box::new(e)))?
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
}

impl Display for Changelog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for section in &self.sections {
            write!(f, "{}", section)?;
        }

        Ok(())
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\nVersion: {}", SECTION_START, self.version)?;

        if let Some(date) = &self.date {
            writeln!(f, "Date: {}", date)?;
        }

        for (category, entries) in &self.categories {
            writeln!(f, "  {}:", category)?;

            for entry in entries {
                let mut lines = entry.lines();
                if let Some(line) = lines.next() {
                    writeln!(f, "    - {}", line)?;
                }
                for line in lines {
                    writeln!(f, "      {}", line)?;
                }
            }
        }

        Ok(())
    }
}

impl FromStr for CategoryType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CategoryType::*;
        Ok(match s {
            "Major Features" => MajorFeatures,
            "Features" => Features,
            "Minor Features" => MinorFeatures,
            "Graphics" => Graphics,
            "Sounds" => Sounds,
            "Optimizations" => Optimizations,
            "Balancing" => Balancing,
            "Combat Balancing" => CombatBalancing,
            "Circuit Network" => CircuitNetwork,
            "Changes" => Changes,
            "Bugfixes" => Bugfixes,
            "Modding" => Modding,
            "Scripting" => Scripting,
            "Gui" => Gui,
            "Control" => Control,
            "Translation" => Translation,
            "Debug" => Debug,
            "Ease of use" => EaseOfUse,
            "Info" => Info,
            "Locale" => Locale,
            o => Other(o.to_owned()),
        })
    }
}

impl Display for CategoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CategoryType::*;
        f.write_str(match self {
            MajorFeatures => "Major Features",
            Features => "Features",
            MinorFeatures => "Minor Features",
            Graphics => "Graphics",
            Sounds => "Sounds",
            Optimizations => "Optimizations",
            Balancing => "Balancing",
            CombatBalancing => "Combat Balancing",
            CircuitNetwork => "Circuit Network",
            Changes => "Changes",
            Bugfixes => "Bugfixes",
            Modding => "Modding",
            Scripting => "Scripting",
            Gui => "Gui",
            Control => "Control",
            Translation => "Translation",
            Debug => "Debug",
            EaseOfUse => "Ease of use",
            Info => "Info",
            Locale => "Locale",
            Other(o) => o,
        })
    }
}
