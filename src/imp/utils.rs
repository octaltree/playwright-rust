use crate::imp::prelude::*;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Viewport {
    pub width: i32,
    pub height: i32
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ProxySettings {
    pub server: String,
    pub bypass: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Geolocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: Option<f64>
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HttpCredentials {
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorScheme {
    Dark,
    Light,
    NoPreference
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<Cookie>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<OriginState>>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_site: Option<SameSite>
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SameSite {
    Lax,
    None,
    Strict
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginState {
    pub origin: String,
    pub local_storage: Vec<LocalStorageEntry>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalStorageEntry {
    pub name: String,
    pub value: String
}
