use crate::imp::prelude::*;

pub use crate::imp::playwright::DeviceDescriptor;

#[derive(Debug, Deserialize, Clone)]
pub struct Viewport {
    pub width: i32,
    pub height: i32
}

impl<'de> Deserialize<'de> for DeviceDescriptor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct DeviceDescriptorImpl {
            name: String,
            descriptor: Descriptor
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Descriptor {
            user_agent: String,
            viewport: Viewport,
            device_scale_factor: f64,
            is_mobile: bool,
            has_touch: bool,
            default_browser_type: String
        }
        let DeviceDescriptorImpl {
            name,
            descriptor:
                Descriptor {
                    user_agent,
                    viewport,
                    device_scale_factor,
                    is_mobile,
                    has_touch,
                    default_browser_type
                }
        } = DeviceDescriptorImpl::deserialize(deserializer)?;
        Ok(DeviceDescriptor {
            name,
            user_agent,
            viewport,
            device_scale_factor,
            is_mobile,
            has_touch,
            default_browser_type
        })
    }
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
