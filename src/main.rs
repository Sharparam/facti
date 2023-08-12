use facti::factorio::{
    dependency::Dependency,
    version::{Version, VersionReq},
    FactorioVersion, ModInfo, ModInfoBuilder,
};

fn main() {
    // Explicitly leave contact null to test serialize of None
    let info = ModInfoBuilder::new(
        "cybersyn-combinator",
        Version::new(1, 2, 3),
        "Cybersyn Combinator",
        "Sharparam <sharparam@sharparam.com>",
    )
    .homepage("https://sharparam.com")
    .description("My fancy mod!")
    .factorio_version(FactorioVersion::new(1, 71))
    .dependency(Dependency::required("project-cybersyn", VersionReq::Latest))
    .dependency(Dependency::optional("nullius", VersionReq::Latest, true))
    .dependency(Dependency::parse("? krastorio2 = 6.6.6").unwrap())
    .dependency(Dependency::independent(
        "lonami",
        VersionReq::parse(">= 4.2.0").unwrap(),
    ))
    .build();
    let json = serde_json::to_string(&info);
    println!("{}", json.unwrap());

    let path = "/home/sharparam/repos/github.com/Sharparam/cybersyn-combinator/src/info.json";
    let json = std::fs::read_to_string(path).expect("failed to read info.json");
    let info: ModInfo = serde_json::from_str(&json).expect("failed to deserialize info");

    println!("Successfully parsed {}: {:#?}", info, info);
}
