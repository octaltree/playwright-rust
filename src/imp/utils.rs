use crate::imp::prelude::*;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Viewport {
    pub width: i32,
    pub height: i32
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ProxySettings {
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Geolocation {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HttpCredentials {
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
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

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
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

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DocumentLoadState {
    DomContentLoaded,
    Load,
    NetworkIdle
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
pub enum KeyboardModifier {
    Alt,
    Control,
    Meta,
    Shift
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Middle,
    Right
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub struct Position {
    x: f64,
    y: f64
}

impl From<(f64, f64)> for Position {
    fn from((x, y): (f64, f64)) -> Self { Self { x, y } }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub struct FloatRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ScreenshotType {
    Jpeg,
    Png
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ElementState {
    Disabled,
    Editable,
    Enabled,
    Hidden,
    Stable,
    Visible
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Header {
    pub name: String,
    pub value: String
}

impl From<Header> for (String, String) {
    fn from(Header { name, value }: Header) -> Self { (name, value) }
}

impl From<(String, String)> for Header {
    fn from((k, v): (String, String)) -> Self { Self { name: k, value: v } }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Length<'a> {
    Value(f64),
    WithUnit(&'a str)
}

impl<'a> From<f64> for Length<'a> {
    fn from(x: f64) -> Self { Self::Value(x) }
}

impl<'a> From<&'a str> for Length<'a> {
    fn from(x: &'a str) -> Self { Self::WithUnit(x) }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct PdfMargins<'a, 'b, 'c, 'd> {
    top: Option<Length<'a>>,
    right: Option<Length<'b>>,
    bottom: Option<Length<'c>>,
    left: Option<Length<'d>>
}

#[derive(Debug, Serialize, PartialEq)]
pub struct File {
    name: String,
    mime: String,
    buffer: String
}

impl File {
    pub fn new(name: String, mime: String, body: &[u8]) -> Self {
        let buffer = base64::encode(body);
        Self { name, mime, buffer }
    }
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BrowserChannel {
    Chrome,
    ChromeBeta,
    ChromeDev,
    ChromeCanary,
    Msedge,
    MsedgeBeta,
    MsedgeDev,
    MsedgeCanary
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    pub url: String,
    pub line_number: i32,
    pub column_number: i32
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTiming {
    pub start_time: f64,
    pub domain_lookup_start: f64,
    pub domain_lookup_end: f64,
    pub connect_start: f64,
    pub secure_connection_start: f64,
    pub connect_end: f64,
    pub request_start: f64,
    pub response_start: f64
}
