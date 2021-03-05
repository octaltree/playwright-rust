use crate::imp::prelude::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Viewport {
    pub width: i32,
    pub height: i32
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
