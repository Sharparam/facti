#![doc = include_str!("../README.md")]

use anyhow::Context;
use facti_api::{portal::SearchQuery, ApiClient};
use facti_lib::{
    dependency::Dependency,
    version::{Version, VersionReq},
    FactorioVersion, ModInfo, ModInfoBuilder,
};
use url::Url;

fn main() -> anyhow::Result<()> {
    test_modinfo()?;
    test_api()?;

    Ok(())
}

fn test_api() -> anyhow::Result<()> {
    let client = ApiClient::builder().build();

    let search_query = SearchQuery {
        namelist: Some(vec!["cybersyn".to_string()]),
        ..Default::default()
    };

    let search_result = client.search(search_query)?;

    println!("search result: {:#?}", search_result);

    let short = client.info_short("cybersyn-combinator")?;
    println!("short: {:#?}", short);

    let full = client.info_full("cybersyn-combinator")?;
    println!("full: {:#?}", full);

    Ok(())
}

fn test_modinfo() -> anyhow::Result<()> {
    // Explicitly leave out contact to test serialize of None
    let info = ModInfoBuilder::new(
        "cybersyn-combinator",
        Version::new(1, 2, 3),
        "Cybersyn Combinator",
        "Sharparam <sharparam@sharparam.com>",
    )
    .homepage(Url::parse("https://sharparam.com").unwrap())
    .description("My fancy mod!")
    .factorio_version(FactorioVersion::new(1, 71))
    .dependency(Dependency::required("project-cybersyn", VersionReq::Latest))
    .dependency(Dependency::optional("nullius", VersionReq::Latest, true))
    .dependency(Dependency::parse("? krastorio2 = 6.6.6")?)
    .dependency(Dependency::independent(
        "lonami",
        VersionReq::parse(">= 4.2.0").unwrap(),
    ))
    .build();
    let json = serde_json::to_string(&info)?;
    println!("{}", json);

    let path = "/home/sharparam/repos/github.com/Sharparam/cybersyn-combinator/src/info.json";
    let json = std::fs::read_to_string(path).expect("failed to read info.json");
    let info: ModInfo = serde_json::from_str(&json).context(format!(
        "Failed to deserialize mod info from file: {}",
        path
    ))?;

    println!("Successfully parsed {}: {:#?}", info, info);

    Ok(())
}
