use clap::{Args, Subcommand, ValueHint};
use facti_api::blocking::ApiClient;
use facti_lib::FactorioVersion;

/// Interact with the Factorio mod portal.
///
/// You can search for mods and show details for a specific mod.
#[derive(Args, Debug)]
pub struct PortalArgs {
    /// Output results as JSON.
    ///
    /// Tip: You can use jq to work with JSON!
    #[arg(short, long)]
    pub json: bool,

    #[command(subcommand)]
    pub command: PortalCommands,
}

#[derive(Subcommand, Debug)]
pub enum PortalCommands {
    /// Search for mods.
    Search(PortalSearchArgs),

    /// Show details about a specific mod.
    Show(PortalShowArgs),
}

#[derive(clap::Args, Debug)]
pub struct PortalSearchArgs {
    /// Include deprecated mods in the search results.
    ///
    /// Default is to not include deprecated mods.
    #[arg(short, long)]
    pub deprecated: bool,

    /// Page number you would like to show.
    #[arg(short, long, default_value_t = 1)]
    pub page: u32,

    /// The amount of results to show, specify 'max' for maximum possible.
    #[arg(short = 'n', long = "size", value_name = "SIZE")]
    pub page_size: Option<facti_api::data::pagination::PageSize>,

    /// The property to sort results by, defaults to name if not given.
    ///
    /// Any of the following are valid:
    ///  - name
    ///  - created or created_at
    ///  - updated or updated_at
    #[arg(short, long = "sort", value_name = "SORT")]
    pub sort_mode: Option<facti_api::data::sorting::SortMode>,

    /// Select whether to sort ascending or descending, default is descending.
    ///
    /// It accepts the following:
    ///  - ascending or asc
    ///  - descending or desc
    #[arg(short = 'o', long = "order", value_name = "ORDER")]
    pub sort_order: Option<facti_api::data::sorting::SortOrder>,

    /// Only return non-deprecated mods compatible with the specified Factorio version.
    ///
    /// Specifying this together with --deprecated will result in the deprecated
    /// flag not having any effect.
    ///
    /// The version must be given in the format of MAJOR.MINOR.
    #[arg(short, long)]
    pub factorio_version: Option<FactorioVersion>,

    /// Limit the results to mods matching the given names.
    #[arg(value_hint = ValueHint::Other)]
    pub names: Vec<String>,
}

#[derive(clap::Args, Debug)]
pub struct PortalShowArgs {
    /// If given, will show more details about the mod.
    #[arg(short, long)]
    pub full: bool,

    pub name: String,
}

impl PortalArgs {
    pub fn run(&self, client: &ApiClient) -> anyhow::Result<()> {
        match &self.command {
            PortalCommands::Search(args) => args.run(client, self.json),
            PortalCommands::Show(args) => args.run(client, self.json),
        }
    }
}

impl PortalSearchArgs {
    pub fn run(&self, client: &ApiClient, json: bool) -> anyhow::Result<()> {
        let query = facti_api::data::portal::SearchQuery {
            hide_deprecated: !self.deprecated,
            page: self.page,
            page_size: self.page_size,
            sort: self.sort_mode,
            sort_order: self.sort_order,
            version: self.factorio_version,
            namelist: match &self.names {
                names if names.is_empty() => None,
                names => Some(names.to_vec()),
            },
        };

        let response = client.search(&query)?;

        if json {
            println!("{}", serde_json::to_string_pretty(&response)?);
        } else {
            for item in response.results {
                println!("{}", item);
            }
        }

        Ok(())
    }
}

impl PortalShowArgs {
    pub fn run(&self, client: &ApiClient, json: bool) -> anyhow::Result<()> {
        let response = if self.full {
            client.info_full(&self.name)?
        } else {
            client.info_short(&self.name)?
        };

        if json {
            println!("{}", serde_json::to_string_pretty(&response)?);
        } else {
            println!("{}", response);
        }

        Ok(())
    }
}
