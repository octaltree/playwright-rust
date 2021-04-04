use crate::imp::{core::*, js_handle::JsHandle as Impl, prelude::*};

pub struct JsHandle {
    inner: Weak<Impl>
}

impl PartialEq for JsHandle {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

impl JsHandle {
    pub(crate) fn new(inner: Weak<Impl>) -> Self { Self { inner } }

    pub(crate) fn guid(&self) -> Result<Str<Guid>, Error> {
        Ok(upgrade(&self.inner)?.guid().to_owned())
    }

    pub async fn get_property(&mut self, name: &str) -> ArcResult<JsHandle> {
        upgrade(&self.inner)?
            .get_property(name)
            .await
            .map(JsHandle::new)
    }

    pub async fn get_properties(&mut self) -> ArcResult<HashMap<String, JsHandle>> {
        let m = upgrade(&self.inner)?.get_properties().await?;
        Ok(m.into_iter().map(|(k, v)| (k, JsHandle::new(v))).collect())
    }

    pub async fn dispose(&mut self) -> ArcResult<()> { upgrade(&self.inner)?.dispose().await }

    pub async fn json_value<U>(&mut self) -> ArcResult<U>
    where
        U: DeserializeOwned
    {
        upgrade(&self.inner)?.json_value().await
    }
}

mod ser {
    use super::*;
    use serde::{ser, ser::SerializeStruct};

    impl Serialize for JsHandle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer
        {
            let mut s = serializer.serialize_struct("4a9c3811-6f00-49e5-8a81-939f932d9061", 1)?;
            let guid = &self.guid().map_err(<S::Error as ser::Error>::custom)?;
            s.serialize_field("guid", &guid)?;
            s.end()
        }
    }
}
