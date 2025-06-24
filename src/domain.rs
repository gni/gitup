use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitUserConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Profile {
    pub name: String,
    pub config: GitUserConfig,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default)]
    pub profiles: HashMap<String, GitUserConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_profile: Option<String>,
}
