use crate::imp::{core::*, prelude::*};

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
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { preview } = serde_json::from_value(channel.initializer.clone())?;
        let var = Mutex::new(Var { preview });
        Ok(Self { channel, var })
    }

    pub(crate) async fn get_property(&self, name: &str) -> ArcResult<Weak<JsHandle>> {
        let mut args = HashMap::new();
        args.insert("name", name);
        let v = send_message!(self, "getProperty", args);
        let guid = only_guid(&v)?;
        let j = find_object!(self.context()?.lock().unwrap(), &guid, JsHandle)?;
        Ok(j)
    }

    pub(crate) async fn get_properties(&self) -> ArcResult<HashMap<String, Weak<JsHandle>>> {
        let v = send_message!(self, "getPropertyList", Map::new());
        let GetPropertiesResponse { properties } =
            serde_json::from_value((*v).clone()).map_err(Error::Serde)?;
        let ps = properties
            .into_iter()
            .map(
                |Property {
                     name,
                     value: OnlyGuid { guid }
                 }| {
                    find_object!(self.context()?.lock().unwrap(), &guid, JsHandle)
                        .map(|o| (name, o))
                }
            )
            .collect::<Result<HashMap<_, _>, Error>>()?;
        Ok(ps)
    }
}

impl RemoteObject for JsHandle {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    preview: String
}

#[derive(Deserialize)]
struct GetPropertiesResponse {
    properties: Vec<Property>
}

#[derive(Deserialize)]
struct Property {
    name: String,
    value: OnlyGuid
}
