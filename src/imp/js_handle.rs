use crate::imp::{core::*, prelude::*};
use std::fmt;

#[derive(Debug)]
pub(crate) struct JsHandle {
    channel: ChannelOwner,
    var: Mutex<Var>
}

#[derive(Debug)]
struct Var {
    preview: String
}

impl JsHandle {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { preview } = serde_json::from_value(channel.initializer.clone())?;
        let var = Mutex::new(Var { preview });
        Ok(Self { channel, var })
    }

    pub(crate) async fn get_property(&self, name: &str) -> ArcResult<Weak<JsHandle>> {
        let mut args = HashMap::new();
        args.insert("name", name);
        let v = send_message!(self, "getProperty", args);
        let guid = only_guid(&v)?;
        let j = get_object!(self.context()?.lock(), guid, JsHandle)?;
        Ok(j)
    }

    pub(crate) async fn get_properties(&self) -> ArcResult<HashMap<String, Weak<JsHandle>>> {
        let v = send_message!(self, "getPropertyList", Map::new());
        let first = first(&v).ok_or(Error::InvalidParams)?;
        let properties: Vec<Property> =
            serde_json::from_value((*first).clone()).map_err(Error::Serde)?;
        let ps = properties
            .into_iter()
            .map(
                |Property {
                     name,
                     value: OnlyGuid { guid }
                 }| {
                    get_object!(self.context()?.lock(), &guid, JsHandle).map(|o| (name, o))
                }
            )
            .collect::<Result<HashMap<_, _>, Error>>()?;
        Ok(ps)
    }

    pub(crate) async fn dispose(&self) -> ArcResult<()> {
        let _ = send_message!(self, "dispose", Map::new());
        Ok(())
    }

    pub(crate) async fn json_value<U>(&self) -> ArcResult<U>
    where
        U: DeserializeOwned
    {
        let v = send_message!(self, "jsonValue", Map::new());
        let first = first(&v).ok_or(Error::ObjectNotFound)?;
        Ok(de::from_value(first).map_err(Error::DeserializationPwJson)?)
    }
}

impl JsHandle {
    fn set_preview(&self, preview: String) {
        let var = &mut self.var.lock();
        var.preview = preview;
    }

    fn on_preview_updated(&self, params: Map<String, Value>) -> Result<(), Error> {
        #[derive(Deserialize)]
        struct De {
            preview: String
        }
        let De { preview } = serde_json::from_value(params.into())?;
        self.set_preview(preview);
        Ok(())
    }
}

impl RemoteObject for JsHandle {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }

    fn handle_event(
        &self,
        _ctx: &Context,
        method: Str<Method>,
        params: Map<String, Value>
    ) -> Result<(), Error> {
        if method.as_str() == "previewUpdated" {
            self.on_preview_updated(params)?;
        }
        Ok(())
    }
}

impl fmt::Display for JsHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.var.lock().preview)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    preview: String
}

#[derive(Deserialize)]
struct Property {
    name: String,
    value: OnlyGuid
}
