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
    username: String,
    password: String
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
    // cookies: Optional[List[Cookie]]
// origins: Optional[List[OriginState]]
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cookie {
    // name: str
// value: str
// url: Optional[str]
// domain: Optional[str]
// path: Optional[str]
// expires: Optional[float]
// httpOnly: Optional[bool]
// secure: Optional[bool]
// sameSite: Optional[Literal["Lax", "None", "Strict"]]
}
