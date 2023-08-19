use facti_lib::version::Version;
use serde::{Deserialize, Serialize};

/// Latest releases for each channel of the game.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct LatestReleases {
    /// Latest available versions for the stable channel.
    pub stable: LatestRelease,

    /// Latest available versions for the experimental channel.
    pub experimental: LatestRelease,
}

/// Latest release for a specific channel of the game.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct LatestRelease {
    /// Latest available version for the stable channel.
    pub alpha: Version,

    /// Latest available version for the demo.
    pub demo: Version,

    /// Latest available version for the headless server.
    ///
    /// The headless server can be downloaded without being authenticated.
    pub headless: Version,
}
