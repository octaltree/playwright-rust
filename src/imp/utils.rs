use crate::imp::prelude::*;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Viewport {
    pub width: i32,
    pub height: i32
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ProxySettings {
    /// Proxy to be used for all requests. HTTP and SOCKS proxies are supported, for example `http://myproxy.com:3128` or\n`socks5://myproxy.com:3128`. Short form `myproxy.com:3128` is considered an HTTP proxy.
    pub server: String,
    /// Optional coma-separated domains to bypass proxy, for example `\".com, chromium.org, .domain.com\"`.
    pub bypass: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Geolocation {
    /// Latitude between -90 and 90.
    pub latitude: f64,
    /// Longitude between -180 and 180.
    pub longitude: f64,
    /// Non-negative accuracy value. Defaults to `0`.
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

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct StorageState {
    pub cookies: Option<Vec<Cookie>>,
    pub origins: Option<Vec<OriginState>>
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub path: Option<String>,
    /// Optional Unix time in seconds.
    pub expires: Option<f64>,
    pub http_only: Option<bool>,
    pub secure: Option<bool>,
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
    /// the x coordinate of the element in pixels.
    x: f64,
    y: f64,
    /// the width of the element in pixels.
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
    /// 0-based line number in the resource.
    pub line_number: i32,
    /// 0-based column number in the resource.
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
