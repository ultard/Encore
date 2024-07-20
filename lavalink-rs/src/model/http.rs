use crate::model::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyo3::pyclass(get_all, set_all))]
/// Updates or creates the player for this guild.
///
/// If every field is None, the player will stop playing.
pub struct UpdatePlayer {
    /// The track to play.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<UpdatePlayerTrack>,
    /// The track end time in milliseconds.
    ///
    /// It must be a value above 0 or None.
    ///
    /// None resets this if it was set previously.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// The player volume.
    ///
    /// In percentage, from 0 to 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<u16>,
    /// The track position in milliseconds.
    ///
    /// This value can be set to start a track at a specific time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// Whether the player should be paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// The filters to apply.
    ///
    /// This will override all previously applied filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<player::Filters>,
    /// The discord websocket connection information.
    ///
    /// Required for creating a player.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<player::ConnectionInfo>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct UpdatePlayerTrack {
    /// The base64 encoded track to play.
    ///
    /// Mutually exclusive with `identifier`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded: Option<String>,
    /// The identifier of the track to play.
    ///
    /// Mutually exclusive with `encoded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Additional track data to be sent back with the `Track` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::pyclass(get_all, set_all))]
/// Updates the session with the resuming state and timeout.
///
/// You must call this method if you wish to restart the discord bot without having all players
/// stop, and provide the current `session_id` when creating the node connection.
pub struct ResumingState {
    /// Whether resuming should be, or is enabled for this session or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resuming: Option<bool>,
    /// The timeout in seconds.
    ///
    /// default is 60s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyo3::pyclass(get_all, set_all))]
/// Information about the Lavalink node.
pub struct Info {
    /// The semver version of the Lavalink server.
    pub version: Version,
    /// The millisecond unix timestamp when the Lavalink jar was built.
    pub build_time: u64,
    /// The git information of the Lavalink server.
    pub git: Git,
    /// The JVM version the Lavalink server is running on.
    pub jvm: String,
    /// The Lavaplayer version being used by the Lavalink server.
    pub lavaplayer: String,
    /// The enabled source managers for the Lavalink server.
    pub source_managers: Vec<String>,
    /// The enabled filters for the Lavalink server.
    pub filters: Vec<String>,
    /// The enabled plugins for the Lavalink server.
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyo3::pyclass(get_all, set_all))]
pub struct Git {
    /// The branch the Lavalink server was built on.
    pub branch: String,
    /// The commit the Lavalink server was built on.
    pub commit: String,
    /// The millisecond unix timestamp for when the commit was created.
    pub commit_time: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::pyclass(get_all, set_all))]
pub struct Plugin {
    /// The name of the plugin
    pub name: String,
    /// The version of the plugin
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyo3::pyclass(get_all, set_all))]
/// Check out [Semantic Versioning 2.0.0](https://semver.org/) to know what these fields mean.
pub struct Version {
    pub semver: String,
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub pre_release: Option<String>,
    pub build: Option<String>,
}
